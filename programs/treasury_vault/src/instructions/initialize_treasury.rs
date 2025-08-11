use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
    name: String,
    description: String,
    treasurer: Pubkey,
    daily_limit: u64,
    weekly_limit: u64,
    monthly_limit: u64,
    require_token_gate: bool,
    token_gate_mint: Option<Pubkey>,
    token_gate_amount: u64,
)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        space=411,
        payer=admin,
        seeds = [
            b"treasury",
            name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    pub system_program: Program<'info, System>,
}

/// Accounts:
/// 0. `[signer]` admin: [AccountInfo] 
/// 1. `[writable]` treasury: [TreasuryConfig] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - name: [String] Name of the treasury
/// - description: [String] type
/// - treasurer: [Pubkey] The treasurer authority that can approve payouts
/// - daily_limit: [u64] Maximum amount that can be spent in a day
/// - weekly_limit: [u64] Maximum amount that can be spent in a week
/// - monthly_limit: [u64] Maximum amount that can be spent in a month
/// - require_token_gate: [bool] Whether token gating is required for recipients
/// - token_gate_mint: [Option<Pubkey>] Optional mint address for token gating
/// - token_gate_amount: [u64] Minimum amount of tokens required for token gating
pub fn handler(
    ctx: Context<InitializeTreasury>, // Remove the & here
    name: String,
    description: String,
    treasurer: Pubkey,
    daily_limit: u64,
    weekly_limit: u64,
    monthly_limit: u64,
    require_token_gate: bool,
    token_gate_mint: Option<Pubkey>,
    token_gate_amount: u64,
) -> Result<()> {
    // Initialize the treasury account
    let treasury = &mut ctx.accounts.treasury;
    treasury.admin = ctx.accounts.admin.key();
    treasury.treasurer = treasurer;
    treasury.name = name;
    treasury.description = description;
    treasury.daily_limit = daily_limit;
    treasury.weekly_limit = weekly_limit;
    treasury.monthly_limit = monthly_limit;
    treasury.require_token_gate = require_token_gate;
    treasury.token_gate_mint = token_gate_mint;
    treasury.token_gate_amount = token_gate_amount;
    treasury.total_balance = 0;
    treasury.daily_total = 0;
    treasury.weekly_total = 0;
    treasury.monthly_total = 0;
    treasury.last_day_reset = Clock::get()?.unix_timestamp;
    treasury.last_week_reset = Clock::get()?.unix_timestamp;
    treasury.last_month_reset = Clock::get()?.unix_timestamp;
    treasury.bump = ctx.bumps.treasury;
    
    Ok(())
}