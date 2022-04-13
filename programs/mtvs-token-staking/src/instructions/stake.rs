use crate::{constants::*, error::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use spl_token_metadata::{state::Metadata, ID as MetadataProgramID};

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
        init,
        seeds = [USER_STAKING_DATA_SEED, data_seed.key().as_ref(), user.key().as_ref()],
        bump,
        payer = user
    )]
    pub user_data: Box<Account<'info, UserData>>,

    /// CHECK: This is a random keypair for generating user_data
    pub data_seed: AccountInfo<'info>,

    pub nft_hold: NftHold<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mtvs_mint,
        associated_token::authority = user
    )]
    pub user_mtvs_ata: Box<Account<'info, TokenAccount>>,

    #[account(address = global_state.mtvs_token_mint)]
    pub mtvs_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct NftHold<'info> {
    #[account(
        constraint = nft_token_acc.mint == nft_mint.key()
    )]
    pub nft_token_acc: Box<Account<'info, TokenAccount>>,
    pub nft_mint: Box<Account<'info, Mint>>,

    #[account(owner = MetadataProgramID)]
    /// CHECK: account check is in context validation
    pub nft_metadata: AccountInfo<'info>,
}

impl<'info> NftHold<'info> {
    pub fn validate(&self, owner: Pubkey, creator: Pubkey) -> Result<()> {
        // Verify if user holds NFT
        require!(
            self.nft_token_acc.owner.eq(&owner) && self.nft_token_acc.amount == 1,
            StakingError::NotNFTHolder
        );

        // Verify Metadata Account Key
        let (metadata_key, _) = Pubkey::find_program_address(
            &[
                b"metadata".as_ref(),
                MetadataProgramID.as_ref(),
                self.nft_mint.key().as_ref(),
            ],
            &MetadataProgramID,
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

        /*// Check update authority - NFT Collection
        require!(
            nft_meta.update_authority.eq(&creator),
            StakingError::IncorrectMetadata
        );*/

        // check verified creator in creators list
        let creators = nft_meta.data.creators.unwrap();
        let verified_creator = creators.iter().find(|&c| c.verified == true);
        if verified_creator.is_none() {
            return Err(error!(StakingError::IncorrectMetadata));
        }
        require!(
            verified_creator.unwrap().address.eq(&creator),
            StakingError::IncorrectMetadata
        );

        Ok(())
    }
}
impl<'info> Stake<'info> {
    fn stake_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.user_mtvs_ata.to_account_info(),
                to: self.pool.to_account_info(),
                authority: self.user.to_account_info(),
            },
        )
    }

    // validate NFT Collection and NFT ownership from metadata account
    pub fn validate(&self, amount: u64) -> Result<()> {
        // check minimum amount
        let deposit_minum = DEPOSIT_MINIMUM_AMOUNT
            .checked_mul(10u64.checked_pow(self.mtvs_mint.decimals as u32).unwrap())
            .unwrap();
        require!(amount >= deposit_minum, StakingError::InsufficientAmount);
        // check nft holding
        self.nft_hold
            .validate(self.user.key(), self.global_state.verify_nft_creator)?;
        Ok(())
    }
}

#[access_control(ctx.accounts.validate(amount))]
pub fn handle(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    let accts = ctx.accounts;

    // Init staking information in user_data
    accts.user_data.user = accts.user.key();
    accts.user_data.nft_mint = accts.nft_hold.nft_mint.key();
    accts.user_data.amount = amount;
    accts.user_data.staked_time = timestamp as u64;
    accts.user_data.last_reward_time = timestamp as u64;
    accts.user_data.seed_key = accts.data_seed.key();

    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_add(amount)
        .unwrap();
    // Update totally staked card in global_state
    accts.global_state.total_stake_card += 1;
    // transfer stake amount to pool
    token::transfer(accts.stake_token_context(), amount)?;

    Ok(())
}
