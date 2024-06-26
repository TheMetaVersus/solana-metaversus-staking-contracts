use crate::{constants::*, error::*, instructions::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
      mut,
      seeds = [REWARD_POOL_SEED],
      bump
    )]
    pub reward_pool: Box<Account<'info, TokenAccount>>,

    #[account(
      mut,
      seeds = [USER_STAKING_DATA_SEED, user_data.seed_key.as_ref(), user.key().as_ref()],
      bump,
      has_one = user
    )]
    pub user_data: Box<Account<'info, UserData>>,

    #[account(
        mut,
        seeds = [USER_STATE_SEED, user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_state: Box<Account<'info, UserState>>,

    pub nft_hold: NftHold<'info>,

    #[account(
      init_if_needed,
      payer = user,
      associated_token::mint = mtvs_mint,
      associated_token::authority = user
    )]
    pub reward_token_acc: Box<Account<'info, TokenAccount>>,

    #[account(address = global_state.mtvs_token_mint)]
    pub mtvs_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> ClaimReward<'info> {
    fn claim_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.reward_pool.to_account_info(),
                to: self.reward_token_acc.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }
    fn validate(&self) -> Result<()> {
        self.nft_hold
            .validate(self.user.key(), self.global_state.verify_nft_creator)?;
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<ClaimReward>) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    let accts = ctx.accounts;

    // reward to claim now
    let reward_to_claim = accts.user_data.calc_rewards(&accts.global_state).unwrap();

    // update user data: make pending reward as 0 and update changeTime
    accts.user_data.claimed_reward = accts
        .user_data
        .claimed_reward
        .checked_add(reward_to_claim as u128)
        .unwrap();
    accts.user_data.last_reward_time = timestamp as u64;

    // update total harvested reward in user state
    accts.user_state.total_claimed_reward = accts
        .user_state
        .total_claimed_reward
        .checked_add(reward_to_claim as u128)
        .unwrap();

    // update total harvested reward in global state
    accts.global_state.total_claimed_reward = accts
        .global_state
        .total_claimed_reward
        .checked_add(reward_to_claim as u128)
        .unwrap();

    // transfer reward from pool to user
    let bump = ctx.bumps.get("global_state").unwrap();
    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts
            .claim_token_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        reward_to_claim,
    )?;
    Ok(())
}
