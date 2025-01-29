use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VestingPlan {
    pub stake_position: Pubkey,
    pub amount_vested: u64,
    pub amount_released: u64,
    pub start: i64,
    pub cliff_duration: i64,
    pub vesting_duration: i64,
}
