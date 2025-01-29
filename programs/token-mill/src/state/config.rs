use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TokenMillConfig {
    pub authority: Pubkey,
    pub pending_authority: Option<Pubkey>,
    pub protocol_fee_recipient: Pubkey,
    pub default_protocol_fee_share: u16,
    pub referral_fee_share: u16,
}
