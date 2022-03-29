
use anchor_lang::prelude::*;

pub fn handle(ctx: Context<Withdraw>) -> Result<()> {        
  Ok(())
}


#[derive(Accounts)]
pub struct Withdraw<'info> {
  #[account(mut)]
  pub user: Signer<'info>
}