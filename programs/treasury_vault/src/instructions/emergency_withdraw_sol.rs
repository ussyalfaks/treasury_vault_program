use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
    amount: u64,
    treasury_seed_name: String,
)]
pub struct EmergencyWithdrawSol<'info> {
    #[account(
        mut,
        seeds = [
            b"treasury",
            treasury_seed_name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    #[account(
        mut,
    )]
    pub admin: Signer<'info>,
}

/// Accounts:
/// 0. `[writable]` treasury: [TreasuryConfig] 
/// 1. `[writable, signer]` admin: [AccountInfo] 
///
/// Data:
/// - amount: [u64] Amount of SOL to withdraw (in lamports)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<EmergencyWithdrawSol>,
    amount: u64,
) -> Result<()> {
    // Verify the signer is the admin
    if ctx.accounts.admin.key() != ctx.accounts.treasury.admin {
        return Err(error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Check if treasury has enough funds
    let treasury_info = ctx.accounts.treasury.to_account_info();
    if **treasury_info.lamports.borrow() < amount {
        return Err(error::ErrorCode::InsufficientFunds.into());
    }
    
    // Transfer SOL
    **treasury_info.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += amount;
    
    // Update treasury balance
    let treasury = &mut ctx.accounts.treasury;
    treasury.total_balance = treasury.total_balance.checked_sub(amount).unwrap();
    
    Ok(())
}