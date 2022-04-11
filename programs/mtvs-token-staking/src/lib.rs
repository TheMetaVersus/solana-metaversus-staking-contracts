use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("8evuSu8JcKmRR96RaLNDHjT7SteBnX69A4fzR2Nb8CFv");

#[program]
pub mod mtvs_token_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        new_authority: Pubkey,
        treasury: Pubkey,
        tier_max_days: [u16; 10],
        tier_percent: [u16; 10],
        available_tier: u8,
    ) -> Result<()> {
        initialize::handle(
            ctx,
            new_authority,
            treasury,
            tier_max_days,
            tier_percent,
            available_tier,
        )
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handle(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        withdraw::handle(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        claim_reward::handle(ctx)
    }
}
