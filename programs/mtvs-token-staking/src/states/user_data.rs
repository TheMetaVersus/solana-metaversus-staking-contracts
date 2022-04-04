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
    // last stake amount change time
    pub last_reward_time: u64,
    // data seed
    pub seed_key: Pubkey,
    // totally claimed reward
    pub claimed_reward: u64,
}

impl UserData {
    pub fn calc_rewards(&self) -> Result<u64> {

        Ok(0)
    }
}
