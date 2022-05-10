use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("BB2vBh1MYrAAobYv69hVEFrMCeAKFEuErHiMMf4ejcJo");
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

    pub fn add_reward(ctx: Context<RewardManagement>, amount: u64) -> Result<()> {
        add_reward::handle(ctx, amount)
    }

    pub fn remove_reward(ctx: Context<RewardManagement>, amount: u64) -> Result<()> {
        remove_reward::handle(ctx, amount)
    }

    pub fn transfer_ownership(ctx: Context<TransferOwnership>, new_admin: Pubkey) -> Result<()> {
        transfer_ownership::handle(ctx, new_admin)
    }

    pub fn change_treasury(ctx: Context<ChangeTreasury>, treasury: Pubkey) -> Result<()> {
        change_treasury::handle(ctx, treasury)
    }

    pub fn change_token_mint(ctx: Context<ChangeTokenMint>, token_mint: Pubkey) -> Result<()> {
        change_token_mint::handle(ctx, token_mint)
    }

    pub fn change_creator(ctx: Context<ChangeCreator>, creator: Pubkey) -> Result<()> {
        change_creator::handle(ctx, creator)
    }

    pub fn change_tier_setting(
        ctx: Context<ChangeTierSetting>,
        tier_max_days: [u16; 10],
        tier_percent: [u16; 10],
        available_tier: u8,
    ) -> Result<()> {
        change_tier_setting::handle(
            ctx, 
            tier_max_days,
            tier_percent,
            available_tier
        )
    }
}
