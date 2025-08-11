use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
    treasury_seed_name: String,
)]
pub struct InitializeTokenVault<'info> {
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
        space=113,
        payer=authority,
        seeds = [
            b"token_vault",
            treasury.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump
    )]
    pub token_vault: Account<'info, TokenVault>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        mut,
        owner=Pubkey::from_str("11111111111111111111111111111111").unwrap(),
    )]
    pub funding: Signer<'info>,

    #[account(
        init,
        payer = funding,
        associated_token::mint = mint,
        associated_token::authority = wallet,
        // Remove the associated_token::token_program line that's causing the error
    )]
    pub assoc_token_account: Account<'info, TokenAccount>,

    /// CHECK: implement manual checks if needed
    pub wallet: UncheckedAccount<'info>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub csl_spl_assoc_token_v0_0_0: Program<'info, AssociatedToken>,
}
/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` token_vault: [TokenVault] 
/// 2. `[]` token_mint: [Mint] 
/// 3. `[signer]` authority: [AccountInfo] Must be admin or treasurer
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 5. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 6. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 7. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 8. `[]` mint: [Mint] The token mint for the new associated token account
/// 9. `[]` token_program: [AccountInfo] SPL Token program
/// 10. `[]` csl_spl_assoc_token_v0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<InitializeTokenVault>,
) -> Result<()> {
    // Initialize the token vault account
    let token_vault = &mut ctx.accounts.token_vault;
    token_vault.treasury = ctx.accounts.treasury.key();
    token_vault.token_mint = ctx.accounts.token_mint.key();
    token_vault.token_account = ctx.accounts.assoc_token_account.key();
    token_vault.balance = 0;
    token_vault.bump = ctx.bumps.token_vault;
    
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && ctx.accounts.authority.key() != treasury.treasurer {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }
    
    Ok(())
}