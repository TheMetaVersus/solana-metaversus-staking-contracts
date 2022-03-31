use crate::{constants::*, error::*, instructions::*, states::*, utils::*};
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
    pub global_state: Account<'info, GlobalState>,

    #[account(
      mut,
      seeds = [REWARD_POOL_SEED],
      bump
    )]
    pub reward_pool: Account<'info, TokenAccount>,

    #[account(
      mut,
      seeds = [USER_STAKING_DATA_SEED, user.key().as_ref()],
      bump,
      has_one = user
    )]
    pub user_data: Account<'info, UserData>,

    pub nft_hold: NftHold<'info>,

    #[account(
      init_if_needed,
      payer = user,
      associated_token::mint = mtvs_mint,
      associated_token::authority = user
    )]
    pub reward_token_acc: Account<'info, TokenAccount>,

    #[account(address = global_state.mtvs_token_mint)]
    pub mtvs_mint: Account<'info, Mint>,

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

    // update user data: make pending reward as 0 and update changeTime
    accts.user_data.pending_reward = 0;
    accts.user_data.last_reward_time = timestamp as u64;

    // reward to claim now
    let reward_to_claim = calc_pending_reward(&accts.user_data).unwrap();

    // update total harvested reward in global state
    accts.global_state.total_harvested_reward = accts
        .global_state
        .total_harvested_reward
        .checked_add(reward_to_claim)
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
