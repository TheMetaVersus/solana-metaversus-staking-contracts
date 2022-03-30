use crate::{constants::*, error::*, states::*, utils::*};
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
      init_if_needed,
      payer = user,
      associated_token::mint = mtvs_mint,
      associated_token::authority = user
    )]
    pub mtvs_token_acc: Account<'info, TokenAccount>,

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
              from: self.pool.to_account_info(),
              to: self.mtvs_token_acc.to_account_info(),
              authority: self.global_state.to_account_info(),
          },
      )
  }
}

pub fn handle(ctx: Context<ClaimReward>) -> Result<()> {
  let timestamp = Clock::get()?.unix_timestamp;

  let accts = ctx.accounts;

  accts.user_data.pending_reward = 0;
  let reward_to_claim = calc_pending_reward(&accts.user_data).unwrap();

  
  Ok(())
}

