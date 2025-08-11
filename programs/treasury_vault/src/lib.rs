
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use std::str::FromStr;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FZF2W7peTaeeAYkL5sz81drHMNW5qQemerM1Cx8FViHC");

#[program]
pub mod treasury_vault {
    use super::*;

/// Accounts:
/// 0. `[signer]` admin: [AccountInfo] 
/// 1. `[writable]` treasury: [TreasuryConfig] 
/// 2. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - name: [String] Name of the treasury
/// - description: [String] type
/// - treasurer: [Pubkey] The treasurer authority that can approve payouts
/// - daily_limit: [u64] Maximum amount that can be spent in a day
/// - weekly_limit: [u64] Maximum amount that can be spent in a week
/// - monthly_limit: [u64] Maximum amount that can be spent in a month
/// - require_token_gate: [bool] Whether token gating is required for recipients
/// - token_gate_mint: [Option<Pubkey>] Optional mint address for token gating
/// - token_gate_amount: [u64] Minimum amount of tokens required for token gating
	pub fn initialize_treasury(ctx: Context<InitializeTreasury>, name: String, description: String, treasurer: Pubkey, daily_limit: u64, weekly_limit: u64, monthly_limit: u64, require_token_gate: bool, token_gate_mint: Option<Pubkey>, token_gate_amount: u64) -> Result<()> {
		initialize_treasury::handler(ctx, name, description, treasurer, daily_limit, weekly_limit, monthly_limit, require_token_gate, token_gate_mint, token_gate_amount)
	}

/// Accounts:
/// 0. `[writable]` treasury: [TreasuryConfig] 
/// 1. `[signer]` admin: [AccountInfo] 
///
/// Data:
/// - new_admin: [Option<Pubkey>] Optional new admin authority
/// - new_treasurer: [Option<Pubkey>] Optional new treasurer authority
/// - description: [Option<String>] type
/// - daily_limit: [Option<u64>] Optional new daily limit
/// - weekly_limit: [Option<u64>] Optional new weekly limit
/// - monthly_limit: [Option<u64>] Optional new monthly limit
/// - require_token_gate: [Option<bool>] Optional update to token gating requirement
/// - token_gate_mint: [Option<Pubkey>] Optional new token gate mint
/// - token_gate_amount: [Option<u64>] Optional new token gate amount
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn update_treasury_config(ctx: Context<UpdateTreasuryConfig>, new_admin: Option<Pubkey>, new_treasurer: Option<Pubkey>, description: Option<String>, daily_limit: Option<u64>, weekly_limit: Option<u64>, monthly_limit: Option<u64>, require_token_gate: Option<bool>, token_gate_mint: Option<Pubkey>, token_gate_amount: Option<u64>, _treasury_seed_name: String) -> Result<()> {
		update_treasury_config::handler(ctx, new_admin, new_treasurer, description, daily_limit, weekly_limit, monthly_limit, require_token_gate, token_gate_mint, token_gate_amount)
	}

/// Accounts:
/// 0. `[writable]` treasury: [TreasuryConfig] 
/// 1. `[writable, signer]` depositor: [AccountInfo] 
///
/// Data:
/// - amount: [u64] Amount of SOL to deposit (in lamports)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64, _treasury_seed_name: String) -> Result<()> {
		deposit_sol::handler(ctx, amount)
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` token_vault: [TokenVault] 
/// 2. `[]` token_mint: [Mint] 
/// 3. `[signer]` authority: [AccountInfo] Must be admin or treasurer
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 5. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 6. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 7. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 8. `[]` mint: [Mint] The token mint for the new associated token account
/// 9. `[]` token_program: [AccountInfo] SPL Token program
/// 10. `[]` csl_spl_assoc_token_v0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn initialize_token_vault(ctx: Context<InitializeTokenVault>, _treasury_seed_name: String) -> Result<()> {
		initialize_token_vault::handler(ctx, )
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` token_vault: [TokenVault] 
/// 2. `[]` token_mint: [Mint] 
/// 3. `[signer]` depositor: [AccountInfo] 
/// 4. `[writable]` source: [AccountInfo] The source account.
/// 5. `[writable]` destination: [AccountInfo] The destination account.
/// 6. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 7. `[]` csl_spl_token_v0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - amount: [u64] Amount of tokens to deposit
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn deposit_token(ctx: Context<DepositToken>, amount: u64, _treasury_seed_name: String) -> Result<()> {
		deposit_token::handler(ctx, amount)
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` recipient: [Recipient] 
/// 2. `[signer]` authority: [AccountInfo] Must be admin or treasurer
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - name: [String] Name of the recipient
/// - role: [u8] Role of the recipient (0=Regular, 1=Privileged)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn add_recipient(ctx: Context<AddRecipient>, recipient_address: Pubkey, name: String, role: u8, _treasury_seed_name: String) -> Result<()> {
		add_recipient::handler(ctx, recipient_address, name, role)
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` recipient: [Recipient] 
/// 2. `[signer]` authority: [AccountInfo] Must be admin or treasurer
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - name: [Option<String>] Optional new name for the recipient
/// - role: [Option<u8>] Optional new role for the recipient
/// - is_active: [Option<bool>] Optional update to active status
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn update_recipient(ctx: Context<UpdateRecipient>, recipient_address: Pubkey, name: Option<String>, role: Option<u8>, is_active: Option<bool>, _treasury_seed_name: String) -> Result<()> {
		update_recipient::handler(ctx, recipient_address, name, role, is_active)
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[]` recipient: [Recipient] 
/// 2. `[writable]` payout_schedule: [PayoutSchedule] 
/// 3. `[signer]` authority: [AccountInfo] Must be admin or treasurer
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - schedule_id: [u64] Unique identifier for this schedule
/// - amount: [u64] Amount to be paid out
/// - token_mint: [Option<Pubkey>] Optional token mint address (null for SOL)
/// - start_time: [i64] When this schedule starts
/// - interval_seconds: [u64] Interval between payouts in seconds (0 for one-time)
/// - max_executions: [u64] Maximum number of executions (0 for unlimited)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn create_payout_schedule(ctx: Context<CreatePayoutSchedule>, recipient_address: Pubkey, schedule_id: u64, amount: u64, token_mint: Option<Pubkey>, start_time: i64, interval_seconds: u64, max_executions: u64, _treasury_seed_name: String) -> Result<()> {
		create_payout_schedule::handler(ctx, recipient_address, schedule_id, amount, token_mint, start_time, interval_seconds, max_executions)
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[]` recipient: [Recipient] 
/// 2. `[writable]` payout_schedule: [PayoutSchedule] 
/// 3. `[signer]` authority: [AccountInfo] Must be admin or treasurer
///
/// Data:
/// - recipient_address: [Pubkey] The recipient's wallet address
/// - schedule_id: [u64] Unique identifier for this schedule
/// - amount: [Option<u64>] Optional new amount to be paid out
/// - start_time: [Option<i64>] Optional new start time
/// - interval_seconds: [Option<u64>] Optional new interval between payouts
/// - max_executions: [Option<u64>] Optional new maximum number of executions
/// - is_active: [Option<bool>] Optional update to active status
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn update_payout_schedule(ctx: Context<UpdatePayoutSchedule>, recipient_address: Pubkey, schedule_id: u64, amount: Option<u64>, start_time: Option<i64>, interval_seconds: Option<u64>, max_executions: Option<u64>, is_active: Option<bool>, _treasury_seed_name: String) -> Result<()> {
		update_payout_schedule::handler(ctx, recipient_address, schedule_id, amount, start_time, interval_seconds, max_executions, is_active)
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
	pub fn execute_sol_payout(ctx: Context<ExecuteSolPayout>, schedule_id: u64, _treasury_seed_name: String) -> Result<()> {
		execute_sol_payout::handler(ctx, schedule_id)
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
	pub fn execute_token_payout(ctx: Context<ExecuteTokenPayout>, recipient_address: Pubkey, schedule_id: u64, _treasury_seed_name: String) -> Result<()> {
		execute_token_payout::handler(ctx, recipient_address, schedule_id)
	}

/// Accounts:
/// 0. `[writable]` treasury: [TreasuryConfig] 
/// 1. `[writable, signer]` admin: [AccountInfo] 
///
/// Data:
/// - amount: [u64] Amount of SOL to withdraw (in lamports)
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn emergency_withdraw_sol(ctx: Context<EmergencyWithdrawSol>, amount: u64, _treasury_seed_name: String) -> Result<()> {
		emergency_withdraw_sol::handler(ctx, amount)
	}

/// Accounts:
/// 0. `[]` treasury: [TreasuryConfig] 
/// 1. `[writable]` token_vault: [TokenVault] 
/// 2. `[]` token_mint: [Mint] 
/// 3. `[signer]` admin: [AccountInfo] 
/// 4. `[writable]` source: [AccountInfo] The source account.
/// 5. `[writable]` destination: [AccountInfo] The destination account.
/// 6. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 7. `[]` csl_spl_token_v0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
///
/// Data:
/// - amount: [u64] Amount of tokens to withdraw
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
	pub fn emergency_withdraw_token(ctx: Context<EmergencyWithdrawToken>, amount: u64, _treasury_seed_name: String) -> Result<()> {
		emergency_withdraw_token::handler(ctx, amount)
	}



}
