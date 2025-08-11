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
async fn initialize_token_vault_ix_success() {
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
		"csl_spl_assoc_token",
		Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		None,
	);

	// DATA
	let treasury_seed_name: String = Default::default();

	// KEYPAIR
	let authority_keypair = Keypair::new();
	let funding_keypair = Keypair::new();
	let token_mint_keypair = Keypair::new();
	let mint_keypair = Keypair::new();

	// PUBKEY
	let authority_pubkey = authority_keypair.pubkey();
	let funding_pubkey = funding_keypair.pubkey();
	let token_mint_pubkey = token_mint_keypair.pubkey();
	let wallet_pubkey = Pubkey::new_unique();
	let mint_pubkey = mint_keypair.pubkey();
	let token_program_pubkey = csl_spl_token_ix_interface::ID;

	// EXECUTABLE PUBKEY
	let system_program_pubkey = Pubkey::from_str("11111111111111111111111111111111").unwrap();
	let csl_spl_assoc_token_v0_0_0_pubkey = Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();

	// PDA
	let (treasury_pda, _treasury_pda_bump) = Pubkey::find_program_address(
		&[
			b"treasury",
			treasury_seed_name.as_bytes().as_ref(),
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

	let (assoc_token_account_pda, _assoc_token_account_pda_bump) = Pubkey::find_program_address(
		&[
			wallet_pubkey.as_ref(),
			token_program_pubkey.as_ref(),
			mint_pubkey.as_ref(),
		],
		&csl_spl_token_ix_interface::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
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

	program_test.add_account(
		funding_pubkey,
		Account {
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = treasury_vault_ix_interface::initialize_token_vault_ix_setup(
		treasury_pda,
		token_vault_pda,
		token_mint_pubkey,
		&authority_keypair,
		system_program_pubkey,
		&funding_keypair,
		assoc_token_account_pda,
		wallet_pubkey,
		mint_pubkey,
		token_program_pubkey,
		csl_spl_assoc_token_v0_0_0_pubkey,
		&treasury_seed_name,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
