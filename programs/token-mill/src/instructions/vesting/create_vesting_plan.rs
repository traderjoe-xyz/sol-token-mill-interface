use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    errors::TokenMillError,
    events::TokenMillVestingPlanCreationEvent,
    state::{Market, MarketStaking, StakePosition},
    VestingPlan,
};

#[event_cpi]
#[derive(Accounts)]
pub struct CreateVestingPlan<'info> {
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

    #[account(init, payer = user, space = 8 + VestingPlan::INIT_SPACE)]
    pub vesting_plan: Account<'info, VestingPlan>,

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

    #[account(mut)]
    pub user: Signer<'info>,

    pub base_token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateVestingPlan>,
    start: i64,
    vesting_amount: u64,
    vesting_duration: i64,
    cliff_duration: i64,
) -> Result<()> {
    emit_cpi!(TokenMillVestingPlanCreationEvent {
        market: ctx.accounts.market.key(),
        user: ctx.accounts.user.key(),
        vesting_plan: ctx.accounts.vesting_plan.key(),
        vesting_amount,
        start,
        vesting_duration,
        cliff_duration,
    });

    Ok(())
}
