use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{events::TokenMillReferralFeeClaimEvent, ReferralAccount};

#[event_cpi]
#[derive(Accounts)]
pub struct ClaimReferralFees<'info> {
    #[account(has_one = referrer)]
    pub referral_account: Account<'info, ReferralAccount>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = referral_account,
        associated_token::token_program = quote_token_program
    )]
    pub referral_account_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = referrer,
        associated_token::token_program = quote_token_program
    )]
    pub referrer_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    pub referrer: Signer<'info>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<ClaimReferralFees>) -> Result<()> {
    let pending_fees = 0;

    emit_cpi!(TokenMillReferralFeeClaimEvent {
        referrer: ctx.accounts.referrer.key(),
        quote_token_mint: ctx.accounts.quote_token_mint.key(),
        fees_distributed: pending_fees,
    });

    Ok(())
}
