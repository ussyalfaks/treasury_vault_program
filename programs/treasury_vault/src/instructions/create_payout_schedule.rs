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
    schedule_id: u64,
    amount: u64,
    token_mint: Option<Pubkey>,
    start_time: i64,
    interval_seconds: u64,
    max_executions: u64,
    treasury_seed_name: String,
)]
pub struct CreatePayoutSchedule<'info> {
    #[account(
        seeds = [
            b"treasury",
            treasury_seed_name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    #[account(
        seeds = [
            b"recipient",
            treasury.key().as_ref(),
            recipient_address.as_ref(),
        ],
        bump
    )]
    pub recipient: Account<'info, Recipient>,

    #[account(
        init,
        space=171,
        payer=authority,
        seeds = [
            b"schedule",
            treasury.key().as_ref(),
            recipient.key().as_ref(),
            schedule_id.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub payout_schedule: Account<'info, PayoutSchedule>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[]` recipient: [Recipient] 
/// 2. `[writable]` payout_schedule: [PayoutSchedule] 
/// 3. `[signer]` authority: [AccountInfo] Must be admin or treasurer
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - schedule_id: [u64] Unique identifier for this schedule
/// - amount: [u64] Amount to be paid out
/// - token_mint: [Option<Pubkey>] Optional token mint address (null for SOL)
/// - start_time: [i64] When this schedule starts
/// - interval_seconds: [u64] Interval between payouts in seconds (0 for one-time)
/// - max_executions: [u64] Maximum number of executions (0 for unlimited)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<CreatePayoutSchedule>,
    recipient_address: Pubkey,
    schedule_id: u64,
    amount: u64,
    token_mint: Option<Pubkey>,
    start_time: i64,
    interval_seconds: u64,
    max_executions: u64,
) -> Result<()> {
    // Initialize the payout schedule
    let payout_schedule = &mut ctx.accounts.payout_schedule;
    payout_schedule.treasury = ctx.accounts.treasury.key();
    payout_schedule.recipient = ctx.accounts.recipient.key();
    payout_schedule.schedule_id = schedule_id;
    payout_schedule.amount = amount;
    payout_schedule.token_mint = token_mint;
    payout_schedule.start_time = start_time;
    payout_schedule.interval_seconds = interval_seconds;
    payout_schedule.max_executions = max_executions;
    payout_schedule.executions = 0;
    payout_schedule.last_execution_time = 0;
    payout_schedule.is_active = true;
    payout_schedule.bump = ctx.bumps.payout_schedule;
    
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && ctx.accounts.authority.key() != treasury.treasurer {
        return Err(error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Verify recipient is active
    if !ctx.accounts.recipient.is_active {
        return Err(error::ErrorCode::InactiveRecipient.into());
    }
    
    Ok(())
}