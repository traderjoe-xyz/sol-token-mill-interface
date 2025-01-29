use anchor_lang::prelude::*;

use super::StakeUpdate;
use crate::events::TokenMillStakingWithdrawalEvent;

pub fn handler(ctx: Context<StakeUpdate>, amount: u64) -> Result<()> {
    emit_cpi!(TokenMillStakingWithdrawalEvent {
        market: ctx.accounts.market.key(),
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}
