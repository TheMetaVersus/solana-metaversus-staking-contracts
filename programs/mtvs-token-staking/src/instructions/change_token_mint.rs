use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeTokenMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [GLOBAL_STATE_SEED],
      bump,
      has_one = authority
    )]
    pub global_state: Box<Account<'info, GlobalState>>,
}

impl<'info> ChangeTokenMint<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(
    ctx: Context<ChangeTokenMint>,
    token_mint: Pubkey,
) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.mtvs_token_mint = token_mint;
    Ok(())
}
