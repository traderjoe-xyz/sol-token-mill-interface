use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    errors::TokenMillError,
    events::TokenMillSwapEvent,
    state::{Market, SWAP_AUTHORITY_BADGE_PDA_SEED},
    SwapAmountType, SwapAuthorityBadge, SwapType, TokenMillConfig,
};

#[event_cpi]
#[derive(Accounts)]
pub struct PermissionedSwap<'info> {
    pub config: Account<'info, TokenMillConfig>,

    #[account(
        mut,
        has_one = config @ TokenMillError::InvalidConfigAccount,
        has_one = base_token_mint @ TokenMillError::InvalidMintAccount,
        has_one = quote_token_mint @ TokenMillError::InvalidMintAccount
    )]
    pub market: AccountLoader<'info, Market>,

    #[account(
        seeds = [SWAP_AUTHORITY_BADGE_PDA_SEED.as_bytes(), market.key().as_ref(), swap_authority.key().as_ref()],
        bump = swap_authority_badge.bump,
    )]
    pub swap_authority_badge: Account<'info, SwapAuthorityBadge>,

    pub base_token_mint: InterfaceAccount<'info, Mint>,

    pub quote_token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = base_token_mint,
        associated_token::authority = market,
        associated_token::token_program = base_token_program
    )]
    pub market_base_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = market,
        associated_token::token_program = quote_token_program
    )]
    pub market_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::mint = base_token_mint)]
    pub user_base_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, token::mint = quote_token_mint)]
    pub user_quote_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = quote_token_mint,
        associated_token::authority = config.protocol_fee_recipient,
        associated_token::token_program = quote_token_program
    )]
    pub protocol_quote_token_ata: InterfaceAccount<'info, TokenAccount>,

    // Referral account can be any token account
    // For UX purposes, LFJ's UI provides the ATA of the `ReferralAccount`, requiring the referrer to claim all the fees he receives
    #[account(mut)]
    pub referral_token_account: Option<InterfaceAccount<'info, TokenAccount>>,

    pub swap_authority: Signer<'info>,
    pub user: Signer<'info>,

    pub base_token_program: Interface<'info, TokenInterface>,

    pub quote_token_program: Interface<'info, TokenInterface>,
}

pub fn handler(
    ctx: Context<PermissionedSwap>,
    swap_type: SwapType,
    _swap_amount_type: SwapAmountType,
    _amount: u64,
    _other_amount_threshold: u64,
) -> Result<(u64, u64)> {
    emit_cpi!(TokenMillSwapEvent {
        user: ctx.accounts.user.key(),
        market: ctx.accounts.market.key(),
        swap_type,
        base_amount: 0,
        quote_amount: 0,
        referral_token_account: None,
        creator_fee: 0,
        staking_fee: 0,
        protocol_fee: 0,
        referral_fee: 0,
    });

    Ok((0, 0))
}
