// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.
//
// Docs: https://docs.codigo.ai/c%C3%B3digo-interface-description-language/specification#errors

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The signer is not authorized to perform this action")]
    UnauthorizedAccess,
    #[msg("The recipient is not valid or not whitelisted")]
    InvalidRecipient,
    #[msg("The recipient is inactive")]
    InactiveRecipient,
    #[msg("The treasury has insufficient funds for this operation")]
    InsufficientFunds,
    #[msg("This operation would exceed the spending limit for the current epoch")]
    SpendingLimitExceeded,
    #[msg("The payout schedule is invalid or not ready for execution")]
    InvalidSchedule,
    #[msg("The recipient does not meet the token-gating requirements")]
    MissingTokenGate,
    #[msg("The token vault is invalid or not initialized")]
    InvalidTokenVault,
    #[msg("The scheduled payout is not due yet")]
    PayoutNotDue,
    #[msg("The maximum number of executions for this schedule has been reached")]
    MaxExecutionsReached,
    #[msg("The streaming schedule is invalid or parameters are incorrect")]
    InvalidStreamingSchedule,
    #[msg("The stream is not active or has been fully withdrawn")]
    StreamInactive,
    #[msg("No withdrawable amount available from stream yet")]
    NoWithdrawableAmount,
    #[msg("Stream cannot be cancelled")]
    StreamNotCancelable,
}