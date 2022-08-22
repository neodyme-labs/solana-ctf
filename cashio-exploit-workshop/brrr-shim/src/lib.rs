//! We need this shim because Anchor macro works weird with nested struct (BrrrCommon)
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use arrow_sunny::Arrow;
use bankman::{Bank, Collateral};
use stable_swap_anchor::SwapInfo;

declare_id!("BRRRot6ig147TBU6EGp7TMesmQrwu729CbG6qu2ZUHWm");

#[program]
pub mod brrr {
    use super::*;

    pub fn foo(_ctx: Context<PrintCash>) -> Result<()> {
        Ok(())
    }

    pub fn bar(_ctx: Context<BurnCash>) -> Result<()> {
        Ok(())
    }

    pub fn baz(_ctx: Context<BrrrCommon>) -> Result<()> {
        Ok(())
    }
}

/// Accounts related to the Saber pool.
#[derive(Accounts)]
pub struct SaberSwapAccounts<'info> {
    /// The [Arrow] used as collateral.
    pub arrow: Box<Account<'info, Arrow>>,
    /// The Saber [SwapInfo] of the collateral.
    pub saber_swap: Box<Account<'info, SwapInfo>>,
    /// Mint of the pool.
    pub pool_mint: Box<Account<'info, Mint>>,
    /// Reserve of token A.
    pub reserve_a: Box<Account<'info, TokenAccount>>,
    /// Reserve of token B.
    pub reserve_b: Box<Account<'info, TokenAccount>>,
}

/// Accounts for printing $CASH.
#[derive(Accounts)]
pub struct PrintCash<'info> {
    /// Common accounts.
    pub common: BrrrCommon<'info>,

    /// The depositor into the pool.
    #[account(mut)]
    pub depositor: Signer<'info>,

    /// The source of the deposited [Collateral] tokens.
    #[account(mut)]
    pub depositor_source: Box<Account<'info, TokenAccount>>,

    /// Destination of the issued $CASH.
    #[account(mut)]
    pub mint_destination: Box<Account<'info, TokenAccount>>,

    /// The [ISSUE_AUTHORITY_ADDRESS].
    /// CHECK: this is handled by Vipers.
    pub issue_authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct BrrrCommon<'info> {
    /// Information about the bank.
    pub bank: Box<Account<'info, Bank>>,

    /// The [Collateral].
    pub collateral: Box<Account<'info, Collateral>>,

    /// Information about the crate.
    pub crate_token: Box<Account<'info, crate_token::CrateToken>>,

    /// [Mint] of the [crate_token::CrateToken].
    #[account(mut)]
    pub crate_mint: Box<Account<'info, Mint>>,

    /// [TokenAccount] holding the [Collateral] tokens of the [crate_token::CrateToken].
    #[account(mut)]
    pub crate_collateral_tokens: Box<Account<'info, TokenAccount>>,

    /// Saber swap accounts.
    pub saber_swap: SaberSwapAccounts<'info>,

    /// [Token] program.
    pub token_program: Program<'info, Token>,

    /// [crate_token::program::CrateToken] program.
    pub crate_token_program: Program<'info, crate_token::program::CrateToken>,
}

/// Accounts for burning $CASH.
#[derive(Accounts)]
pub struct BurnCash<'info> {
    /// Common accounts.
    pub common: BrrrCommon<'info>,

    /// The depositor into the pool.
    #[account(mut)]
    pub burner: Signer<'info>,

    /// The source of the burned $CASH.
    #[account(mut)]
    pub burned_cash_source: Box<Account<'info, TokenAccount>>,

    /// Destination of the issued tokens.
    #[account(mut)]
    pub withdraw_destination: Box<Account<'info, TokenAccount>>,

    /// Author fee token destination
    #[account(mut)]
    pub author_fee_destination: Account<'info, TokenAccount>,

    /// Protocol fee token destination
    #[account(mut)]
    pub protocol_fee_destination: Account<'info, TokenAccount>,

    /// The [WITHDRAW_AUTHORITY_ADDRESS].
    /// CHECK: this is handled by Vipers.
    pub withdraw_authority: UncheckedAccount<'info>,
}
