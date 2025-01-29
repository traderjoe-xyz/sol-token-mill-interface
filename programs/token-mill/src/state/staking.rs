use anchor_lang::prelude::*;

pub const MARKET_STAKING_PDA_SEED: &str = "market_staking";
pub const STAKING_POSITION_PDA_SEED: &str = "stake_position";

#[account]
#[derive(InitSpace)]
pub struct MarketStaking {
    pub market: Pubkey,
    pub amount_staked: u64,
    pub total_amount_vested: u64,
    pub acc_reward_amount_per_share: u128,
}

#[account]
#[derive(InitSpace)]
pub struct StakePosition {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount_staked: u64,
    pub total_amount_vested: u64,
    pub pending_rewards: u64,
    pub acc_reward_amount_per_share: u128,
}
