use anchor_lang::prelude::*;

pub const REFERRAL_ACCOUNT_PDA_SEED: &str = "referral";

#[account]
#[derive(Debug, InitSpace)]
pub struct ReferralAccount {
    pub bump: u8,
    pub config: Pubkey,
    pub referrer: Pubkey,
}
