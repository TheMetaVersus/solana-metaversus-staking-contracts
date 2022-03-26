use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

pub mod account;
pub mod constants;
pub mod error;
pub mod utils;

use account::*;
use constants::*;
use error::*;
use utils::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod mtvs_token_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let accts = ctx.accounts;
        accts.global_state.authority = accts.authority.key();
        accts.global_state.verify_nft_mint = accts.verify_nft_mint.key();
        accts.global_state.mtvs_token_mint = accts.mtvs_token_mint.key();
        accts.global_state.pool_address = accts.pool_token_account.key();
        Ok(())
    }

    pub fn initialize_staking_data(ctx: Context<InitStakingData>) -> Result<()> {
        let accts = ctx.accounts;
        accts.user_staking_data.user = accts.user.key();
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        
        // TODO: first verify MTVS NFT
        let accts = ctx.accounts;
        Ok(())
    }

    pub fn unstake(ctx: Context<Stake>) -> Result<()> {
        let accts = ctx.accounts;
        Ok(())
    }

    pub fn harvest(ctx: Context<Stake>) -> Result<()> {
        let accts = ctx.accounts;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        payer = authority,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        seeds = [POOL_TOKEN_ACCOUNT_SEED],
        bump,
        token::mint = mtvs_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub pool_token_account: Account<'info, TokenAccount>,

    pub verify_nft_mint: Account<'info, Mint>,
    pub mtvs_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitStakingData<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [USER_STAKING_DATA_SEED, user.key().as_ref()],
        bump,
        payer = user
    )]
    pub user_staking_data: Account<'info, UserStakingData>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Only NFT holders can stake mtvs tokens
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    // TODO: seed check, authority check, account check
    /// CHECK:Should be checked
    #[account(mut)]
    pub pool: AccountInfo<'info>,

    #[account(
        seeds = [USER_STAKING_DATA_SEED, user.key().as_ref()],
        bump
    )]
    pub user_staking_data: Account<'info, UserStakingData>,

    // Verify NFT ownership
    // TODO: Add token amount check
    #[account(
        associated_token::mint = verify_nft_mint,
        associated_token::authority = user
    )]
    pub verify_nft_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mtvs_token_mint,
        associated_token::authority = user
    )]
    pub mtvs_ata: Account<'info, TokenAccount>,

    #[account(address = global_state.verify_nft_mint)]
    pub verify_nft_mint: Account<'info, Mint>,

    #[account(address = global_state.mtvs_token_mint)]
    pub mtvs_token_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}




