use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    errors::TokenMillError,
    events::TokenMillStakingRewardsClaimEvent,
    state::{Market, MarketStaking, StakePosition},
};

#[event_cpi]
#[derive(Accounts)]
pub struct StakingRewardsClaim<'info> {
    #[account(mut, has_one = quote_token_mint @ TokenMillError::InvalidMintAccount)]
    pub market: AccountLoader<'info, Market>,

    #[account(mut, has_one = market @ TokenMillError::InvalidMarket)]
    pub staking: Account<'info, MarketStaking>,

    #[account(
        mut,
        has_one = user @ TokenMillError::InvalidAuthority,
        has_one = market @ TokenMillError::InvalidMarket
    )]
    pub stake_position: Account<'info, StakePosition>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = market,
        associated_token::token_program = quote_token_program
    )]
    pub market_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = user,
        associated_token::token_program = quote_token_program
    )]
    pub user_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<StakingRewardsClaim>) -> Result<u64> {
    let pending_rewards = 0;

    emit_cpi!(TokenMillStakingRewardsClaimEvent {
        market: ctx.accounts.market.key(),
        user: ctx.accounts.user.key(),
        amount_distributed: pending_rewards,
    });

    Ok(pending_rewards)
}
