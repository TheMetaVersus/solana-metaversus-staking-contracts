use crate::{constants::*, error::*, states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub fn handle(ctx: Context<Withdraw>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [POOL_SEED],
        bump
    )]
    pub pool: Account<'info, TokenAccount>,

    #[account(
        seeds = [USER_STAKING_DATA_SEED, user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_data: Account<'info, UserData>,

    #[account(
        constraint = nft_token_acc.mint == nft_mint.key(),
        constraint = nft_token_acc.owner == user.key()
    )]
    pub nft_token_acc: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = mtvs_token_acc.mint == global_state.mtvs_token_mint,
        constraint = mtvs_token_acc.owner == user.key()
    )]
    pub mtvs_token_acc: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
