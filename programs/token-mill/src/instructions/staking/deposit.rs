use crate::{
    errors::TokenMillError,
    events::TokenMillStakingDepositEvent,
    state::{Market, MarketStaking, StakePosition},
};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[event_cpi]
#[derive(Accounts)]
pub struct StakeUpdate<'info> {
    #[account(mut, has_one = base_token_mint @ TokenMillError::InvalidMintAccount)]
    pub market: AccountLoader<'info, Market>,

    #[account(mut, has_one = market @ TokenMillError::InvalidMarket)]
    pub staking: Account<'info, MarketStaking>,

    #[account(
        mut,
        has_one = market @ TokenMillError::InvalidMarket,
        has_one = user @ TokenMillError::InvalidAuthority
    )]
    pub stake_position: Account<'info, StakePosition>,

    pub base_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = base_token_mint,
        associated_token::authority = market,
        associated_token::token_program = base_token_program
    )]
    pub market_base_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = base_token_mint,
        associated_token::authority = user,
        associated_token::token_program = base_token_program
    )]
    pub user_base_token_ata: InterfaceAccount<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub base_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<StakeUpdate>, amount: u64) -> Result<()> {
    emit_cpi!(TokenMillStakingDepositEvent {
        market: ctx.accounts.market.key(),
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}
