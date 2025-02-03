use anchor_lang::prelude::*;

pub const SWAP_AUTHORITY_BADGE_PDA_SEED: &str = "swap_authority";

#[account]
#[derive(Debug, InitSpace)]
pub struct SwapAuthorityBadge {
    pub bump: u8,
}
