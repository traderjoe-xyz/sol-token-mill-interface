use anchor_lang::prelude::*;

use crate::{errors::TokenMillError, state::TokenMillConfig};

#[event_cpi]
#[derive(Accounts)]
pub struct ConfigUpdate<'info> {
    #[account(mut, has_one = authority @ TokenMillError::InvalidAuthority)]
    pub config: Account<'info, TokenMillConfig>,

    pub authority: Signer<'info>,
}

pub fn handler(_ctx: Context<ConfigUpdate>, _new_authority: Option<Pubkey>) -> Result<()> {
    Ok(())
}
