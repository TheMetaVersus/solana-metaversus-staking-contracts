use crate::{constants::*, error::*, states::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use spl_token_metadata::{state::Metadata, ID as MetaProgramID};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        mut,
        seeds = [POOL_SEED],
        bump
    )]
    pub pool: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [USER_STAKING_DATA_SEED, user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub user_data: Box<Account<'info, UserData>>,

    #[account(
        constraint = nft_token_acc.mint == nft_mint.key(),
        constraint = nft_token_acc.owner == user.key()
    )]
    pub nft_token_acc: Box<Account<'info, TokenAccount>>,
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(owner = MetaProgramID)]
    /// CHECK: account check is in context validation
    pub nft_metadata: AccountInfo<'info>,

    #[account(
        mut,
        constraint = mtvs_token_acc.mint == global_state.mtvs_token_mint,
        constraint = mtvs_token_acc.owner == user.key()
    )]
    pub mtvs_token_acc: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    fn stake_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.mtvs_token_acc.to_account_info(),
                to: self.pool.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    // validate NFT Collection and NFT ownership from metadata account
    pub fn validate(&self) -> Result<()> {
        // Verify if user holds NFT
        require!(self.nft_token_acc.amount == 1, StakingError::NotNFTHolder);

        // Verify Metadata Account Key
        let (metadata_key, _) = Pubkey::find_program_address(
            &[
                b"metadata".as_ref(),
                MetaProgramID.as_ref(),
                self.nft_mint.key().as_ref(),
            ],
            &MetaProgramID,
        );
        require!(
            metadata_key.eq(&self.nft_metadata.key()),
            StakingError::IncorrectMetadata
        );
        // Metadata of NFT
        let nft_meta: Metadata = Metadata::from_account_info(&self.nft_metadata)?;
        // Check mint key in metadata
        require!(
            nft_meta.mint.eq(&self.nft_mint.key()),
            StakingError::IncorrectMetadata
        );
        // Check update authority - NFT Collection
        require!(
            nft_meta
                .update_authority
                .eq(&self.global_state.verify_nft_creator),
            StakingError::IncorrectMetadata
        );
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    let accts = ctx.accounts;

    // Update staking information in user_data
    accts.user_data.nft_mint = accts.nft_mint.key();
    accts.user_data.amount = accts.user_data.amount.checked_add(amount).unwrap();
    accts.user_data.pending_reward = calc_pending_reward(&accts.user_data).unwrap();
    accts.user_data.last_reward_time = timestamp as u64;

    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_add(amount)
        .unwrap();
    // transfer stake amount to pool
    token::transfer(accts.stake_token_context(), amount)?;

    Ok(())
}
