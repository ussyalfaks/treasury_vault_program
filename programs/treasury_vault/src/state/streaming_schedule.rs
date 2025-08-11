use anchor_lang::prelude::*;

#[account]
pub struct StreamingSchedule {
    pub treasury: Pubkey,
    pub recipient: Pubkey,
    pub stream_id: u64,
    pub total_amount: u64,
    pub amount_per_second: u64,
    pub start_time: i64,
    pub cliff_time: i64,        // Time when cliff vesting ends
    pub end_time: i64,          // When stream ends
    pub token_mint: Option<Pubkey>, // None for SOL
    pub withdrawn_amount: u64,   // Amount already withdrawn
    pub is_active: bool,
    pub is_cancelable: bool,     // Can be cancelled by admin
    pub created_by: Pubkey,      // Who created the stream
    pub bump: u8,
}

impl StreamingSchedule {
    /// Calculate how much can be withdrawn at current time
    pub fn calculate_withdrawable_amount(&self, current_time: i64) -> u64 {
        // Stream hasn't started yet
        if current_time < self.start_time {
            return 0;
        }

        // Before cliff time, nothing can be withdrawn
        if current_time < self.cliff_time {
            return 0;
        }

        // After stream ends, full amount is withdrawable
        if current_time >= self.end_time {
            return self.total_amount.saturating_sub(self.withdrawn_amount);
        }

        // Calculate streamed amount based on time elapsed since cliff
        let time_since_cliff = (current_time - self.cliff_time) as u64;
        let streamed_amount = time_since_cliff
            .saturating_mul(self.amount_per_second)
            .min(self.total_amount);

        // Return withdrawable amount (streamed - already withdrawn)
        streamed_amount.saturating_sub(self.withdrawn_amount)
    }

    /// Check if stream is still active and not fully withdrawn
    pub fn is_stream_active(&self, current_time: i64) -> bool {
        self.is_active && 
        current_time >= self.start_time && 
        self.withdrawn_amount < self.total_amount
    }
}
