use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
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

impl<'info> TransferOwnership<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(
    ctx: Context<TransferOwnership>,
    new_authority: Pubkey,
) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.authority = new_authority;
    Ok(())
}
