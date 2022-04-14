use crate::constants::{ONE_DAY, REWARD_DENOMIATOR};
use crate::error::StakingError;
use crate::states::GlobalState;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserData {
    // staker
    pub user: Pubkey,
    // staked amount
    pub amount: u64,
    // nft which is used for verify
    pub nft_mint: Pubkey,
    // last claimed time
    pub last_reward_time: u64,
    // staked time
    pub staked_time: u64,
    // data seed
    pub seed_key: Pubkey,
    // totally claimed reward
    pub claimed_reward: u128,

    // reserved space
    pub reserved: [u128; 2]
}

impl UserData {
    pub fn calc_rewards(&self, global_state: &GlobalState) -> Result<u64> {
        let current_timestamp = Clock::get()?.unix_timestamp as u64;
        let stake_duration = current_timestamp.checked_sub(self.staked_time).unwrap();

        // get tier of current stake duration
        let mut tier = global_state
            .tier_max_days
            .iter()
            .position(|&x| stake_duration <= (x as u64).checked_mul(ONE_DAY).unwrap())
            .unwrap();

        // If tier is 3 and max tier is 2, then tier should be 2.
        tier = tier.min(global_state.available_tier as usize);

        let locked_days = stake_duration.checked_div(ONE_DAY).unwrap();

        // get total reward from first stake time
        let total_reward = (self.amount as u128)
            .checked_mul(global_state.tier_percent[tier] as u128)
            .unwrap()
            //.checked_mul(stake_duration as u128)
            //.unwrap()
            //.checked_div(ONE_DAY as u128)
            //.unwrap()
            .checked_mul(locked_days as u128)
            .unwrap()
            .checked_div(REWARD_DENOMIATOR as u128)
            .unwrap();
        // calculate claimable reward at the moment
        let claimable_reward = total_reward.checked_sub(self.claimed_reward).unwrap() as u64;
        Ok(claimable_reward)
    }
}
