use {
	treasury_vault::{
			entry,
			ID as PROGRAM_ID,
	},
	solana_sdk::{
		entrypoint::{ProcessInstruction, ProgramResult},
		pubkey::Pubkey,
	},
	anchor_lang::prelude::AccountInfo,
	solana_program_test::*,
};

// Type alias for the entry function pointer used to convert the entry function into a ProcessInstruction function pointer.
pub type ProgramEntry = for<'info> fn(
	program_id: &Pubkey,
	accounts: &'info [AccountInfo<'info>],
	instruction_data: &[u8],
) -> ProgramResult;

// Macro to convert the entry function into a ProcessInstruction function pointer.
#[macro_export]
macro_rules! convert_entry {
	($entry:expr) => {
		// Use unsafe block to perform memory transmutation.
		unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
	};
}

pub fn get_program_test() -> ProgramTest {
	let program_test = ProgramTest::new(
		"treasury_vault",
		PROGRAM_ID,
		processor!(convert_entry!(entry)),
	);
	program_test
}
	
pub mod treasury_vault_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		treasury_vault::{
			ID as PROGRAM_ID,
			accounts as treasury_vault_accounts,
			instruction as treasury_vault_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	pub use treasury_vault::ID;

	pub fn initialize_treasury_ix_setup(
		admin: &Keypair,
		treasury: Pubkey,
		system_program: Pubkey,
		name: &String,
		description: &String,
		treasurer: Pubkey,
		daily_limit: u64,
		weekly_limit: u64,
		monthly_limit: u64,
		require_token_gate: bool,
		token_gate_mint: Option<Pubkey>,
		token_gate_amount: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::InitializeTreasury {
			admin: admin.pubkey(),
			treasury: treasury,
			system_program: system_program,
		};

		let data = 	treasury_vault_instruction::InitializeTreasury {
				name: name.clone(),
				description: description.clone(),
				treasurer,
				daily_limit,
				weekly_limit,
				monthly_limit,
				require_token_gate,
				token_gate_mint,
				token_gate_amount,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&admin.pubkey()),
		);

		transaction.sign(&[
			&admin,
		], recent_blockhash);

		return transaction;
	}

	pub fn update_treasury_config_ix_setup(
		treasury: Pubkey,
		admin: &Keypair,
		new_admin: Option<Pubkey>,
		new_treasurer: Option<Pubkey>,
		description: Option<String>,
		daily_limit: Option<u64>,
		weekly_limit: Option<u64>,
		monthly_limit: Option<u64>,
		require_token_gate: Option<bool>,
		token_gate_mint: Option<Pubkey>,
		token_gate_amount: Option<u64>,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::UpdateTreasuryConfig {
			treasury: treasury,
			admin: admin.pubkey(),
		};

		let data = 	treasury_vault_instruction::UpdateTreasuryConfig {
				new_admin,
				new_treasurer,
				description,
				daily_limit,
				weekly_limit,
				monthly_limit,
				require_token_gate,
				token_gate_mint,
				token_gate_amount,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&admin.pubkey()),
		);

		transaction.sign(&[
			&admin,
		], recent_blockhash);

		return transaction;
	}

	pub fn deposit_sol_ix_setup(
		treasury: Pubkey,
		depositor: &Keypair,
		amount: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::DepositSol {
			treasury: treasury,
			depositor: depositor.pubkey(),
		};

		let data = 	treasury_vault_instruction::DepositSol {
				amount,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&depositor.pubkey()),
		);

		transaction.sign(&[
			&depositor,
		], recent_blockhash);

		return transaction;
	}

	pub fn initialize_token_vault_ix_setup(
		treasury: Pubkey,
		token_vault: Pubkey,
		token_mint: Pubkey,
		authority: &Keypair,
		system_program: Pubkey,
		funding: &Keypair,
		assoc_token_account: Pubkey,
		wallet: Pubkey,
		mint: Pubkey,
		token_program: Pubkey,
		associated_token_program: Pubkey,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::InitializeTokenVault {
			treasury: treasury,
			token_vault: token_vault,
			token_mint: token_mint,
			authority: authority.pubkey(),
			system_program: system_program,
			funding: funding.pubkey(),
			assoc_token_account: assoc_token_account,
			wallet: wallet,
			mint: mint,
			token_program: token_program,
			associated_token_program: associated_token_program,
		};

		let data = 	treasury_vault_instruction::InitializeTokenVault {
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
			&funding,
		], recent_blockhash);

		return transaction;
	}

	pub fn deposit_token_ix_setup(
		treasury: Pubkey,
		token_vault: Pubkey,
		token_mint: Pubkey,
		depositor: &Keypair,
		source: Pubkey,
		destination: Pubkey,
		authority: &Keypair,
		csl_spl_token_v0_0_0: Pubkey,
		amount: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::DepositToken {
			treasury: treasury,
			token_vault: token_vault,
			token_mint: token_mint,
			depositor: depositor.pubkey(),
			source: source,
			destination: destination,
			authority: authority.pubkey(),
			csl_spl_token_v0_0_0: csl_spl_token_v0_0_0,
		};

		let data = 	treasury_vault_instruction::DepositToken {
				amount,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&depositor.pubkey()),
		);

		transaction.sign(&[
			&depositor,
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn add_recipient_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		authority: &Keypair,
		system_program: Pubkey,
		recipient_address: Pubkey,
		name: &String,
		role: u8,
		treasury_seed_name: &String,
		token_gate_mint: Option<Pubkey>,
		recipient_token_account: Option<Pubkey>,
		token_program: Option<Pubkey>,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::AddRecipient {
			treasury: treasury,
			recipient: recipient,
			authority: authority.pubkey(),
			system_program: system_program,
			token_gate_mint: token_gate_mint,
			recipient_token_account: recipient_token_account,
			token_program: token_program,
		};

		let data = 	treasury_vault_instruction::AddRecipient {
				recipient_address,
				name: name.clone(),
				role,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn update_recipient_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		authority: &Keypair,
		recipient_address: Pubkey,
		name: Option<String>,
		role: Option<u8>,
		is_active: Option<bool>,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::UpdateRecipient {
			treasury: treasury,
			recipient: recipient,
			authority: authority.pubkey(),
		};

		let data = 	treasury_vault_instruction::UpdateRecipient {
				recipient_address,
				name,
				role,
				is_active,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn create_payout_schedule_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		payout_schedule: Pubkey,
		authority: &Keypair,
		system_program: Pubkey,
		recipient_address: Pubkey,
		schedule_id: u64,
		amount: u64,
		token_mint: Option<Pubkey>,
		start_time: i64,
		interval_seconds: u64,
		max_executions: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::CreatePayoutSchedule {
			treasury: treasury,
			recipient: recipient,
			payout_schedule: payout_schedule,
			authority: authority.pubkey(),
			system_program: system_program,
		};

		let data = 	treasury_vault_instruction::CreatePayoutSchedule {
				recipient_address,
				schedule_id,
				amount,
				token_mint,
				start_time,
				interval_seconds,
				max_executions,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn update_payout_schedule_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		payout_schedule: Pubkey,
		authority: &Keypair,
		recipient_address: Pubkey,
		schedule_id: u64,
		amount: Option<u64>,
		start_time: Option<i64>,
		interval_seconds: Option<u64>,
		max_executions: Option<u64>,
		is_active: Option<bool>,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::UpdatePayoutSchedule {
			treasury: treasury,
			recipient: recipient,
			payout_schedule: payout_schedule,
			authority: authority.pubkey(),
		};

		let data = 	treasury_vault_instruction::UpdatePayoutSchedule {
				recipient_address,
				schedule_id,
				amount,
				start_time,
				interval_seconds,
				max_executions,
				is_active,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn execute_sol_payout_ix_setup(
		fee_payer: &Keypair,
		treasury: Pubkey,
		recipient: Pubkey,
		payout_schedule: Pubkey,
		recipient_address: Pubkey,
		schedule_id: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::ExecuteSolPayout {
			fee_payer: fee_payer.pubkey(),
			treasury: treasury,
			recipient: recipient,
			payout_schedule: payout_schedule,
			recipient_address: recipient_address,
		};

		let data = 	treasury_vault_instruction::ExecuteSolPayout {
				schedule_id,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
		], recent_blockhash);

		return transaction;
	}

	pub fn execute_token_payout_ix_setup(
		fee_payer: &Keypair,
		treasury: Pubkey,
		recipient: Pubkey,
		payout_schedule: Pubkey,
		token_vault: Pubkey,
		token_mint: Pubkey,
		source: Pubkey,
		destination: Pubkey,
		authority: &Keypair,
		csl_spl_token_v0_0_0: Pubkey,
		recipient_address: Pubkey,
		schedule_id: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::ExecuteTokenPayout {
			fee_payer: fee_payer.pubkey(),
			treasury: treasury,
			recipient: recipient,
			payout_schedule: payout_schedule,
			token_vault: token_vault,
			token_mint: token_mint,
			source: source,
			destination: destination,
			authority: authority.pubkey(),
			csl_spl_token_v0_0_0: csl_spl_token_v0_0_0,
		};

		let data = 	treasury_vault_instruction::ExecuteTokenPayout {
				recipient_address,
				schedule_id,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn emergency_withdraw_sol_ix_setup(
		treasury: Pubkey,
		admin: &Keypair,
		amount: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::EmergencyWithdrawSol {
			treasury: treasury,
			admin: admin.pubkey(),
		};

		let data = 	treasury_vault_instruction::EmergencyWithdrawSol {
				amount,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&admin.pubkey()),
		);

		transaction.sign(&[
			&admin,
		], recent_blockhash);

		return transaction;
	}

	pub fn emergency_withdraw_token_ix_setup(
		treasury: Pubkey,
		token_vault: Pubkey,
		token_mint: Pubkey,
		admin: &Keypair,
		source: Pubkey,
		destination: Pubkey,
		authority: &Keypair,
		csl_spl_token_v0_0_0: Pubkey,
		amount: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::EmergencyWithdrawToken {
			treasury: treasury,
			token_vault: token_vault,
			token_mint: token_mint,
			admin: admin.pubkey(),
			source: source,
			destination: destination,
			authority: authority.pubkey(),
			csl_spl_token_v0_0_0: csl_spl_token_v0_0_0,
		};

		let data = 	treasury_vault_instruction::EmergencyWithdrawToken {
				amount,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&admin.pubkey()),
		);

		transaction.sign(&[
			&admin,
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn create_streaming_schedule_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		streaming_schedule: Pubkey,
		authority: &Keypair,
		system_program: Pubkey,
		recipient_address: Pubkey,
		stream_id: u64,
		total_amount: u64,
		amount_per_second: u64,
		start_time: i64,
		cliff_time: i64,
		duration_seconds: u64,
		token_mint: Option<Pubkey>,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::CreateStreamingSchedule {
			treasury: treasury,
			recipient: recipient,
			streaming_schedule: streaming_schedule,
			authority: authority.pubkey(),
			system_program: system_program,
		};

		let data = 	treasury_vault_instruction::CreateStreamingSchedule {
				recipient_address,
				stream_id,
				total_amount,
				amount_per_second,
				start_time,
				cliff_time,
				duration_seconds,
				token_mint,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn cancel_stream_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		streaming_schedule: Pubkey,
		authority: &Keypair,
		recipient_wallet: Pubkey,
		recipient_address: Pubkey,
		stream_id: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::CancelStream {
			treasury: treasury,
			recipient: recipient,
			streaming_schedule: streaming_schedule,
			authority: authority.pubkey(),
			recipient_wallet: recipient_wallet,
		};

		let data = 	treasury_vault_instruction::CancelStream {
				recipient_address,
				stream_id,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&authority.pubkey()),
		);

		transaction.sign(&[
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn withdraw_from_stream_ix_setup(
		treasury: Pubkey,
		recipient: Pubkey,
		streaming_schedule: Pubkey,
		recipient_signer: &Keypair,
		stream_id: u64,
		treasury_seed_name: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = treasury_vault_accounts::WithdrawFromStream {
			treasury: treasury,
			recipient: recipient,
			streaming_schedule: streaming_schedule,
			recipient_signer: recipient_signer.pubkey(),
		};

		let data = 	treasury_vault_instruction::WithdrawFromStream {
				stream_id,
				_treasury_seed_name: treasury_seed_name.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&recipient_signer.pubkey()),
		);

		transaction.sign(&[
			&recipient_signer,
		], recent_blockhash);

		return transaction;
	}

}

pub mod csl_spl_token_ix_interface {
	use anchor_lang::prelude::*;
	declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
}

pub mod csl_spl_assoc_token_ix_interface {
	use anchor_lang::prelude::*;
	declare_id!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
}

pub mod system_program {
	use anchor_lang::prelude::*;
	declare_id!("11111111111111111111111111111111");
}
