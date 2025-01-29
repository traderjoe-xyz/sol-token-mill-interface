use anchor_lang::prelude::*;

use crate::{
    events::TokenMillMarketFreedEvent,
    state::{Market, MARKET_AUTHORITY_PDA_SEED},
    MarketAuthority,
};

#[event_cpi]
#[derive(Accounts)]
pub struct FreeMarket<'info> {
    #[account(mut)]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [MARKET_AUTHORITY_PDA_SEED.as_bytes(), market.key().as_ref(), authority.key().as_ref()],
        bump = market_authority.bump,
    )]
    pub market_authority: Account<'info, MarketAuthority>,

    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<FreeMarket>) -> Result<()> {
    emit_cpi!(TokenMillMarketFreedEvent {
        market: ctx.accounts.market.key(),
    });

    Ok(())
}
