use anchor_lang::prelude::*;

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

