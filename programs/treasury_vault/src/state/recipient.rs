
use anchor_lang::prelude::*;

#[account]
pub struct Recipient {
	pub treasury: Pubkey,
	pub recipient_address: Pubkey,
	pub name: String,
	pub role: u8,
	pub is_active: bool,
	pub total_received: u64,
	pub last_payout_time: i64,
	pub bump: u8,
}
