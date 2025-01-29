use anchor_lang::prelude::*;

use crate::{
    errors::TokenMillError, events::TokenMillConfigOwnershipTransferEvent, state::TokenMillConfig,
};

#[event_cpi]
#[derive(Accounts)]
pub struct AcceptConfigOwnership<'info> {
    #[account(mut, constraint = config.pending_authority == Some(pending_authority.key()) @ TokenMillError::InvalidAuthority)]
    pub config: Account<'info, TokenMillConfig>,

    pub pending_authority: Signer<'info>,
}

pub fn handler(ctx: Context<AcceptConfigOwnership>) -> Result<()> {
    emit_cpi!(TokenMillConfigOwnershipTransferEvent {
        config: ctx.accounts.config.key(),
        new_authority: ctx.accounts.pending_authority.key(),
    });

    Ok(())
}
