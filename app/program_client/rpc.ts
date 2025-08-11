import {
  AnchorProvider,
  BN,
  IdlAccounts,
  Program,
  web3,
} from "@coral-xyz/anchor";
import { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";
import { TreasuryVault } from "../../target/types/treasury_vault";
import idl from "../../target/idl/treasury_vault.json";
import * as pda from "./pda";

import { CslSplToken } from "../../target/types/csl_spl_token";
import idlCslSplToken from "../../target/idl/csl_spl_token.json";

import { CslSplAssocToken } from "../../target/types/csl_spl_assoc_token";
import idlCslSplAssocToken from "../../target/idl/csl_spl_assoc_token.json";



let _program: Program<TreasuryVault>;
let _programCslSplToken: Program<CslSplToken>;
let _programCslSplAssocToken: Program<CslSplAssocToken>;


export const initializeClient = (
    programId: web3.PublicKey,
    anchorProvider = AnchorProvider.env(),
) => {
    _program = new Program<TreasuryVault>(
        idl as never,
        programId,
        anchorProvider,
    );

    _programCslSplToken = new Program<CslSplToken>(
        idlCslSplToken as never,
        new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        anchorProvider,
    );
    _programCslSplAssocToken = new Program<CslSplAssocToken>(
        idlCslSplAssocToken as never,
        new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        anchorProvider,
    );

};

export type InitializeTreasuryArgs = {
  admin: web3.PublicKey;
  name: string;
  description: string;
  treasurer: web3.PublicKey;
  dailyLimit: bigint;
  weeklyLimit: bigint;
  monthlyLimit: bigint;
  requireTokenGate: boolean;
  tokenGateMint: web3.PublicKey | undefined;
  tokenGateAmount: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[signer]` admin: {@link PublicKey} 
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} Name of the treasury
 * - description: {@link string} type
 * - treasurer: {@link PublicKey} The treasurer authority that can approve payouts
 * - daily_limit: {@link BigInt} Maximum amount that can be spent in a day
 * - weekly_limit: {@link BigInt} Maximum amount that can be spent in a week
 * - monthly_limit: {@link BigInt} Maximum amount that can be spent in a month
 * - require_token_gate: {@link boolean} Whether token gating is required for recipients
 * - token_gate_mint: {@link PublicKey | undefined} Optional mint address for token gating
 * - token_gate_amount: {@link BigInt} Minimum amount of tokens required for token gating
 */
export const initializeTreasuryBuilder = (
	args: InitializeTreasuryArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.name,
    }, _program.programId);

  return _program
    .methods
    .initializeTreasury(
      args.name,
      args.description,
      args.treasurer,
      new BN(args.dailyLimit.toString()),
      new BN(args.weeklyLimit.toString()),
      new BN(args.monthlyLimit.toString()),
      args.requireTokenGate,
      args.tokenGateMint,
      new BN(args.tokenGateAmount.toString()),
    )
    .accountsStrict({
      admin: args.admin,
      treasury: treasuryPubkey,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[signer]` admin: {@link PublicKey} 
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} Name of the treasury
 * - description: {@link string} type
 * - treasurer: {@link PublicKey} The treasurer authority that can approve payouts
 * - daily_limit: {@link BigInt} Maximum amount that can be spent in a day
 * - weekly_limit: {@link BigInt} Maximum amount that can be spent in a week
 * - monthly_limit: {@link BigInt} Maximum amount that can be spent in a month
 * - require_token_gate: {@link boolean} Whether token gating is required for recipients
 * - token_gate_mint: {@link PublicKey | undefined} Optional mint address for token gating
 * - token_gate_amount: {@link BigInt} Minimum amount of tokens required for token gating
 */
export const initializeTreasury = (
	args: InitializeTreasuryArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    initializeTreasuryBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[signer]` admin: {@link PublicKey} 
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} Name of the treasury
 * - description: {@link string} type
 * - treasurer: {@link PublicKey} The treasurer authority that can approve payouts
 * - daily_limit: {@link BigInt} Maximum amount that can be spent in a day
 * - weekly_limit: {@link BigInt} Maximum amount that can be spent in a week
 * - monthly_limit: {@link BigInt} Maximum amount that can be spent in a month
 * - require_token_gate: {@link boolean} Whether token gating is required for recipients
 * - token_gate_mint: {@link PublicKey | undefined} Optional mint address for token gating
 * - token_gate_amount: {@link BigInt} Minimum amount of tokens required for token gating
 */
export const initializeTreasurySendAndConfirm = async (
  args: Omit<InitializeTreasuryArgs, "admin"> & {
    signers: {
      admin: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return initializeTreasuryBuilder({
      ...args,
      admin: args.signers.admin.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.admin])
    .rpc();
}

export type UpdateTreasuryConfigArgs = {
  admin: web3.PublicKey;
  newAdmin: web3.PublicKey | undefined;
  newTreasurer: web3.PublicKey | undefined;
  description: string | undefined;
  dailyLimit: bigint | undefined;
  weeklyLimit: bigint | undefined;
  monthlyLimit: bigint | undefined;
  requireTokenGate: boolean | undefined;
  tokenGateMint: web3.PublicKey | undefined;
  tokenGateAmount: bigint | undefined;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[signer]` admin: {@link PublicKey} 
 *
 * Data:
 * - new_admin: {@link PublicKey | undefined} Optional new admin authority
 * - new_treasurer: {@link PublicKey | undefined} Optional new treasurer authority
 * - description: {@link string | undefined} type
 * - daily_limit: {@link BigInt | undefined} Optional new daily limit
 * - weekly_limit: {@link BigInt | undefined} Optional new weekly limit
 * - monthly_limit: {@link BigInt | undefined} Optional new monthly limit
 * - require_token_gate: {@link boolean | undefined} Optional update to token gating requirement
 * - token_gate_mint: {@link PublicKey | undefined} Optional new token gate mint
 * - token_gate_amount: {@link BigInt | undefined} Optional new token gate amount
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updateTreasuryConfigBuilder = (
	args: UpdateTreasuryConfigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);

  return _program
    .methods
    .updateTreasuryConfig(
      args.newAdmin,
      args.newTreasurer,
      args.description,
      args.dailyLimit ? new BN(args.dailyLimit.toString()) : undefined,
      args.weeklyLimit ? new BN(args.weeklyLimit.toString()) : undefined,
      args.monthlyLimit ? new BN(args.monthlyLimit.toString()) : undefined,
      args.requireTokenGate,
      args.tokenGateMint,
      args.tokenGateAmount ? new BN(args.tokenGateAmount.toString()) : undefined,
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      admin: args.admin,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[signer]` admin: {@link PublicKey} 
 *
 * Data:
 * - new_admin: {@link PublicKey | undefined} Optional new admin authority
 * - new_treasurer: {@link PublicKey | undefined} Optional new treasurer authority
 * - description: {@link string | undefined} type
 * - daily_limit: {@link BigInt | undefined} Optional new daily limit
 * - weekly_limit: {@link BigInt | undefined} Optional new weekly limit
 * - monthly_limit: {@link BigInt | undefined} Optional new monthly limit
 * - require_token_gate: {@link boolean | undefined} Optional update to token gating requirement
 * - token_gate_mint: {@link PublicKey | undefined} Optional new token gate mint
 * - token_gate_amount: {@link BigInt | undefined} Optional new token gate amount
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updateTreasuryConfig = (
	args: UpdateTreasuryConfigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    updateTreasuryConfigBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[signer]` admin: {@link PublicKey} 
 *
 * Data:
 * - new_admin: {@link PublicKey | undefined} Optional new admin authority
 * - new_treasurer: {@link PublicKey | undefined} Optional new treasurer authority
 * - description: {@link string | undefined} type
 * - daily_limit: {@link BigInt | undefined} Optional new daily limit
 * - weekly_limit: {@link BigInt | undefined} Optional new weekly limit
 * - monthly_limit: {@link BigInt | undefined} Optional new monthly limit
 * - require_token_gate: {@link boolean | undefined} Optional update to token gating requirement
 * - token_gate_mint: {@link PublicKey | undefined} Optional new token gate mint
 * - token_gate_amount: {@link BigInt | undefined} Optional new token gate amount
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updateTreasuryConfigSendAndConfirm = async (
  args: Omit<UpdateTreasuryConfigArgs, "admin"> & {
    signers: {
      admin: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return updateTreasuryConfigBuilder({
      ...args,
      admin: args.signers.admin.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.admin])
    .rpc();
}

export type DepositSolArgs = {
  depositor: web3.PublicKey;
  amount: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[writable, signer]` depositor: {@link PublicKey} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of SOL to deposit (in lamports)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const depositSolBuilder = (
	args: DepositSolArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);

  return _program
    .methods
    .depositSol(
      new BN(args.amount.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      depositor: args.depositor,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[writable, signer]` depositor: {@link PublicKey} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of SOL to deposit (in lamports)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const depositSol = (
	args: DepositSolArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    depositSolBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[writable, signer]` depositor: {@link PublicKey} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of SOL to deposit (in lamports)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const depositSolSendAndConfirm = async (
  args: Omit<DepositSolArgs, "depositor"> & {
    signers: {
      depositor: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return depositSolBuilder({
      ...args,
      depositor: args.signers.depositor.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.depositor])
    .rpc();
}

export type InitializeTokenVaultArgs = {
  tokenMint: web3.PublicKey;
  authority: web3.PublicKey;
  funding: web3.PublicKey;
  wallet: web3.PublicKey;
  mint: web3.PublicKey;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 * 5. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
 * 6. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
 * 7. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
 * 8. `[]` mint: {@link Mint} The token mint for the new associated token account
 * 9. `[]` token_program: {@link PublicKey} SPL Token program
 * 10. `[]` csl_spl_assoc_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplAssocTokenProgram v0.0.0
 *
 * Data:
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const initializeTokenVaultBuilder = (
	args: InitializeTokenVaultArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [tokenVaultPubkey] = pda.deriveTokenVaultPDA({
        treasury: args.treasury,
        tokenMint: args.tokenMint,
    }, _program.programId);
    const [assocTokenAccountPubkey] = pda.CslSplTokenPDAs.deriveAccountPDA({
        wallet: args.wallet,
        tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        mint: args.mint,
    }, new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"));

  return _program
    .methods
    .initializeTokenVault(
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      tokenVault: tokenVaultPubkey,
      tokenMint: args.tokenMint,
      authority: args.authority,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
      funding: args.funding,
      assocTokenAccount: assocTokenAccountPubkey,
      wallet: args.wallet,
      mint: args.mint,
      tokenProgram: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
      cslSplAssocTokenV000: new web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 * 5. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
 * 6. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
 * 7. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
 * 8. `[]` mint: {@link Mint} The token mint for the new associated token account
 * 9. `[]` token_program: {@link PublicKey} SPL Token program
 * 10. `[]` csl_spl_assoc_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplAssocTokenProgram v0.0.0
 *
 * Data:
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const initializeTokenVault = (
	args: InitializeTokenVaultArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    initializeTokenVaultBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 * 5. `[writable, signer]` funding: {@link PublicKey} Funding account (must be a system account)
 * 6. `[writable]` assoc_token_account: {@link PublicKey} Associated token account address to be created
 * 7. `[]` wallet: {@link PublicKey} Wallet address for the new associated token account
 * 8. `[]` mint: {@link Mint} The token mint for the new associated token account
 * 9. `[]` token_program: {@link PublicKey} SPL Token program
 * 10. `[]` csl_spl_assoc_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplAssocTokenProgram v0.0.0
 *
 * Data:
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const initializeTokenVaultSendAndConfirm = async (
  args: Omit<InitializeTokenVaultArgs, "authority" | "funding"> & {
    signers: {
      authority: web3.Signer,
      funding: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return initializeTokenVaultBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
      funding: args.signers.funding.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority, args.signers.funding])
    .rpc();
}

export type DepositTokenArgs = {
  tokenMint: web3.PublicKey;
  depositor: web3.PublicKey;
  source: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  amount: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` depositor: {@link PublicKey} 
 * 4. `[writable]` source: {@link PublicKey} The source account.
 * 5. `[writable]` destination: {@link PublicKey} The destination account.
 * 6. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 7. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to deposit
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const depositTokenBuilder = (
	args: DepositTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [tokenVaultPubkey] = pda.deriveTokenVaultPDA({
        treasury: args.treasury,
        tokenMint: args.tokenMint,
    }, _program.programId);

  return _program
    .methods
    .depositToken(
      new BN(args.amount.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      tokenVault: tokenVaultPubkey,
      tokenMint: args.tokenMint,
      depositor: args.depositor,
      source: args.source,
      destination: args.destination,
      authority: args.authority,
      cslSplTokenV000: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` depositor: {@link PublicKey} 
 * 4. `[writable]` source: {@link PublicKey} The source account.
 * 5. `[writable]` destination: {@link PublicKey} The destination account.
 * 6. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 7. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to deposit
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const depositToken = (
	args: DepositTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    depositTokenBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` depositor: {@link PublicKey} 
 * 4. `[writable]` source: {@link PublicKey} The source account.
 * 5. `[writable]` destination: {@link PublicKey} The destination account.
 * 6. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 7. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to deposit
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const depositTokenSendAndConfirm = async (
  args: Omit<DepositTokenArgs, "depositor" | "authority"> & {
    signers: {
      depositor: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return depositTokenBuilder({
      ...args,
      depositor: args.signers.depositor.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.depositor, args.signers.authority])
    .rpc();
}

export type AddRecipientArgs = {
  authority: web3.PublicKey;
  recipientAddress: web3.PublicKey;
  name: string;
  role: number;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` recipient: {@link Recipient} 
 * 2. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - name: {@link string} Name of the recipient
 * - role: {@link number} Role of the recipient (0=Regular, 1=Privileged)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const addRecipientBuilder = (
	args: AddRecipientArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [recipientPubkey] = pda.deriveRecipientPDA({
        treasury: args.treasury,
        recipientAddress: args.recipientAddress,
    }, _program.programId);

  return _program
    .methods
    .addRecipient(
      args.recipientAddress,
      args.name,
      args.role,
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      recipient: recipientPubkey,
      authority: args.authority,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` recipient: {@link Recipient} 
 * 2. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - name: {@link string} Name of the recipient
 * - role: {@link number} Role of the recipient (0=Regular, 1=Privileged)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const addRecipient = (
	args: AddRecipientArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    addRecipientBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` recipient: {@link Recipient} 
 * 2. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - name: {@link string} Name of the recipient
 * - role: {@link number} Role of the recipient (0=Regular, 1=Privileged)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const addRecipientSendAndConfirm = async (
  args: Omit<AddRecipientArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return addRecipientBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type UpdateRecipientArgs = {
  authority: web3.PublicKey;
  recipientAddress: web3.PublicKey;
  name: string | undefined;
  role: number | undefined;
  isActive: boolean | undefined;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` recipient: {@link Recipient} 
 * 2. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - name: {@link string | undefined} Optional new name for the recipient
 * - role: {@link number | undefined} Optional new role for the recipient
 * - is_active: {@link boolean | undefined} Optional update to active status
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updateRecipientBuilder = (
	args: UpdateRecipientArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [recipientPubkey] = pda.deriveRecipientPDA({
        treasury: args.treasury,
        recipientAddress: args.recipientAddress,
    }, _program.programId);

  return _program
    .methods
    .updateRecipient(
      args.recipientAddress,
      args.name,
      args.role,
      args.isActive,
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      recipient: recipientPubkey,
      authority: args.authority,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` recipient: {@link Recipient} 
 * 2. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - name: {@link string | undefined} Optional new name for the recipient
 * - role: {@link number | undefined} Optional new role for the recipient
 * - is_active: {@link boolean | undefined} Optional update to active status
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updateRecipient = (
	args: UpdateRecipientArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    updateRecipientBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` recipient: {@link Recipient} 
 * 2. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - name: {@link string | undefined} Optional new name for the recipient
 * - role: {@link number | undefined} Optional new role for the recipient
 * - is_active: {@link boolean | undefined} Optional update to active status
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updateRecipientSendAndConfirm = async (
  args: Omit<UpdateRecipientArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return updateRecipientBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type CreatePayoutScheduleArgs = {
  authority: web3.PublicKey;
  recipientAddress: web3.PublicKey;
  scheduleId: bigint;
  amount: bigint;
  tokenMint: web3.PublicKey | undefined;
  startTime: bigint;
  intervalSeconds: bigint;
  maxExecutions: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[]` recipient: {@link Recipient} 
 * 2. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - amount: {@link BigInt} Amount to be paid out
 * - token_mint: {@link PublicKey | undefined} Optional token mint address (null for SOL)
 * - start_time: {@link BigInt} When this schedule starts
 * - interval_seconds: {@link BigInt} Interval between payouts in seconds (0 for one-time)
 * - max_executions: {@link BigInt} Maximum number of executions (0 for unlimited)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const createPayoutScheduleBuilder = (
	args: CreatePayoutScheduleArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [recipientPubkey] = pda.deriveRecipientPDA({
        treasury: args.treasury,
        recipientAddress: args.recipientAddress,
    }, _program.programId);
    const [payoutSchedulePubkey] = pda.derivePayoutSchedulePDA({
        treasury: args.treasury,
        recipient: args.recipient,
        scheduleId: args.scheduleId,
    }, _program.programId);

  return _program
    .methods
    .createPayoutSchedule(
      args.recipientAddress,
      new BN(args.scheduleId.toString()),
      new BN(args.amount.toString()),
      args.tokenMint,
      new BN(args.startTime.toString()),
      new BN(args.intervalSeconds.toString()),
      new BN(args.maxExecutions.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      recipient: recipientPubkey,
      payoutSchedule: payoutSchedulePubkey,
      authority: args.authority,
      systemProgram: new web3.PublicKey("11111111111111111111111111111111"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[]` recipient: {@link Recipient} 
 * 2. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - amount: {@link BigInt} Amount to be paid out
 * - token_mint: {@link PublicKey | undefined} Optional token mint address (null for SOL)
 * - start_time: {@link BigInt} When this schedule starts
 * - interval_seconds: {@link BigInt} Interval between payouts in seconds (0 for one-time)
 * - max_executions: {@link BigInt} Maximum number of executions (0 for unlimited)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const createPayoutSchedule = (
	args: CreatePayoutScheduleArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createPayoutScheduleBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[]` recipient: {@link Recipient} 
 * 2. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - amount: {@link BigInt} Amount to be paid out
 * - token_mint: {@link PublicKey | undefined} Optional token mint address (null for SOL)
 * - start_time: {@link BigInt} When this schedule starts
 * - interval_seconds: {@link BigInt} Interval between payouts in seconds (0 for one-time)
 * - max_executions: {@link BigInt} Maximum number of executions (0 for unlimited)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const createPayoutScheduleSendAndConfirm = async (
  args: Omit<CreatePayoutScheduleArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createPayoutScheduleBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type UpdatePayoutScheduleArgs = {
  authority: web3.PublicKey;
  recipientAddress: web3.PublicKey;
  scheduleId: bigint;
  amount: bigint | undefined;
  startTime: bigint | undefined;
  intervalSeconds: bigint | undefined;
  maxExecutions: bigint | undefined;
  isActive: boolean | undefined;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[]` recipient: {@link Recipient} 
 * 2. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - amount: {@link BigInt | undefined} Optional new amount to be paid out
 * - start_time: {@link BigInt | undefined} Optional new start time
 * - interval_seconds: {@link BigInt | undefined} Optional new interval between payouts
 * - max_executions: {@link BigInt | undefined} Optional new maximum number of executions
 * - is_active: {@link boolean | undefined} Optional update to active status
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updatePayoutScheduleBuilder = (
	args: UpdatePayoutScheduleArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [recipientPubkey] = pda.deriveRecipientPDA({
        treasury: args.treasury,
        recipientAddress: args.recipientAddress,
    }, _program.programId);
    const [payoutSchedulePubkey] = pda.derivePayoutSchedulePDA({
        treasury: args.treasury,
        recipient: args.recipient,
        scheduleId: args.scheduleId,
    }, _program.programId);

  return _program
    .methods
    .updatePayoutSchedule(
      args.recipientAddress,
      new BN(args.scheduleId.toString()),
      args.amount ? new BN(args.amount.toString()) : undefined,
      args.startTime ? new BN(args.startTime.toString()) : undefined,
      args.intervalSeconds ? new BN(args.intervalSeconds.toString()) : undefined,
      args.maxExecutions ? new BN(args.maxExecutions.toString()) : undefined,
      args.isActive,
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      recipient: recipientPubkey,
      payoutSchedule: payoutSchedulePubkey,
      authority: args.authority,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[]` recipient: {@link Recipient} 
 * 2. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - amount: {@link BigInt | undefined} Optional new amount to be paid out
 * - start_time: {@link BigInt | undefined} Optional new start time
 * - interval_seconds: {@link BigInt | undefined} Optional new interval between payouts
 * - max_executions: {@link BigInt | undefined} Optional new maximum number of executions
 * - is_active: {@link boolean | undefined} Optional update to active status
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updatePayoutSchedule = (
	args: UpdatePayoutScheduleArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    updatePayoutScheduleBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[]` recipient: {@link Recipient} 
 * 2. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 3. `[signer]` authority: {@link PublicKey} Must be admin or treasurer
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - amount: {@link BigInt | undefined} Optional new amount to be paid out
 * - start_time: {@link BigInt | undefined} Optional new start time
 * - interval_seconds: {@link BigInt | undefined} Optional new interval between payouts
 * - max_executions: {@link BigInt | undefined} Optional new maximum number of executions
 * - is_active: {@link boolean | undefined} Optional update to active status
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const updatePayoutScheduleSendAndConfirm = async (
  args: Omit<UpdatePayoutScheduleArgs, "authority"> & {
    signers: {
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return updatePayoutScheduleBuilder({
      ...args,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.authority])
    .rpc();
}

export type ExecuteSolPayoutArgs = {
  feePayer: web3.PublicKey;
  recipientAddress: web3.PublicKey;
  scheduleId: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[writable]` recipient: {@link Recipient} 
 * 3. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 4. `[writable]` recipient_address: {@link PublicKey} 
 *
 * Data:
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const executeSolPayoutBuilder = (
	args: ExecuteSolPayoutArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [recipientPubkey] = pda.deriveRecipientPDA({
        treasury: args.treasury,
        recipientAddress: args.recipientAddress,
    }, _program.programId);
    const [payoutSchedulePubkey] = pda.derivePayoutSchedulePDA({
        treasury: args.treasury,
        recipient: args.recipient,
        scheduleId: args.scheduleId,
    }, _program.programId);

  return _program
    .methods
    .executeSolPayout(
      new BN(args.scheduleId.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      treasury: treasuryPubkey,
      recipient: recipientPubkey,
      payoutSchedule: payoutSchedulePubkey,
      recipientAddress: args.recipientAddress,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[writable]` recipient: {@link Recipient} 
 * 3. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 4. `[writable]` recipient_address: {@link PublicKey} 
 *
 * Data:
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const executeSolPayout = (
	args: ExecuteSolPayoutArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    executeSolPayoutBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[writable]` recipient: {@link Recipient} 
 * 3. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 4. `[writable]` recipient_address: {@link PublicKey} 
 *
 * Data:
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const executeSolPayoutSendAndConfirm = async (
  args: Omit<ExecuteSolPayoutArgs, "feePayer"> & {
    signers: {
      feePayer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return executeSolPayoutBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer])
    .rpc();
}

export type ExecuteTokenPayoutArgs = {
  feePayer: web3.PublicKey;
  tokenMint: web3.PublicKey;
  source: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  recipientAddress: web3.PublicKey;
  scheduleId: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[writable]` recipient: {@link Recipient} 
 * 3. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 4. `[writable]` token_vault: {@link TokenVault} 
 * 5. `[]` token_mint: {@link Mint} 
 * 6. `[writable]` source: {@link PublicKey} The source account.
 * 7. `[writable]` destination: {@link PublicKey} The destination account.
 * 8. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 9. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const executeTokenPayoutBuilder = (
	args: ExecuteTokenPayoutArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [recipientPubkey] = pda.deriveRecipientPDA({
        treasury: args.treasury,
        recipientAddress: args.recipientAddress,
    }, _program.programId);
    const [payoutSchedulePubkey] = pda.derivePayoutSchedulePDA({
        treasury: args.treasury,
        recipient: args.recipient,
        scheduleId: args.scheduleId,
    }, _program.programId);
    const [tokenVaultPubkey] = pda.deriveTokenVaultPDA({
        treasury: args.treasury,
        tokenMint: args.tokenMint,
    }, _program.programId);

  return _program
    .methods
    .executeTokenPayout(
      args.recipientAddress,
      new BN(args.scheduleId.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      treasury: treasuryPubkey,
      recipient: recipientPubkey,
      payoutSchedule: payoutSchedulePubkey,
      tokenVault: tokenVaultPubkey,
      tokenMint: args.tokenMint,
      source: args.source,
      destination: args.destination,
      authority: args.authority,
      cslSplTokenV000: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[writable]` recipient: {@link Recipient} 
 * 3. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 4. `[writable]` token_vault: {@link TokenVault} 
 * 5. `[]` token_mint: {@link Mint} 
 * 6. `[writable]` source: {@link PublicKey} The source account.
 * 7. `[writable]` destination: {@link PublicKey} The destination account.
 * 8. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 9. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const executeTokenPayout = (
	args: ExecuteTokenPayoutArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    executeTokenPayoutBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` treasury: {@link TreasuryConfig} 
 * 2. `[writable]` recipient: {@link Recipient} 
 * 3. `[writable]` payout_schedule: {@link PayoutSchedule} 
 * 4. `[writable]` token_vault: {@link TokenVault} 
 * 5. `[]` token_mint: {@link Mint} 
 * 6. `[writable]` source: {@link PublicKey} The source account.
 * 7. `[writable]` destination: {@link PublicKey} The destination account.
 * 8. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 9. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - recipient_address: {@link PublicKey} The recipient's wallet address
 * - schedule_id: {@link BigInt} Unique identifier for this schedule
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const executeTokenPayoutSendAndConfirm = async (
  args: Omit<ExecuteTokenPayoutArgs, "feePayer" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return executeTokenPayoutBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority])
    .rpc();
}

export type EmergencyWithdrawSolArgs = {
  admin: web3.PublicKey;
  amount: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[writable, signer]` admin: {@link PublicKey} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of SOL to withdraw (in lamports)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const emergencyWithdrawSolBuilder = (
	args: EmergencyWithdrawSolArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);

  return _program
    .methods
    .emergencyWithdrawSol(
      new BN(args.amount.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      admin: args.admin,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[writable, signer]` admin: {@link PublicKey} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of SOL to withdraw (in lamports)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const emergencyWithdrawSol = (
	args: EmergencyWithdrawSolArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    emergencyWithdrawSolBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[writable]` treasury: {@link TreasuryConfig} 
 * 1. `[writable, signer]` admin: {@link PublicKey} 
 *
 * Data:
 * - amount: {@link BigInt} Amount of SOL to withdraw (in lamports)
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const emergencyWithdrawSolSendAndConfirm = async (
  args: Omit<EmergencyWithdrawSolArgs, "admin"> & {
    signers: {
      admin: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return emergencyWithdrawSolBuilder({
      ...args,
      admin: args.signers.admin.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.admin])
    .rpc();
}

export type EmergencyWithdrawTokenArgs = {
  tokenMint: web3.PublicKey;
  admin: web3.PublicKey;
  source: web3.PublicKey;
  destination: web3.PublicKey;
  authority: web3.PublicKey;
  amount: bigint;
  treasurySeedName: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` admin: {@link PublicKey} 
 * 4. `[writable]` source: {@link PublicKey} The source account.
 * 5. `[writable]` destination: {@link PublicKey} The destination account.
 * 6. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 7. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to withdraw
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const emergencyWithdrawTokenBuilder = (
	args: EmergencyWithdrawTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<TreasuryVault, never> => {
    const [treasuryPubkey] = pda.deriveTreasuryPDA({
        name: args.treasurySeedName,
    }, _program.programId);
    const [tokenVaultPubkey] = pda.deriveTokenVaultPDA({
        treasury: args.treasury,
        tokenMint: args.tokenMint,
    }, _program.programId);

  return _program
    .methods
    .emergencyWithdrawToken(
      new BN(args.amount.toString()),
      args.treasurySeedName,
    )
    .accountsStrict({
      treasury: treasuryPubkey,
      tokenVault: tokenVaultPubkey,
      tokenMint: args.tokenMint,
      admin: args.admin,
      source: args.source,
      destination: args.destination,
      authority: args.authority,
      cslSplTokenV000: new web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` admin: {@link PublicKey} 
 * 4. `[writable]` source: {@link PublicKey} The source account.
 * 5. `[writable]` destination: {@link PublicKey} The destination account.
 * 6. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 7. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to withdraw
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const emergencyWithdrawToken = (
	args: EmergencyWithdrawTokenArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    emergencyWithdrawTokenBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Accounts:
 * 0. `[]` treasury: {@link TreasuryConfig} 
 * 1. `[writable]` token_vault: {@link TokenVault} 
 * 2. `[]` token_mint: {@link Mint} 
 * 3. `[signer]` admin: {@link PublicKey} 
 * 4. `[writable]` source: {@link PublicKey} The source account.
 * 5. `[writable]` destination: {@link PublicKey} The destination account.
 * 6. `[signer]` authority: {@link PublicKey} The source account's owner/delegate.
 * 7. `[]` csl_spl_token_v0_0_0: {@link PublicKey} Auto-generated, CslSplTokenProgram v0.0.0
 *
 * Data:
 * - amount: {@link BigInt} Amount of tokens to withdraw
 * - treasury_seed_name: {@link string} Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
 */
export const emergencyWithdrawTokenSendAndConfirm = async (
  args: Omit<EmergencyWithdrawTokenArgs, "admin" | "authority"> & {
    signers: {
      admin: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return emergencyWithdrawTokenBuilder({
      ...args,
      admin: args.signers.admin.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.admin, args.signers.authority])
    .rpc();
}

// Getters

export const getTreasuryConfig = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<TreasuryVault>["treasuryConfig"]> => _program.account.treasuryConfig.fetch(publicKey, commitment);

export const getRecipient = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<TreasuryVault>["recipient"]> => _program.account.recipient.fetch(publicKey, commitment);

export const getPayoutSchedule = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<TreasuryVault>["payoutSchedule"]> => _program.account.payoutSchedule.fetch(publicKey, commitment);

export const getTokenVault = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<TreasuryVault>["tokenVault"]> => _program.account.tokenVault.fetch(publicKey, commitment);
export module CslSplTokenGetters {
    export const getMint = (
        publicKey: web3.PublicKey,
        commitment?: web3.Commitment
    ): Promise<IdlAccounts<CslSplToken>["mint"]> => _programCslSplToken.account.mint.fetch(publicKey, commitment);
    
    export const getAccount = (
        publicKey: web3.PublicKey,
        commitment?: web3.Commitment
    ): Promise<IdlAccounts<CslSplToken>["account"]> => _programCslSplToken.account.account.fetch(publicKey, commitment);
}

