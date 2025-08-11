use anchor_lang::prelude::*;

#[account]
pub struct PayoutSchedule {
    pub treasury: Pubkey,
    pub recipient: Pubkey,
    pub schedule_id: u64,
    pub amount: u64,
    pub token_mint: Option<Pubkey>,
    pub start_time: i64,
    pub interval_seconds: u64,
    pub executions: u64,
    pub max_executions: u64,
    pub is_active: bool,
    pub last_execution_time: i64,
    pub bump: u8,
}