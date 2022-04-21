use crate::{constants::*, error::*, instructions::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
  token::{self},
};
use crate::add_reward::RewardManagement;

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<RewardManagement>, amount: u64) -> Result<()> {
    let accts = ctx.accounts;
    // transfer reward from pool to user
    let bump = ctx.bumps.get("global_state").unwrap();
    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts
            .remove_reward_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        amount,
    )?;
    Ok(())
}
