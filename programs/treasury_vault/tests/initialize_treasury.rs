pub mod common;

use std::str::FromStr;
use {
    common::{
		get_program_test,
		treasury_vault_ix_interface,
		csl_spl_token_ix_interface,
		csl_spl_assoc_token_ix_interface,
	},
    solana_program_test::tokio,
    solana_sdk::{
        account::Account, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer, system_program,
    },
};


#[tokio::test]
async fn initialize_treasury_ix_success() {
	let mut program_test = get_program_test();

	// PROGRAMS
	program_test.prefer_bpf(true);

	program_test.add_program(
		"account_compression",
		Pubkey::from_str("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK").unwrap(),
		None,
	);

	program_test.add_program(
		"noop",
		Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap(),
		None,
	);

	// DATA
	let name: String = Default::default();
	let description: String = Default::default();
	let treasurer: Pubkey = Pubkey::default();
	let daily_limit: u64 = Default::default();
	let weekly_limit: u64 = Default::default();
	let monthly_limit: u64 = Default::default();
	let require_token_gate: bool = Default::default();
	let token_gate_mint = None;
	let token_gate_amount: u64 = Default::default();

	// KEYPAIR
	let admin_keypair = Keypair::new();

	// PUBKEY
	let admin_pubkey = admin_keypair.pubkey();

	// EXECUTABLE PUBKEY
	let system_program_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();

	// PDA
	let (treasury_pda, _treasury_pda_bump) = Pubkey::find_program_address(
		&[
			b"treasury",
			name.as_bytes().as_ref(),
		],
		&treasury_vault::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		admin_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = treasury_vault_ix_interface::initialize_treasury_ix_setup(
		&admin_keypair,
		treasury_pda,
		system_program_pubkey,
		&name,
		&description,
		treasurer,
		daily_limit,
		weekly_limit,
		monthly_limit,
		require_token_gate,
		token_gate_mint,
		token_gate_amount,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
