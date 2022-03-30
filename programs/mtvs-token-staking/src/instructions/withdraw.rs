use crate::{constants::*, error::*, states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

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

impl<'info> Withdraw<'info> {
    fn withdraw_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.mtvs_token_acc.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }
}

pub fn handle(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    let accts = ctx.accounts;

    // Update staking information in user_data
    accts.user_data.nft_mint = accts.nft_mint.key();
    accts.user_data.amount = accts.user_data.amount.checked_sub(amount).unwrap();
    accts.user_data.pending_reward = calc_pending_reward(&accts.user_data).unwrap();
    accts.user_data.last_stake_time = timestamp as u64;

    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_sub(amount)
        .unwrap();

    // transfer stake amount to pool
    let bump = ctx.bumps.get("global_state").unwrap();
    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts
            .withdraw_token_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        amount,
    )?;
    Ok(())
}
