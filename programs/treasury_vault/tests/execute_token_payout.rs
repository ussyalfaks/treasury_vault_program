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
async fn execute_token_payout_ix_success() {
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

	program_test.add_program(
		"csl_spl_token",
		Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
		None,
	);

	// DATA
	let recipient_address: Pubkey = Pubkey::default();
	let schedule_id: u64 = Default::default();
	let treasury_seed_name: String = Default::default();

	// KEYPAIR
	let fee_payer_keypair = Keypair::new();
	let authority_keypair = Keypair::new();
	let token_mint_keypair = Keypair::new();

	// PUBKEY
	let fee_payer_pubkey = fee_payer_keypair.pubkey();
	let authority_pubkey = authority_keypair.pubkey();
	let token_mint_pubkey = token_mint_keypair.pubkey();
	let source_pubkey = Pubkey::new_unique();
	let destination_pubkey = Pubkey::new_unique();

	// EXECUTABLE PUBKEY
	let csl_spl_token_v0_0_0_pubkey = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

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

	let (payout_schedule_pda, _payout_schedule_pda_bump) = Pubkey::find_program_address(
		&[
			b"schedule",
			treasury_pda.as_ref(),
			recipient_pda.as_ref(),
			schedule_id.to_le_bytes().as_ref(),
		],
		&treasury_vault::ID,
	);

	let (token_vault_pda, _token_vault_pda_bump) = Pubkey::find_program_address(
		&[
			b"token_vault",
			treasury_pda.as_ref(),
			token_mint_pubkey.as_ref(),
		],
		&treasury_vault::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		fee_payer_pubkey,
		Account {
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		authority_pubkey,
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

	let ix = treasury_vault_ix_interface::execute_token_payout_ix_setup(
		&fee_payer_keypair,
		treasury_pda,
		recipient_pda,
		payout_schedule_pda,
		token_vault_pda,
		token_mint_pubkey,
		source_pubkey,
		destination_pubkey,
		&authority_keypair,
		csl_spl_token_v0_0_0_pubkey,
		recipient_address,
		schedule_id,
		&treasury_seed_name,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
