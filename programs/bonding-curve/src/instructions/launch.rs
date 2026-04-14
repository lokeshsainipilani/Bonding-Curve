use crate::{
    constants:: TOKEN_DECIMAL,
    states::{BondingCurve, Config}
};

use anchor_lang::{prelude::*, system_program, solana_program::sysvar};

use anchor_spl::{
    associated_token::{self, AssociatedToken},
    metadata::{self, mpl_token_metadata::types::DateV2, Metadata},
    token::{self, spl_token::instruction::AuthorityType, Mint, Token, TokenAccount}
};

#[derive(Accounts)]
pub struct Launch<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        seeds = [Config::SEED_PREFIX.as_bytes()],
        bump
    )]
    global_config: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = creator,
        mint::decimals = TOKEN_DECIMAL,
        mint::authority = global_config.key(),
    )]
    token_mint: Box<Account<'info, Config>>,

    #[account(
        init,
        payer = creator,
        space = 8 + BondingCurve::LEN,
        seeds = [BondingCurve::SEED_PREFIX.as_bytes(), &token_mint.key().to_bytes()],
        bump
    )]
    bonding_curve: Box<Account<'info>, BondingCurve>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = token_mint,
        associated_token::authority = bonding_curve,
    )]
    curve_token_account: Box<Account<'info>, TokenAccount>
}