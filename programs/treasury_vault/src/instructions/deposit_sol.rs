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
pub struct DepositSol<'info> {
    #[account(
        mut,
        seeds = [
            b"treasury",
            treasury_seed_name.as_bytes().as_ref(),
        ],
        bump,
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    #[account(
        mut,
    )]
    pub depositor: Signer<'info>,
}

/// Accounts:
/// 0. `[writable]` treasury: [TreasuryConfig] 
/// 1. `[writable, signer]` depositor: [AccountInfo] 
///
/// Data:
/// - amount: [u64] Amount of SOL to deposit (in lamports)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<DepositSol>,
    amount: u64,
) -> Result<()> {
    // Transfer SOL from depositor to treasury
    let depositor_info = ctx.accounts.depositor.to_account_info();
    let treasury_info = ctx.accounts.treasury.to_account_info();
    
    // Check if depositor has enough SOL
    if **depositor_info.lamports.borrow() < amount {
        return Err(error::ErrorCode::InsufficientFunds.into());
    }
    
    // Transfer SOL
    **depositor_info.try_borrow_mut_lamports()? -= amount;
    **treasury_info.try_borrow_mut_lamports()? += amount;
    
    // Update treasury balance
    let treasury = &mut ctx.accounts.treasury;
    treasury.total_balance = treasury.total_balance.checked_add(amount).unwrap();
    
    Ok(())
}