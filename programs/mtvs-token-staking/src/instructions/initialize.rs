use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::{
  states::*,
  constants::*
};

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.authority = accts.authority.key();
    accts.global_state.verify_nft_mint = accts.verify_nft_mint.key();
    accts.global_state.mtvs_token_mint = accts.mtvs_token_mint.key();
    accts.global_state.pool_address = accts.pool_token_account.key();
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        payer = authority,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        seeds = [POOL_TOKEN_ACCOUNT_SEED],
        bump,
        token::mint = mtvs_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub verify_nft_mint: Account<'info, Mint>,
    pub mtvs_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}