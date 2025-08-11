
use anchor_lang::prelude::*;

#[account]
pub struct TokenVault {
	pub treasury: Pubkey,
	pub token_mint: Pubkey,
	pub token_account: Pubkey,
	pub balance: u64,
	pub bump: u8,
}
