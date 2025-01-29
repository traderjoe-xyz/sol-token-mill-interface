use anchor_lang::prelude::*;

use crate::{
    errors::TokenMillError, events::TokenMillMarketLockedEvent, state::Market, MarketAuthority,
    MARKET_AUTHORITY_PDA_SEED,
};

#[event_cpi]
#[derive(Accounts)]
#[instruction(authority: Pubkey)]
pub struct LockMarket<'info> {
    #[account(mut, has_one = creator @ TokenMillError::InvalidAuthority)]
    pub market: AccountLoader<'info, Market>,

    #[account(
        init,
        seeds = [MARKET_AUTHORITY_PDA_SEED.as_bytes(), market.key().as_ref(), authority.as_ref()],
        bump,
        payer = creator,
        space = 8 + MarketAuthority::INIT_SPACE
    )]
    pub market_authority: Account<'info, MarketAuthority>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// A market can only be locked upon creation, when the circulating supply is still 0
pub fn handler(ctx: Context<LockMarket>, _authority: Pubkey) -> Result<()> {
    emit_cpi!(TokenMillMarketLockedEvent {
        market: ctx.accounts.market.key(),
        authority: ctx.accounts.market_authority.key(),
    });

    Ok(())
}
