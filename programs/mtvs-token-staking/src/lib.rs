use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;
pub mod utils;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mtvs_token_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handle(ctx)
    }

    pub fn init_user_data(ctx: Context<InitUserData>) -> Result<()> {
        init_user_data::handle(ctx)
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
