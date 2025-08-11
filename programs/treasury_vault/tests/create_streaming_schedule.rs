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
async fn create_streaming_schedule_ix_success() {
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
	let recipient_address: Pubkey = Pubkey::default();
	let stream_id: u64 = 1;
	let total_amount: u64 = 1000000000; // 1 SOL
	let amount_per_second: u64 = 11574; // ~1 SOL per day
	let start_time: i64 = 1640995200; // 2022-01-01 00:00:00 UTC
	let cliff_time: i64 = 1640995200; // Same as start time for no cliff
	let duration_seconds: u64 = 86400; // 1 day
	let token_mint: Option<Pubkey> = None; // SOL stream
	let treasury_seed_name: String = String::from("test_treasury");

	// KEYPAIR
	let authority_keypair = Keypair::new();

	// PUBKEY
	let authority_pubkey = authority_keypair.pubkey();

	// EXECUTABLE PUBKEY
	let system_program_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();

	// PDA
	let (treasury_pda, _treasury_pda_bump) = Pubkey::find_program_address(
		&[
			b"treasury",
			treasury_seed_name.as_bytes().as_ref(),
		],
		&treasury_vault::ID,
	);

	let (recipient_pda, _recipient_pda_bump) = Pubkey::find_program_address(
		&[
			b"recipient",
			treasury_pda.as_ref(),
			recipient_address.as_ref(),
		],
		&treasury_vault::ID,
	);

	let (streaming_schedule_pda, _streaming_schedule_pda_bump) = Pubkey::find_program_address(
		&[
			b"stream",
			treasury_pda.as_ref(),
			recipient_pda.as_ref(),
			stream_id.to_le_bytes().as_ref(),
		],
		&treasury_vault::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		authority_pubkey,
		Account {
			lamports: 1_000_000_000, // 1 SOL for fees
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = treasury_vault_ix_interface::create_streaming_schedule_ix_setup(
		treasury_pda,
		recipient_pda,
		streaming_schedule_pda,
		&authority_keypair,
		system_program_pubkey,
		recipient_address,
		stream_id,
		total_amount,
		amount_per_second,
		start_time,
		cliff_time,
		duration_seconds,
		token_mint,
		&treasury_seed_name,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());
}
