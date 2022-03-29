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
    // rewards will be charged
    pub pending_reward: u64,
    // last stake amount change time
    pub last_stake_time: u64,
}
