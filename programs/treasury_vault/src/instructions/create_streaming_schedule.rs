
use crate::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    recipient_address: Pubkey,
    stream_id: u64,
    total_amount: u64,
    amount_per_second: u64,
    start_time: i64,
    cliff_time: i64,
    duration_seconds: u64,
    token_mint: Option<Pubkey>,
    treasury_seed_name: String,
)]
pub struct CreateStreamingSchedule<'info> {
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
        space = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 8 + 33 + 8 + 1 + 1 + 32 + 1, // ~200 bytes
        payer = authority,
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
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateStreamingSchedule>,
    recipient_address: Pubkey,
    stream_id: u64,
    total_amount: u64,
    amount_per_second: u64,
    start_time: i64,
    cliff_time: i64,
    duration_seconds: u64,
    token_mint: Option<Pubkey>,
) -> Result<()> {
    // Verify authority is admin or treasurer
    let treasury = &ctx.accounts.treasury;
    if ctx.accounts.authority.key() != treasury.admin && 
       ctx.accounts.authority.key() != treasury.treasurer {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }

    // Verify recipient is active
    if !ctx.accounts.recipient.is_active {
        return Err(crate::error::ErrorCode::InactiveRecipient.into());
    }

    // Validate streaming parameters
    if total_amount == 0 || amount_per_second == 0 || duration_seconds == 0 {
        return Err(crate::error::ErrorCode::InvalidSchedule.into());
    }

    if cliff_time < start_time {
        return Err(crate::error::ErrorCode::InvalidSchedule.into());
    }

    let end_time = start_time + duration_seconds as i64;
    if end_time <= cliff_time {
        return Err(crate::error::ErrorCode::InvalidSchedule.into());
    }

    // Validate that total_amount matches amount_per_second * duration
    let expected_total = amount_per_second.saturating_mul(duration_seconds);
    if total_amount > expected_total {
        return Err(crate::error::ErrorCode::InvalidSchedule.into());
    }

    // Check if treasury has sufficient funds
    match token_mint {
        None => {
            // SOL stream - check treasury balance
            let treasury_info = ctx.accounts.treasury.to_account_info();
            if **treasury_info.lamports.borrow() < total_amount {
                return Err(crate::error::ErrorCode::InsufficientFunds.into());
            }
        },
        Some(_) => {
            // Token stream - would need to check token vault balance
            // This requires additional accounts in the context
        }
    }

    // Initialize the streaming schedule
    let streaming_schedule = &mut ctx.accounts.streaming_schedule;
    streaming_schedule.treasury = treasury.key();
    streaming_schedule.recipient = ctx.accounts.recipient.key();
    streaming_schedule.stream_id = stream_id;
    streaming_schedule.total_amount = total_amount;
    streaming_schedule.amount_per_second = amount_per_second;
    streaming_schedule.start_time = start_time;
    streaming_schedule.cliff_time = cliff_time;
    streaming_schedule.end_time = end_time;
    streaming_schedule.token_mint = token_mint;
    streaming_schedule.withdrawn_amount = 0;
    streaming_schedule.is_active = true;
    streaming_schedule.is_cancelable = true;
    streaming_schedule.created_by = ctx.accounts.authority.key();
    streaming_schedule.bump = ctx.bumps.streaming_schedule;

    msg!(
        "Streaming schedule created: {} tokens over {} seconds to {}",
        total_amount,
        duration_seconds,
        recipient_address
    );

    Ok(())
}