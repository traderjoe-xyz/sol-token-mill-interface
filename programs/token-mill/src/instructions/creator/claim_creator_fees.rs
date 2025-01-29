use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{errors::TokenMillError, events::TokenMillCreatorFeeClaimEvent, state::Market};

#[event_cpi]
#[derive(Accounts)]
pub struct ClaimCreatorFees<'info> {
    #[account(
        mut,
        has_one = creator @ TokenMillError::InvalidAuthority,
        has_one = quote_token_mint @ TokenMillError::InvalidQuoteTokenMint
    )]
    pub market: AccountLoader<'info, Market>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = market,
        associated_token::token_program = quote_token_program
    )]
    pub market_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = creator,
        associated_token::token_program = quote_token_program
    )]
    pub creator_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    pub creator: Signer<'info>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<ClaimCreatorFees>) -> Result<()> {
    let pending_fees = 0;

    emit_cpi!(TokenMillCreatorFeeClaimEvent {
        market: ctx.accounts.market.key(),
        creator: ctx.accounts.creator.key(),
        fees_distributed: pending_fees,
    });

    Ok(())
}
