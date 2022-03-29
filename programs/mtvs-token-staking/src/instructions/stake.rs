use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::{
    states::*,
    constants::*,
    error::*
};

pub fn handle(ctx: Context<Stake>, amount: u64) -> Result<()> {        
  // TODO: first verify MTVS NFT
  let accts = ctx.accounts;
  Ok(())
}

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
    pub user_staking_data: Account<'info, UserData>,

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

impl<'info> Stake<'info> {
    pub fn validate(&self) -> Result<()> {
        // Verify if user holds NFT 
        require!(self.verify_nft_ata.amount == 1, StakingError::NotNFTHolder);
        Ok(())
    }
}