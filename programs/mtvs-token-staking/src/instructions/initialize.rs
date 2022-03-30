use crate::{constants::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
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
        seeds = [POOL_SEED],
        bump,
        token::mint = mtvs_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub pool: Account<'info, TokenAccount>,

    /// CHECK: This should a nft creator address.
    /// Can be both system account and or candy machine pda
    pub nft_creator: AccountInfo<'info>,
    pub mtvs_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
pub fn handle(ctx: Context<Initialize>) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.authority = accts.authority.key();
    accts.global_state.verify_nft_creator = accts.nft_creator.key();
    accts.global_state.mtvs_token_mint = accts.mtvs_token_mint.key();
    Ok(())
}