use anchor_lang::prelude::*;

use crate::{ReferralAccount, TokenMillConfig, REFERRAL_ACCOUNT_PDA_SEED};

#[derive(Accounts)]
#[instruction(referrer: Pubkey)]
pub struct CreateReferralAccount<'info> {
    pub config: Account<'info, TokenMillConfig>,

    #[account(
        init,
        seeds = [REFERRAL_ACCOUNT_PDA_SEED.as_bytes(), config.key().as_ref(), referrer.as_ref()],
        bump,
        payer = user,
        space = 8 + ReferralAccount::INIT_SPACE
    )]
    pub referral_account: Account<'info, ReferralAccount>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<CreateReferralAccount>, _referrer: Pubkey) -> Result<()> {
    Ok(())
}
