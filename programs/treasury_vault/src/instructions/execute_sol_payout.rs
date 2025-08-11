use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
    schedule_id: u64,
    treasury_seed_name: String,
)]
pub struct ExecuteSolPayout<'info> {
    #[account(
        mut,
        owner=Pubkey::from_str("11111111111111111111111111111111").unwrap(),
    )]
    pub fee_payer: Signer<'info>,

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
        seeds = [
            b"recipient",
            treasury.key().as_ref(),
            recipient_address.key().as_ref(),
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

    #[account(
        mut,
    )]
    /// CHECK: implement manual checks if needed
    pub recipient_address: UncheckedAccount<'info>,
}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` treasury: [TreasuryConfig] 
/// 2. `[writable]` recipient: [Recipient] 
/// 3. `[writable]` payout_schedule: [PayoutSchedule] 
/// 4. `[writable]` recipient_address: [AccountInfo] 
///
/// Data:
/// - schedule_id: [u64] Unique identifier for this schedule
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<ExecuteSolPayout>,
    schedule_id: u64,
) -> Result<()> {
    // Get current time
    let current_time = Clock::get()?.unix_timestamp;
    
    // Verify payout schedule is active
    if !ctx.accounts.payout_schedule.is_active {
        return Err(error::ErrorCode::InvalidSchedule.into());
    }
    
    // Verify payout is due
    if current_time < ctx.accounts.payout_schedule.start_time {
        return Err(error::ErrorCode::PayoutNotDue.into());
    }
    
    // Check if max executions reached
    if ctx.accounts.payout_schedule.max_executions > 0 && 
       ctx.accounts.payout_schedule.executions >= ctx.accounts.payout_schedule.max_executions {
        return Err(error::ErrorCode::MaxExecutionsReached.into());
    }
    
    // For recurring payments, check if next payment is due
    if ctx.accounts.payout_schedule.interval_seconds > 0 && 
       ctx.accounts.payout_schedule.last_execution_time > 0 {
        let next_execution = ctx.accounts.payout_schedule.last_execution_time + 
                            ctx.accounts.payout_schedule.interval_seconds as i64;
        if current_time < next_execution {
            return Err(error::ErrorCode::PayoutNotDue.into());
        }
    }
    
    // Check if treasury has enough funds
    let amount = ctx.accounts.payout_schedule.amount;
    let treasury_info = ctx.accounts.treasury.to_account_info();
    if **treasury_info.lamports.borrow() < amount {
        return Err(error::ErrorCode::InsufficientFunds.into());
    }
    
    // Check spending limits
    let treasury = &mut ctx.accounts.treasury;
    
    // Reset daily limit if needed
    if current_time - treasury.last_day_reset >= 86400 { // 24 hours in seconds
        treasury.daily_total = 0;
        treasury.last_day_reset = current_time;
    }
    
    // Reset weekly limit if needed
    if current_time - treasury.last_week_reset >= 604800 { // 7 days in seconds
        treasury.weekly_total = 0;
        treasury.last_week_reset = current_time;
    }
    
    // Reset monthly limit if needed
    if current_time - treasury.last_month_reset >= 2592000 { // 30 days in seconds
        treasury.monthly_total = 0;
        treasury.last_month_reset = current_time;
    }
    
    // Check if this payment would exceed limits
    if treasury.daily_total.checked_add(amount).unwrap() > treasury.daily_limit {
        return Err(error::ErrorCode::SpendingLimitExceeded.into());
    }
    
    if treasury.weekly_total.checked_add(amount).unwrap() > treasury.weekly_limit {
        return Err(error::ErrorCode::SpendingLimitExceeded.into());
    }
    
    if treasury.monthly_total.checked_add(amount).unwrap() > treasury.monthly_limit {
        return Err(error::ErrorCode::SpendingLimitExceeded.into());
    }
    
    // Transfer SOL
    **treasury_info.try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.recipient_address.to_account_info().try_borrow_mut_lamports()? += amount;
    
    // Update treasury totals
    treasury.total_balance = treasury.total_balance.checked_sub(amount).unwrap();
    treasury.daily_total = treasury.daily_total.checked_add(amount).unwrap();
    treasury.weekly_total = treasury.weekly_total.checked_add(amount).unwrap();
    treasury.monthly_total = treasury.monthly_total.checked_add(amount).unwrap();
    
    // Update recipient
    let recipient = &mut ctx.accounts.recipient;
    recipient.total_received = recipient.total_received.checked_add(amount).unwrap();
    recipient.last_payout_time = current_time;
    
    // Update payout schedule
    let payout_schedule = &mut ctx.accounts.payout_schedule;
    payout_schedule.executions = payout_schedule.executions.checked_add(1).unwrap();
    payout_schedule.last_execution_time = current_time;
    
    // If this was the last execution and max_executions is set, mark as inactive
    if payout_schedule.max_executions > 0 && 
       payout_schedule.executions >= payout_schedule.max_executions {
        payout_schedule.is_active = false;
    }
    
    Ok(())
}