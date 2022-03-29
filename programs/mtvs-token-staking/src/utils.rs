use crate::states::UserData;
use anchor_lang::prelude::*;

pub fn calc_pending_reward(user_data: &UserData) -> Result<u64> {
    // TODO: reward calculation

    let rewards = user_data.pending_reward;

    Ok(0)
}
