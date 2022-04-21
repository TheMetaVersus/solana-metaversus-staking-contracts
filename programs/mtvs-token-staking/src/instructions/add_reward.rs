use crate::{constants::*, error::*, instructions::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
#[derive(Accounts)]
pub struct RewardManagement<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

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
      init_if_needed,
      payer = admin,
      associated_token::mint = mtvs_mint,
      associated_token::authority = admin
    )]
    pub reward_token_acc: Box<Account<'info, TokenAccount>>,

    #[account(address = global_state.mtvs_token_mint)]
    pub mtvs_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> RewardManagement<'info> {
    pub fn remove_reward_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.reward_pool.to_account_info(),
                to: self.reward_token_acc.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }
    fn add_reward_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.reward_token_acc.to_account_info(),
                to: self.reward_pool.to_account_info(),
                authority: self.admin.to_account_info(),
            },
        )
    }
    pub fn validate(&self) -> Result<()> {
      require!(self.admin.key().eq(&self.global_state.authority), StakingError::NotAllowedAuthority);
      Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<RewardManagement>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts.add_reward_context(),
        amount,
    )?;
    Ok(())
}
