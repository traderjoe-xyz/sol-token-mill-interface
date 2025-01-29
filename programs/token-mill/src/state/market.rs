use anchor_lang::prelude::*;

use crate::constant::PRICES_LENGTH;

pub const MARKET_PDA_SEED: &str = "market";

#[zero_copy]
#[derive(Debug, InitSpace)]
pub struct MarketFees {
    /// staking_fee_share + creator_fee_share + protocol_fee_share = 100%
    pub staking_fee_share: u16,
    pub creator_fee_share: u16,
    _space: u32,

    pub pending_staking_fees: u64,
    pub pending_creator_fees: u64,
}

#[account(zero_copy)]
#[derive(Debug, InitSpace)]
pub struct Market {
    pub config: Pubkey,
    pub creator: Pubkey,

    pub base_token_mint: Pubkey,
    pub quote_token_mint: Pubkey,

    pub base_reserve: u64,

    pub bid_prices: [u64; PRICES_LENGTH],
    pub ask_prices: [u64; PRICES_LENGTH],

    pub width_scaled: u64,
    pub total_supply: u64,

    pub fees: MarketFees,

    pub quote_token_decimals: u8,
    pub bump: u8,

    pub is_permissioned: u8,
    _space: [u8; 5],
}
