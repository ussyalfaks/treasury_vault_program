use anchor_lang::prelude::*;

#[account]
pub struct TreasuryConfig {
    pub admin: Pubkey,
    pub treasurer: Pubkey,
    pub name: String,
    pub description: String,
    pub daily_limit: u64,
    pub weekly_limit: u64,
    pub monthly_limit: u64,
    pub total_balance: u64,
    pub daily_total: u64,
    pub weekly_total: u64,
    pub monthly_total: u64,
    pub last_day_reset: i64,
    pub last_week_reset: i64,
    pub last_month_reset: i64,
    pub require_token_gate: bool,
    pub token_gate_mint: Option<Pubkey>,
    pub token_gate_amount: u64,
    pub bump: u8,
}