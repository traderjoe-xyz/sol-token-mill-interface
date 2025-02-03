use anchor_lang::prelude::*;

use crate::{
    events::TokenMillMarketFreedEvent,
    state::{Market, SWAP_AUTHORITY_BADGE_PDA_SEED},
    SwapAuthorityBadge,
};

#[event_cpi]
#[derive(Accounts)]
pub struct FreeMarket<'info> {
    #[account(mut)]
    pub market: AccountLoader<'info, Market>,

    #[account(
        mut,
        seeds = [SWAP_AUTHORITY_BADGE_PDA_SEED.as_bytes(), market.key().as_ref(), swap_authority.key().as_ref()],
        bump = swap_authority_badge.bump,
    )]
    pub swap_authority_badge: Account<'info, SwapAuthorityBadge>,

    pub swap_authority: Signer<'info>,
}

pub fn handler(ctx: Context<FreeMarket>) -> Result<()> {
    emit_cpi!(TokenMillMarketFreedEvent {
        market: ctx.accounts.market.key(),
    });

    Ok(())
}
