use anchor_lang::prelude::*;

use super::MarketSettingsUpdate;
use crate::events::TokenMillMarketFeeSharesUpdateEvent;

pub fn handler(
    ctx: Context<MarketSettingsUpdate>,
    new_creator_fee_share: u16,
    new_staking_fee_share: u16,
) -> Result<()> {
    emit_cpi!(TokenMillMarketFeeSharesUpdateEvent {
        market: ctx.accounts.market.key(),
        new_creator_fee_share,
        new_staking_fee_share,
    });

    Ok(())
}
