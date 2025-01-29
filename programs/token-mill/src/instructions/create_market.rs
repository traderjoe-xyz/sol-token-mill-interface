use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Token2022,
    token_interface::{Mint, TokenAccount},
};

use crate::{
    constant::MILL_TOKEN_DECIMALS,
    errors::TokenMillError,
    events::TokenMillMarketCreationEvent,
    state::{Market, TokenMillConfig},
    QuoteTokenBadge, QuoteTokenBadgeStatus, MARKET_PDA_SEED, QUOTE_TOKEN_BADGE_PDA_SEED,
};

#[event_cpi]
#[derive(Accounts)]
pub struct CreateMarket<'info> {
    pub config: Account<'info, TokenMillConfig>,

    #[account(
        init,
        seeds = [MARKET_PDA_SEED.as_bytes(), base_token_mint.key().as_ref()],
        bump,
        payer = creator,
        space = 8 + Market::INIT_SPACE
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        init,
        payer = creator,
        mint::token_program = token_program,
        mint::authority = market,
        mint::decimals = MILL_TOKEN_DECIMALS,
        extensions::metadata_pointer::authority = market,
        extensions::metadata_pointer::metadata_address = base_token_mint,
    )]
    pub base_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = base_token_mint,
        associated_token::authority = market,
        associated_token::token_program = token_program
    )]
    pub market_base_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        seeds = [
            QUOTE_TOKEN_BADGE_PDA_SEED.as_bytes(),
            config.key().as_ref(),
            quote_token_mint.key().as_ref(),
        ],
        bump = quote_token_badge.bump,
        constraint = quote_token_badge.status == QuoteTokenBadgeStatus::Enabled || creator.key() == config.authority @ TokenMillError::InvalidQuoteAssetBadge,
    )]
    pub quote_token_badge: Option<Account<'info, QuoteTokenBadge>>,

    pub quote_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(
    ctx: Context<CreateMarket>,
    _name: String,
    _symbol: String,
    _uri: String,
    total_supply: u64,
    creator_fee_share: u16,
    staking_fee_share: u16,
) -> Result<()> {
    emit_cpi!(TokenMillMarketCreationEvent {
        config: ctx.accounts.config.key(),
        market: ctx.accounts.market.key(),
        creator: ctx.accounts.creator.key(),
        base_token_mint: ctx.accounts.base_token_mint.key(),
        quote_token_mint: ctx.accounts.quote_token_mint.key(),
        total_supply,
        protocol_fee_share: ctx.accounts.config.default_protocol_fee_share,
        referral_fee_share: ctx.accounts.config.referral_fee_share,
        creator_fee_share,
        staking_fee_share,
    });

    Ok(())
}
