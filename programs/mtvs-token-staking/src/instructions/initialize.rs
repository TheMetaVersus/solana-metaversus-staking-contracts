use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        payer = authority,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init_if_needed,
        seeds = [POOL_SEED],
        bump,
        token::mint = mtvs_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub pool: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [REWARD_POOL_SEED],
        bump,
        token::mint = mtvs_token_mint,
        token::authority = global_state,
        payer = authority
    )]
    pub reward_pool: Account<'info, TokenAccount>,

    /// CHECK: This should a nft creator address.
    /// Can be both system account and or candy machine pda
    pub nft_creator: AccountInfo<'info>,
    pub mtvs_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn validate(&self) -> Result<()> {
        if self.global_state.is_initialized == 1 {
            require!(
                self.global_state.authority.eq(&self.authority.key()),
                StakingError::NotAllowedAuthority
            )
        }
        Ok(())
    }
}

/// Initialize Staking Program for the first time
/// to init global state with some data for validation
///
#[access_control(ctx.accounts.validate())]
pub fn handle(
    ctx: Context<Initialize>,
    new_authority: Pubkey,
    tier_max_days: [u16; 10],
    tier_percent: [u16; 10],
    available_tier: u8,
) -> Result<()> {
    let accts = ctx.accounts;
    accts.global_state.is_initialized = 1;
    accts.global_state.authority = new_authority;
    accts.global_state.verify_nft_creator = accts.nft_creator.key();
    accts.global_state.mtvs_token_mint = accts.mtvs_token_mint.key();
    accts.global_state.tier_max_days = tier_max_days;
    accts.global_state.tier_percent = tier_percent;
    accts.global_state.available_tier = available_tier;
    Ok(())
}
