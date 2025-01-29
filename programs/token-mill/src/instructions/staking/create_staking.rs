use anchor_lang::prelude::*;

use crate::{
    state::{Market, MarketStaking},
    MARKET_STAKING_PDA_SEED,
};

#[derive(Accounts)]
pub struct CreateStaking<'info> {
    pub market: AccountLoader<'info, Market>,

    #[account(
        init,
        payer = payer,
        space = 8 + MarketStaking::INIT_SPACE,
        seeds = [MARKET_STAKING_PDA_SEED.as_bytes(), market.key().as_ref()],
        bump
    )]
    pub staking: Account<'info, MarketStaking>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<CreateStaking>) -> Result<()> {
    Ok(())
}
