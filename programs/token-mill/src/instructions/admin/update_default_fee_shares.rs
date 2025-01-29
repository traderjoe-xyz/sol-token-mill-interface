use anchor_lang::prelude::*;

use super::ConfigUpdate;
use crate::events::TokenMillDefaultFeeSharesUpdateEvent;

pub fn handler(
    ctx: Context<ConfigUpdate>,
    new_default_protocol_fee_share: u16,
    new_referral_fee_share: u16,
) -> Result<()> {
    emit_cpi!(TokenMillDefaultFeeSharesUpdateEvent {
        config: ctx.accounts.config.key(),
        new_default_protocol_fee_share,
        new_referral_fee_share,
    });

    Ok(())
}
