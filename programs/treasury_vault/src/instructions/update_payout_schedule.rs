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
    amount: Option<u64>,
    start_time: Option<i64>,
    interval_seconds: Option<u64>,
    max_executions: Option<u64>,
    is_active: Option<bool>,
    treasury_seed_name: String,
)]
pub struct UpdatePayoutSchedule<'info> {
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
        mut,
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
}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[]` recipient: [Recipient] 
/// 2. `[writable]` payout_schedule: [PayoutSchedule] 
/// 3. `[signer]` authority: [AccountInfo] Must be admin or treasurer
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - schedule_id: [u64] Unique identifier for this schedule
/// - amount: [Option<u64>] Optional new amount to be paid out
/// - start_time: [Option<i64>] Optional new start time
/// - interval_seconds: [Option<u64>] Optional new interval between payouts
/// - max_executions: [Option<u64>] Optional new maximum number of executions
/// - is_active: [Option<bool>] Optional update to active status
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<UpdatePayoutSchedule>,
    _recipient_address: Pubkey,
    _schedule_id: u64,
    amount: Option<u64>,
    start_time: Option<i64>,
    interval_seconds: Option<u64>,
    max_executions: Option<u64>,
    is_active: Option<bool>,
) -> Result<()> {
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && ctx.accounts.authority.key() != treasury.treasurer {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Update payout schedule with new values if provided
    let payout_schedule = &mut ctx.accounts.payout_schedule;
    
    if let Some(new_amount) = amount {
        payout_schedule.amount = new_amount;
    }
    
    if let Some(new_start_time) = start_time {
        payout_schedule.start_time = new_start_time;
    }
    
    if let Some(new_interval) = interval_seconds {
        payout_schedule.interval_seconds = new_interval;
    }
    
    if let Some(new_max) = max_executions {
        payout_schedule.max_executions = new_max;
    }
    
    if let Some(active) = is_active {
        payout_schedule.is_active = active;
    }
    
    Ok(())
}