use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mtvs_token_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        new_authority: Pubkey,
        tier_max_days: [u16; 10],
        tier_percent: [u16; 10],
        available_tier: u8,
    ) -> Result<()> {
        initialize::handle(
            ctx,
            new_authority,
            tier_max_days,
            tier_percent,
            available_tier,
        )
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handle(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::handle(ctx, amount)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        claim_reward::handle(ctx)
    }
}
