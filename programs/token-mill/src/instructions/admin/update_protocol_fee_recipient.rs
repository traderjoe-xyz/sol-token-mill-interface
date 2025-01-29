use anchor_lang::prelude::*;

use super::ConfigUpdate;
use crate::events::TokenMillProtocolFeeRecipientUpdateEvent;

pub fn handler(ctx: Context<ConfigUpdate>, new_protocol_fee_recipient: Pubkey) -> Result<()> {
    emit_cpi!(TokenMillProtocolFeeRecipientUpdateEvent {
        config: ctx.accounts.config.key(),
        new_protocol_fee_recipient,
    });

    Ok(())
}
