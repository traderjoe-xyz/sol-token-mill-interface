use anchor_lang::prelude::*;

use super::MarketSettingsUpdate;
use crate::events::TokenMillCreatorUpdateEvent;

pub fn handler(ctx: Context<MarketSettingsUpdate>, new_creator: Pubkey) -> Result<()> {
    emit_cpi!(TokenMillCreatorUpdateEvent {
        market: ctx.accounts.market.key(),
        new_creator,
    });

    Ok(())
}
