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
    treasury_seed_name: String,
)]
pub struct ExecuteTokenPayout<'info> {
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

impl<'info> ExecuteTokenPayout<'info> {
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
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` treasury: [TreasuryConfig] 
/// 2. `[writable]` recipient: [Recipient] 
/// 3. `[writable]` payout_schedule: [PayoutSchedule] 
/// 4. `[writable]` token_vault: [TokenVault] 
/// 5. `[]` token_mint: [Mint] 
/// 6. `[writable]` source: [AccountInfo] The source account.
/// 7. `[writable]` destination: [AccountInfo] The destination account.
/// 8. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 9. `[]` csl_spl_token_v0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - schedule_id: [u64] Unique identifier for this schedule
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<ExecuteTokenPayout>,
    recipient_address: Pubkey,
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
    
    // Verify token mint matches the schedule
    if ctx.accounts.payout_schedule.token_mint.is_none() || 
       ctx.accounts.payout_schedule.token_mint.unwrap() != ctx.accounts.token_mint.key() {
        return Err(error::ErrorCode::InvalidTokenVault.into());
    }
    
    // Check if token vault has enough funds
    let amount = ctx.accounts.payout_schedule.amount;
    if ctx.accounts.token_vault.balance < amount {
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
    
    // Transfer tokens
    ctx.accounts.cpi_csl_spl_token_transfer(amount)?;
    
    // Update token vault balance
    let token_vault = &mut ctx.accounts.token_vault;
    token_vault.balance = token_vault.balance.checked_sub(amount).unwrap();
    
    // Update treasury totals
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