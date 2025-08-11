// Updated src/instructions/add_recipient.rs with token gating validation

use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
    recipient_address: Pubkey,
    name: String,
    role: u8,
    treasury_seed_name: String,
)]
pub struct AddRecipient<'info> {
    #[account(
        seeds = [
            b"treasury",
            treasury_seed_name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    #[account(
        init,
        space=127,
        payer=authority,
        seeds = [
            b"recipient",
            treasury.key().as_ref(),
            recipient_address.as_ref(),
        ],
        bump
    )]
    pub recipient: Account<'info, Recipient>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    // Optional accounts for token gating validation
    /// CHECK: Only required if treasury.require_token_gate is true
    pub token_gate_mint: Option<Account<'info, Mint>>,

    /// CHECK: Only required if treasury.require_token_gate is true  
    pub recipient_token_account: Option<Account<'info, TokenAccount>>,

    /// CHECK: Token program for validation
    pub token_program: Option<Program<'info, Token>>,
}

pub fn handler(
    ctx: Context<AddRecipient>,
    recipient_address: Pubkey,
    name: String,
    role: u8,
) -> Result<()> {
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && ctx.accounts.authority.key() != treasury.treasurer {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Validate token gating requirements if enabled
    if treasury.require_token_gate {
        // Ensure token gate mint is provided and matches treasury config
        let token_gate_mint = ctx.accounts.token_gate_mint
            .as_ref()
            .ok_or(crate::error::ErrorCode::MissingTokenGate)?;
        
        if Some(token_gate_mint.key()) != treasury.token_gate_mint {
            return Err(crate::error::ErrorCode::MissingTokenGate.into());
        }
        
        // Validate recipient's token account
        let recipient_token_account = ctx.accounts.recipient_token_account
            .as_ref()
            .ok_or(crate::error::ErrorCode::MissingTokenGate)?;
        
        // Verify token account belongs to recipient
        if recipient_token_account.owner != recipient_address {
            return Err(crate::error::ErrorCode::MissingTokenGate.into());
        }
        
        // Verify token account is for correct mint
        if recipient_token_account.mint != token_gate_mint.key() {
            return Err(crate::error::ErrorCode::MissingTokenGate.into());
        }
        
        // Verify recipient has enough tokens
        if recipient_token_account.amount < treasury.token_gate_amount {
            return Err(crate::error::ErrorCode::MissingTokenGate.into());
        }
        
        msg!(
            "Token gate validated: recipient {} has {} tokens (required: {})",
            recipient_address,
            recipient_token_account.amount,
            treasury.token_gate_amount
        );
    }
    
    // Initialize the recipient account
    let recipient = &mut ctx.accounts.recipient;
    recipient.treasury = ctx.accounts.treasury.key();
    recipient.recipient_address = recipient_address;
    recipient.name = name;
    recipient.role = role;
    recipient.is_active = true;
    recipient.total_received = 0;
    recipient.last_payout_time = 0;
    recipient.bump = ctx.bumps.recipient;
    
    msg!(
        "Recipient added: {} with role {} (token gating: {})",
        recipient_address,
        role,
        treasury.require_token_gate
    );
    
    Ok(())
}

// Helper function to check if recipient still meets token gating requirements
pub fn verify_recipient_token_gate_status(
    treasury: &Account<TreasuryConfig>,
    recipient_token_account: Option<&Account<TokenAccount>>,
    recipient_address: &Pubkey,
) -> bool {
    if !treasury.require_token_gate {
        return true;
    }
    
    match (treasury.token_gate_mint, recipient_token_account) {
        (Some(gate_mint), Some(token_account)) => {
            token_account.owner == *recipient_address &&
            token_account.mint == gate_mint &&
            token_account.amount >= treasury.token_gate_amount
        },
        _ => false,
    }
}