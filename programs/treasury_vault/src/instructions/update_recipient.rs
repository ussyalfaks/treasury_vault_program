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
    name: Option<String>,
    role: Option<u8>,
    is_active: Option<bool>,
    treasury_seed_name: String,
)]
pub struct UpdateRecipient<'info> {
    #[account(
        seeds = [
            b"treasury",
            treasury_seed_name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    #[account(
        mut,
        seeds = [
            b"recipient",
            treasury.key().as_ref(),
            recipient_address.as_ref(),
        ],
        bump
    )]
    pub recipient: Account<'info, Recipient>,

    pub authority: Signer<'info>,
}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` recipient: [Recipient] 
/// 2. `[signer]` authority: [AccountInfo] Must be admin or treasurer
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - name: [Option<String>] Optional new name for the recipient
/// - role: [Option<u8>] Optional new role for the recipient
/// - is_active: [Option<bool>] Optional update to active status
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<UpdateRecipient>,
    _recipient_address: Pubkey,
    name: Option<String>,
    role: Option<u8>,
    is_active: Option<bool>,
) -> Result<()> {
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && ctx.accounts.authority.key() != treasury.treasurer {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Update recipient with new values if provided
    let recipient = &mut ctx.accounts.recipient;
    
    if let Some(new_name) = name {
        recipient.name = new_name;
    }
    
    if let Some(new_role) = role {
        recipient.role = new_role;
    }
    
    if let Some(active) = is_active {
        recipient.is_active = active;
    }
    
    Ok(())
}