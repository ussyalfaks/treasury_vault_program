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
pub struct EmergencyWithdrawToken<'info> {
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
            b"token_vault",
            treasury.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump
    )]
    pub token_vault: Account<'info, TokenVault>,

    pub token_mint: Account<'info, Mint>,

    pub admin: Signer<'info>,

    #[account(
        mut,
    )]
    /// CHECK: implement manual checks if needed
    pub source: UncheckedAccount<'info>,

    #[account(
        mut,
    )]
    /// CHECK: implement manual checks if needed
    pub destination: UncheckedAccount<'info>,

    #[account(
        owner=Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
    )]
    pub authority: Signer<'info>,

    pub csl_spl_token_v0_0_0: Program<'info, Token>,
}

impl<'info> EmergencyWithdrawToken<'info> {
    pub fn cpi_csl_spl_token_transfer(&self, amount: u64) -> Result<()> {
        anchor_spl::token::transfer(
            CpiContext::new(self.csl_spl_token_v0_0_0.to_account_info(), 
                anchor_spl::token::Transfer {
                    from: self.source.to_account_info(),
                    to: self.destination.to_account_info(),
                    authority: self.authority.to_account_info()
                }
            ),
            amount, 
        )
    }
}


/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` token_vault: [TokenVault] 
/// 2. `[]` token_mint: [Mint] 
/// 3. `[signer]` admin: [AccountInfo] 
/// 4. `[writable]` source: [AccountInfo] The source account.
/// 5. `[writable]` destination: [AccountInfo] The destination account.
/// 6. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 7. `[]` csl_spl_token_v0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - amount: [u64] Amount of tokens to withdraw
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<EmergencyWithdrawToken>,
    amount: u64,
) -> Result<()> {
    // Verify the signer is the admin
    if ctx.accounts.admin.key() != ctx.accounts.treasury.admin {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Check if token vault has enough funds
    if ctx.accounts.token_vault.balance < amount {
        return Err(crate::error::ErrorCode::InsufficientFunds.into());
    }
    
    // Transfer tokens
    ctx.accounts.cpi_csl_spl_token_transfer(amount)?;
    
    // Update token vault balance
    let token_vault = &mut ctx.accounts.token_vault;
    token_vault.balance = token_vault.balance.checked_sub(amount).unwrap();
    
    Ok(())
}