use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{
    errors::TokenMillError,
    events::TokenMillQuoteTokenBadgeEvent,
    state::{QuoteTokenBadge, TokenMillConfig},
    QUOTE_TOKEN_BADGE_PDA_SEED,
};

#[event_cpi]
#[derive(Accounts)]
pub struct CreateQuoteAssetBadge<'info> {
    #[account(has_one = authority @ TokenMillError::InvalidAuthority)]
    pub config: Account<'info, TokenMillConfig>,

    #[account(
        init,
        seeds = [
            QUOTE_TOKEN_BADGE_PDA_SEED.as_bytes(),
            config.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + QuoteTokenBadge::INIT_SPACE
    )]
    pub quote_asset_badge: Account<'info, QuoteTokenBadge>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateQuoteAssetBadge>) -> Result<()> {
    emit_cpi!(TokenMillQuoteTokenBadgeEvent {
        config: ctx.accounts.config.key(),
        quote_token_mint: ctx.accounts.token_mint.key(),
        quote_asset_badge_status: ctx.accounts.quote_asset_badge.status,
    });

    Ok(())
}
