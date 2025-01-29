use anchor_lang::prelude::*;

use crate::{
    constant::PRICES_LENGTH, errors::TokenMillError, events::TokenMillMarketPriceSetEvent,
    state::Market,
};

#[event_cpi]
#[derive(Accounts)]
pub struct MarketSettingsUpdate<'info> {
    #[account(mut, has_one = creator @ TokenMillError::InvalidAuthority)]
    pub market: AccountLoader<'info, Market>,

    pub creator: Signer<'info>,
}

pub fn handler(
    ctx: Context<MarketSettingsUpdate>,
    bid_prices: [u64; PRICES_LENGTH],
    ask_prices: [u64; PRICES_LENGTH],
) -> Result<()> {
    emit_cpi!(TokenMillMarketPriceSetEvent {
        market: ctx.accounts.market.key(),
        bid_prices,
        ask_prices,
    });

    Ok(())
}
