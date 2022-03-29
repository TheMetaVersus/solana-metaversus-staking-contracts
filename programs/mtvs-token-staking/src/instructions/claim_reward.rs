use anchor_lang::prelude::*;

pub fn handle(ctx: Context<ClaimReward>) -> Result<()> {        
  Ok(())
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
  #[account(mut)]
  pub user: Signer<'info>
}