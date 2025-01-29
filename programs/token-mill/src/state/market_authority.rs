use anchor_lang::prelude::*;

pub const MARKET_AUTHORITY_PDA_SEED: &str = "market_authority";

#[account]
#[derive(Debug, InitSpace)]
pub struct MarketAuthority {
    pub bump: u8,
}
