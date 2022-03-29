use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    // admin
    pub authority: Pubkey,
    // nft creator for verify
    pub verify_nft_creator: Pubkey,
    // token for staking
    pub mtvs_token_mint: Pubkey,
    // total staked user count
    pub total_stake_user: u64,
    // totally harvested rewards
    pub total_harvested_reward: u64,
}
