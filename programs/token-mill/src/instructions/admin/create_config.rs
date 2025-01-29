use anchor_lang::prelude::*;

use crate::{events::TokenMillConfigCreationEvent, state::TokenMillConfig};

#[event_cpi]
#[derive(Accounts)]
pub struct CreateConfig<'info> {
    #[account(init, payer = payer, space = 8 + TokenMillConfig::INIT_SPACE)]
    pub config: Account<'info, TokenMillConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateConfig>,
    authority: Pubkey,
    _protocol_fee_recipient: Pubkey,
    protocol_fee_share: u16,
    referral_fee_share: u16,
) -> Result<()> {
    emit_cpi!(TokenMillConfigCreationEvent {
        config: ctx.accounts.config.key(),
        authority,
        default_protocol_fee_share: protocol_fee_share,
        referral_fee_share,
    });

    Ok(())
}
