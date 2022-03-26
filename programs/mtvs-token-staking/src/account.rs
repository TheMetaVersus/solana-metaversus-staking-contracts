use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    // admin
    pub authority: Pubkey,
    // nft for verify
    pub verify_nft_mint: Pubkey,
    // token for staking
    pub mtvs_token_mint: Pubkey,
    // pool token account address
    pub pool_address: Pubkey,
}

#[account]
#[derive(Default)]
pub struct UserStakingData {
    // staker
    pub user: Pubkey,
    // staked amount
    pub amount: u64,
    // nft which is used for verify
    pub nft_mint: Pubkey,
    // rewards will be charged
    pub pending_reward: u64,
    // last stake amount change time
    pub last_stake_time: u64
}
