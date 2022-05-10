use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeTierSetting<'info> {
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

impl<'info> ChangeTierSetting<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(
    ctx: Context<ChangeTierSetting>,
    tier_max_days: [u16; 10],
    tier_percent: [u16; 10],
    available_tier: u8,
) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.tier_max_days = tier_max_days;
    accts.global_state.tier_percent = tier_percent;
    accts.global_state.available_tier = available_tier;
    Ok(())
}
