use anchor_lang::prelude::*;

use crate::{
    errors::TokenMillError, events::TokenMillMarketLockedEvent, state::Market, SwapAuthorityBadge,
    SWAP_AUTHORITY_BADGE_PDA_SEED,
};

#[event_cpi]
#[derive(Accounts)]
#[instruction(authority: Pubkey)]
pub struct LockMarket<'info> {
    #[account(mut, has_one = creator @ TokenMillError::InvalidAuthority)]
    pub market: AccountLoader<'info, Market>,

    #[account(
        init,
        seeds = [SWAP_AUTHORITY_BADGE_PDA_SEED.as_bytes(), market.key().as_ref(), authority.as_ref()],
        bump,
        payer = creator,
        space = 8 + SwapAuthorityBadge::INIT_SPACE
    )]
    pub swap_authority_badge: Account<'info, SwapAuthorityBadge>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// A market can only be locked upon creation, when the circulating supply is still 0
pub fn handler(ctx: Context<LockMarket>, _authority: Pubkey) -> Result<()> {
    emit_cpi!(TokenMillMarketLockedEvent {
        market: ctx.accounts.market.key(),
        swap_authority: ctx.accounts.swap_authority_badge.key(),
    });

    Ok(())
}
