use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::*;

/// Validates token gating requirements for a recipient
pub fn validate_token_gate(
    treasury: &Account<TreasuryConfig>,
    recipient_token_account: Option<&Account<TokenAccount>>,
    recipient_address: &Pubkey,
) -> Result<()> {
    // Skip validation if token gating is not required
    if !treasury.require_token_gate {
        return Ok(());
    }

    // If token gating is required, we must have a token gate mint configured
    let token_gate_mint = treasury.token_gate_mint
        .ok_or(crate::error::ErrorCode::MissingTokenGate)?;

    // We must have the recipient's token account
    let token_account = recipient_token_account
        .ok_or(crate::error::ErrorCode::MissingTokenGate)?;

    // Verify the token account belongs to the recipient
    if token_account.owner != *recipient_address {
        return Err(crate::error::ErrorCode::MissingTokenGate.into());
    }

    // Verify the token account is for the correct mint
    if token_account.mint != token_gate_mint {
        return Err(crate::error::ErrorCode::MissingTokenGate.into());
    }

    // Verify the recipient has enough tokens
    if token_account.amount < treasury.token_gate_amount {
        return Err(crate::error::ErrorCode::MissingTokenGate.into());
    }

    Ok(())
}

/// Validates token gating when adding a new recipient
pub fn validate_token_gate_for_recipient_creation(
    treasury: &Account<TreasuryConfig>,
    recipient_token_account: Option<&Account<TokenAccount>>,
    recipient_address: &Pubkey,
) -> Result<()> {
    // If token gating is required, validate it upfront
    if treasury.require_token_gate {
        validate_token_gate(treasury, recipient_token_account, recipient_address)?;
        
        // Log successful token gate validation
        msg!(
            "Token gate validated: recipient {} has {} tokens of required {} tokens",
            recipient_address,
            recipient_token_account.unwrap().amount,
            treasury.token_gate_amount
        );
    }
    
    Ok(())
}

/// Helper function to get the expected token account address for a recipient
pub fn get_expected_token_account_address(
    recipient_address: &Pubkey,
    token_mint: &Pubkey,
    token_program_id: &Pubkey,
) -> Pubkey {
    anchor_spl::associated_token::get_associated_token_address(
        recipient_address,
        token_mint
    )
}

/// Validates that a token account is the expected associated token account
pub fn validate_associated_token_account(
    token_account: &Account<TokenAccount>,
    owner: &Pubkey,
    mint: &Pubkey,
) -> Result<()> {
    let expected_address = get_expected_token_account_address(
        owner,
        mint,
        &anchor_spl::token::ID,
    );
    
    if token_account.key() != expected_address {
        return Err(crate::error::ErrorCode::InvalidTokenVault.into());
    }
    
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use anchor_lang::prelude::*;
    
//     // Mock treasury config for testing
//     fn create_mock_treasury(require_token_gate: bool, token_gate_amount: u64) -> Account<TreasuryConfig> {
//         // This would be implemented with proper test setup
//         todo!("Implement test helpers")
//     }
    
//     #[test]
//     fn test_token_gate_validation_disabled() {
//         // Test when token gating is disabled
//         let treasury = create_mock_treasury(false, 0);
//         let result = validate_token_gate(&treasury, None, &Pubkey::new_unique());
//         assert!(result.is_ok());
//     }
    
//     #[test]
//     fn test_token_gate_validation_enabled_but_missing_account() {
//         // Test when token gating is enabled but no token account provided
//         let treasury = create_mock_treasury(true, 100);
//         let result = validate_token_gate(&treasury, None, &Pubkey::new_unique());
//         assert!(result.is_err());
//     }
// }