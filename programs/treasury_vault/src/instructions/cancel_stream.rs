
use crate::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(
    recipient_address: Pubkey,
    stream_id: u64,
    treasury_seed_name: String,
)]
pub struct CancelStream<'info> {
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
            b"stream",
            treasury.key().as_ref(),
            recipient.key().as_ref(),
            stream_id.to_le_bytes().as_ref(),
        ],
        bump,
        close = authority // Close account and return rent to authority
    )]
    pub streaming_schedule: Account<'info, StreamingSchedule>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    /// CHECK: Recipient address for final withdrawal
    pub recipient_wallet: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<CancelStream>,
    _recipient_address: Pubkey,
    _stream_id: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let streaming_schedule = &ctx.accounts.streaming_schedule;

    // Verify authority can cancel (admin, treasurer, or stream creator)
    let treasury = &ctx.accounts.treasury;
    let is_authorized = ctx.accounts.authority.key() == treasury.admin ||
                       ctx.accounts.authority.key() == treasury.treasurer ||
                       ctx.accounts.authority.key() == streaming_schedule.created_by;

    if !is_authorized {
        return Err(crate::error::ErrorCode::UnauthorizedAccess.into());
    }

    // Check if stream is cancelable
    if !streaming_schedule.is_cancelable {
        return Err(crate::error::ErrorCode::InvalidSchedule.into());
    }

    // Calculate final withdrawable amount for recipient
    let final_withdrawable = streaming_schedule.calculate_withdrawable_amount(current_time);
    
    if final_withdrawable > 0 {
        // Transfer final amount to recipient
        match streaming_schedule.token_mint {
            None => {
                // SOL transfer
                let treasury_info = ctx.accounts.treasury.to_account_info();
                let recipient_info = ctx.accounts.recipient_wallet.to_account_info();

                **treasury_info.try_borrow_mut_lamports()? -= final_withdrawable;
                **recipient_info.try_borrow_mut_lamports()? += final_withdrawable;

                let treasury_mut = &mut ctx.accounts.treasury;
                treasury_mut.total_balance = treasury_mut.total_balance
                    .saturating_sub(final_withdrawable);
            },
            Some(_) => {
                // Token transfer implementation needed
                return Err(crate::error::ErrorCode::InvalidTokenVault.into());
            }
        }
    }

    msg!(
        "Stream cancelled: {} tokens transferred to recipient, stream closed",
        final_withdrawable
    );

    // Account automatically closed due to close constraint
    Ok(())
}
