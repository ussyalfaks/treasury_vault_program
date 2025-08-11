import {PublicKey} from "@solana/web3.js";
import {BN} from "@coral-xyz/anchor";

export type TreasurySeeds = {
    name: string, 
};

export const deriveTreasuryPDA = (
    seeds: TreasurySeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("treasury"),
            Buffer.from(seeds.name, "utf8"),
        ],
        programId,
    )
};

export type RecipientSeeds = {
    treasury: PublicKey, 
    recipientAddress: PublicKey, 
};

export const deriveRecipientPDA = (
    seeds: RecipientSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("recipient"),
            seeds.treasury.toBuffer(),
            seeds.recipientAddress.toBuffer(),
        ],
        programId,
    )
};

export type PayoutScheduleSeeds = {
    treasury: PublicKey, 
    recipient: PublicKey, 
    scheduleId: bigint, 
};

export const derivePayoutSchedulePDA = (
    seeds: PayoutScheduleSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("schedule"),
            seeds.treasury.toBuffer(),
            seeds.recipient.toBuffer(),
            Buffer.from(BigUint64Array.from([seeds.scheduleId]).buffer),
        ],
        programId,
    )
};

export type TokenVaultSeeds = {
    treasury: PublicKey, 
    tokenMint: PublicKey, 
};

export const deriveTokenVaultPDA = (
    seeds: TokenVaultSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("token_vault"),
            seeds.treasury.toBuffer(),
            seeds.tokenMint.toBuffer(),
        ],
        programId,
    )
};

export module CslSplTokenPDAs {
    export type AccountSeeds = {
        wallet: PublicKey, 
        tokenProgram: PublicKey, 
        mint: PublicKey, 
    };
    
    export const deriveAccountPDA = (
        seeds: AccountSeeds,
        programId: PublicKey
    ): [PublicKey, number] => {
        return PublicKey.findProgramAddressSync(
            [
                seeds.wallet.toBuffer(),
                seeds.tokenProgram.toBuffer(),
                seeds.mint.toBuffer(),
            ],
            programId,
        )
    };
    
}

export module CslSplAssocTokenPDAs {
    export module CslSplTokenPDAs {
        export type AccountSeeds = {
            wallet: PublicKey, 
            tokenProgram: PublicKey, 
            mint: PublicKey, 
        };
        
        export const deriveAccountPDA = (
            seeds: AccountSeeds,
            programId: PublicKey
        ): [PublicKey, number] => {
            return PublicKey.findProgramAddressSync(
                [
                    seeds.wallet.toBuffer(),
                    seeds.tokenProgram.toBuffer(),
                    seeds.mint.toBuffer(),
                ],
                programId,
            )
        };
        
    }
    
}

