use crate::{constants::*, error::*, instructions::*, states::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
/// UserData Account will be closed when user withdraws tokens.
/// All lamports will go to super_authority wallet
/// In withdraw function, there is no claim part.
/// so Claim Instruction should be prior to Withdraw instruction
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, address = global_state.authority)]
    pub super_authority: SystemAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [POOL_SEED],
        bump
    )]
    pub pool: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [USER_STAKING_DATA_SEED, user_data.seed_key.as_ref(), user.key().as_ref()],
        bump,
        has_one = user,
        close = super_authority
    )]
    pub user_data: Account<'info, UserData>,

    pub nft_hold: NftHold<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mtvs_mint,
        associated_token::authority = user
    )]
    pub user_mtvs_ata: Account<'info, TokenAccount>,

    #[account(address = global_state.mtvs_token_mint)]
    pub mtvs_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Withdraw<'info> {
    fn withdraw_token_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.pool.to_account_info(),
                to: self.user_mtvs_ata.to_account_info(),
                authority: self.global_state.to_account_info(),
            },
        )
    }
    fn validate(&self) -> Result<()> {
        self.nft_hold
            .validate(self.user.key(), self.global_state.verify_nft_creator)?;
        Ok(())
    }
}

#[access_control(ctx.accounts.validate())]
pub fn handle(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp;

    let accts = ctx.accounts;

    // Update totally staked amount in global_state
    accts.global_state.total_staked_amount = accts
        .global_state
        .total_staked_amount
        .checked_sub(amount)
        .unwrap();

    // Update card count
    accts.global_state.total_stake_card -= 1;

    // transfer stake amount to pool
    let bump = ctx.bumps.get("global_state").unwrap();

    // global_state is owner of pool account, so it's seeds should be signer
    token::transfer(
        accts
            .withdraw_token_context()
            .with_signer(&[&[GLOBAL_STATE_SEED.as_ref(), &[*bump]]]),
        amount,
    )?;
    Ok(())
}
