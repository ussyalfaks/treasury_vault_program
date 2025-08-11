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
}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` recipient: [Recipient] 
/// 2. `[signer]` authority: [AccountInfo] Must be admin or treasurer
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - name: [String] Name of the recipient
/// - role: [u8] Role of the recipient (0=Regular, 1=Privileged)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<AddRecipient>,
    recipient_address: Pubkey,
    name: String,
    role: u8,
) -> Result<()> {
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
    
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && ctx.accounts.authority.key() != treasury.treasurer {
        return Err(error::ErrorCode::UnauthorizedAccess.into());
    }
    
    Ok(())
}