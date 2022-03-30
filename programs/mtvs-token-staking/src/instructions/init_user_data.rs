use crate::{constants::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct InitUserData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        seeds = [USER_STAKING_DATA_SEED, user.key().as_ref()],
        bump,
        payer = user
    )]
    pub user_data: Account<'info, UserData>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Initialize User's Staking Data Account
/// to save user's staking information.
/// This should be unique per user
pub fn handle(ctx: Context<InitUserData>) -> Result<()> {
    let accts = ctx.accounts;
    accts.user_data.user = accts.user.key();
    accts.global_state.total_stake_user += 1;
    Ok(())
}
