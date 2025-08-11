
use crate::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    stream_id: u64,
    treasury_seed_name: String,
)]
pub struct WithdrawFromStream<'info> {
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
            recipient_signer.key().as_ref(),
        ],
        bump
    )]
    pub recipient: Account<'info, Recipient>,

    #[account(
        mut,
        seeds = [
            b"stream",
            treasury.key().as_ref(),
            recipient.key().as_ref(),
            stream_id.to_le_bytes().as_ref(),
        ],
        bump
    )]
    pub streaming_schedule: Account<'info, StreamingSchedule>,

    #[account(mut)]
    pub recipient_signer: Signer<'info>,
}

pub fn handler(
    ctx: Context<WithdrawFromStream>,
    _stream_id: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let streaming_schedule = &mut ctx.accounts.streaming_schedule;

    // Verify stream is active
    if !streaming_schedule.is_stream_active(current_time) {
        return Err(crate::error::ErrorCode::InvalidSchedule.into());
    }

    // Calculate withdrawable amount
    let withdrawable = streaming_schedule.calculate_withdrawable_amount(current_time);
    if withdrawable == 0 {
        return Err(crate::error::ErrorCode::PayoutNotDue.into());
    }

    // Do spending limit checks first (before taking mutable references)
    {
        let treasury = &ctx.accounts.treasury;
        let mut daily_total = treasury.daily_total;
        let mut weekly_total = treasury.weekly_total;
        let mut monthly_total = treasury.monthly_total;
        
        // Reset totals if time periods have passed
        if current_time - treasury.last_day_reset >= 86400 {
            daily_total = 0;
        }
        
        if current_time - treasury.last_week_reset >= 604800 {
            weekly_total = 0;
        }
        
        if current_time - treasury.last_month_reset >= 2592000 {
            monthly_total = 0;
        }
        
        // Check limits
        if daily_total.checked_add(withdrawable).unwrap() > treasury.daily_limit {
            return Err(crate::error::ErrorCode::SpendingLimitExceeded.into());
        }
        
        if weekly_total.checked_add(withdrawable).unwrap() > treasury.weekly_limit {
            return Err(crate::error::ErrorCode::SpendingLimitExceeded.into());
        }
        
        if monthly_total.checked_add(withdrawable).unwrap() > treasury.monthly_limit {
            return Err(crate::error::ErrorCode::SpendingLimitExceeded.into());
        }
    }

    // Transfer funds based on token type (using immutable borrow)
    match streaming_schedule.token_mint {
        None => {
            // SOL transfer
            let treasury_info = ctx.accounts.treasury.to_account_info();
            let recipient_info = ctx.accounts.recipient_signer.to_account_info();

            if **treasury_info.lamports.borrow() < withdrawable {
                return Err(crate::error::ErrorCode::InsufficientFunds.into());
            }

            **treasury_info.try_borrow_mut_lamports()? -= withdrawable;
            **recipient_info.try_borrow_mut_lamports()? += withdrawable;
        },
        Some(_) => {
            // Token transfer - would need token accounts in context
            // Implementation similar to execute_token_payout
            return Err(crate::error::ErrorCode::InvalidTokenVault.into());
        }
    }

    // Now update all state with mutable borrows
    {
        let treasury = &mut ctx.accounts.treasury;
        
        // Reset and update time periods
        if current_time - treasury.last_day_reset >= 86400 {
            treasury.daily_total = 0;
            treasury.last_day_reset = current_time;
        }
        
        if current_time - treasury.last_week_reset >= 604800 {
            treasury.weekly_total = 0;
            treasury.last_week_reset = current_time;
        }
        
        if current_time - treasury.last_month_reset >= 2592000 {
            treasury.monthly_total = 0;
            treasury.last_month_reset = current_time;
        }
        
        // Update totals
        treasury.daily_total = treasury.daily_total.checked_add(withdrawable).unwrap();
        treasury.weekly_total = treasury.weekly_total.checked_add(withdrawable).unwrap();
        treasury.monthly_total = treasury.monthly_total.checked_add(withdrawable).unwrap();
        
        // Update treasury balance
        treasury.total_balance = treasury.total_balance.saturating_sub(withdrawable);
    }

    // Update stream state
    streaming_schedule.withdrawn_amount = streaming_schedule.withdrawn_amount
        .checked_add(withdrawable).unwrap();

    // Check if stream is fully withdrawn
    if streaming_schedule.withdrawn_amount >= streaming_schedule.total_amount {
        streaming_schedule.is_active = false;
    }

    // Update recipient stats
    let recipient = &mut ctx.accounts.recipient;
    recipient.total_received = recipient.total_received.checked_add(withdrawable).unwrap();
    recipient.last_payout_time = current_time;

    msg!(
        "Stream withdrawal: {} tokens withdrawn, {} total withdrawn",
        withdrawable,
        streaming_schedule.withdrawn_amount
    );

    Ok(())
}
