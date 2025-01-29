use anchor_lang::prelude::*;

use crate::{
    state::{Market, StakePosition},
    STAKING_POSITION_PDA_SEED,
};

#[derive(Accounts)]
pub struct CreateStakePosition<'info> {
    pub market: AccountLoader<'info, Market>,

    #[account(
        init,
        payer = user,
        space = 8 + StakePosition::INIT_SPACE,
        seeds = [STAKING_POSITION_PDA_SEED.as_bytes(), market.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake_position: Account<'info, StakePosition>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<CreateStakePosition>) -> Result<()> {
    Ok(())
}
