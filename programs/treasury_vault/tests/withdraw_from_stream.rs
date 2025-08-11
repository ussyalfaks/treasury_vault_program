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
async fn withdraw_from_stream_ix_success() {
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
	let stream_id: u64 = 1;
	let treasury_seed_name: String = String::from("test_treasury");

	// KEYPAIR
	let recipient_signer_keypair = Keypair::new();

	// PUBKEY
	let recipient_signer_pubkey = recipient_signer_keypair.pubkey();

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
			recipient_signer_pubkey.as_ref(),
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
		recipient_signer_pubkey,
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

	let ix = treasury_vault_ix_interface::withdraw_from_stream_ix_setup(
		treasury_pda,
		recipient_pda,
		streaming_schedule_pda,
		&recipient_signer_keypair,
		stream_id,
		&treasury_seed_name,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());
}
