use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{
    errors::TokenMillError,
    events::TokenMillQuoteTokenBadgeEvent,
    state::{QuoteTokenBadge, QuoteTokenBadgeStatus, TokenMillConfig},
    QUOTE_TOKEN_BADGE_PDA_SEED,
};

#[event_cpi]
#[derive(Accounts)]
pub struct UpdateQuoteAssetBadge<'info> {
    #[account(has_one = authority @ TokenMillError::InvalidAuthority)]
    pub config: Account<'info, TokenMillConfig>,

    #[account(
        mut,
        seeds = [
            QUOTE_TOKEN_BADGE_PDA_SEED.as_bytes(),
            config.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump = quote_asset_badge.bump,
    )]
    pub quote_asset_badge: Account<'info, QuoteTokenBadge>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateQuoteAssetBadge>, status: QuoteTokenBadgeStatus) -> Result<()> {
    emit_cpi!(TokenMillQuoteTokenBadgeEvent {
        config: ctx.accounts.config.key(),
        quote_token_mint: ctx.accounts.token_mint.key(),
        quote_asset_badge_status: status,
    });

    Ok(())
}
