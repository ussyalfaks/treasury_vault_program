use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(
    new_admin: Option<Pubkey>,
    new_treasurer: Option<Pubkey>,
    description: Option<String>,
    daily_limit: Option<u64>,
    weekly_limit: Option<u64>,
    monthly_limit: Option<u64>,
    require_token_gate: Option<bool>,
    token_gate_mint: Option<Pubkey>,
    token_gate_amount: Option<u64>,
    treasury_seed_name: String,
)]
pub struct UpdateTreasuryConfig<'info> {
    #[account(
        mut,
        seeds = [
            b"treasury",
            treasury_seed_name.as_bytes().as_ref(),
        ],
        bump
    )]
    pub treasury: Account<'info, TreasuryConfig>,

    pub admin: Signer<'info>,
}

/// Accounts:
/// 0. `[writable]` treasury: [TreasuryConfig] 
/// 1. `[signer]` admin: [AccountInfo] 
///
/// Data:
/// - new_admin: [Option<Pubkey>] Optional new admin authority
/// - new_treasurer: [Option<Pubkey>] Optional new treasurer authority
/// - description: [Option<String>] type
/// - daily_limit: [Option<u64>] Optional new daily limit
/// - weekly_limit: [Option<u64>] Optional new weekly limit
/// - monthly_limit: [Option<u64>] Optional new monthly limit
/// - require_token_gate: [Option<bool>] Optional update to token gating requirement
/// - token_gate_mint: [Option<Pubkey>] Optional new token gate mint
/// - token_gate_amount: [Option<u64>] Optional new token gate amount
/// - treasury_seed_name: [String] Auto-generated, from the input "treasury" for the its seed definition "Treasury", sets the seed named "name"
pub fn handler(
    ctx: Context<UpdateTreasuryConfig>,
    new_admin: Option<Pubkey>,
    new_treasurer: Option<Pubkey>,
    description: Option<String>,
    daily_limit: Option<u64>,
    weekly_limit: Option<u64>,
    monthly_limit: Option<u64>,
    require_token_gate: Option<bool>,
    token_gate_mint: Option<Pubkey>,
    token_gate_amount: Option<u64>,
) -> Result<()> {
    // Verify the signer is the admin
    let treasury = &mut ctx.accounts.treasury;
    if ctx.accounts.admin.key() != treasury.admin {
        return Err(error::ErrorCode::UnauthorizedAccess.into());
    }
    
    // Update treasury config with new values if provided
    if let Some(admin) = new_admin {
        treasury.admin = admin;
    }
    
    if let Some(treasurer) = new_treasurer {
        treasury.treasurer = treasurer;
    }
    
    if let Some(desc) = description {
        treasury.description = desc;
    }
    
    if let Some(limit) = daily_limit {
        treasury.daily_limit = limit;
    }
    
    if let Some(limit) = weekly_limit {
        treasury.weekly_limit = limit;
    }
    
    if let Some(limit) = monthly_limit {
        treasury.monthly_limit = limit;
    }
    
    if let Some(require) = require_token_gate {
        treasury.require_token_gate = require;
    }
    
    if let Some(mint) = token_gate_mint {
        treasury.token_gate_mint = Some(mint);
    }
    
    if let Some(amount) = token_gate_amount {
        treasury.token_gate_amount = amount;
    }
    
    Ok(())
}