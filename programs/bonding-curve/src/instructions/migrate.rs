use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccountm, Mint};
use crate::states::{BondingCurve, Config};
use crate::errors::Error;
use crate::utils::{sol_transfer_with_signer, token_transfer_with_signer, MigrationCompleted};
use raydium_amm_v3::{
    self,
    states::{AmmConfig, POOL_SEED, POOL_TICK_ARRAY_BITMAP_SEED, POOL_VAULT_SEED},
    program::AmmV3,
    libraries::tick_math,
};

#[derive(Accounts)]
pub struct Migrate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds::[Config::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds::[BondingCurve::SEED_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump,
        constraint = bonding_curve.is_completed @ Error::CurveNotCompleted,
        constraint = !bonding_curve.is_migrated @ Error::AlreadyMigrated,
    )]
    pub bonding_curve: Account<'info, BondingCurve>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    pub wsol_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = curve_token_account.owner == bonding_curve.key(),
        constraint = curve_token_account.amount > 0 @ Error::InsufficientTokenBalance,

    )]
    pub curve_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = *curve_sol_account.owner == bonding_curve.key(),
        constraint = curve_sol_account.lamports() > 0 @ Error::InsufficientSolBalance,
    )]
    pub curve_sol_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            POOL_SEED.as_bytes(),
            amm_config.key().as_ref(),
            wsol_mint.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        seeds::program = raydium_program,
        bump
    )]
    pub pool_state: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            wsol_mint.key().as_ref()
        ],
        seeds::program = raydium_program,
        bump,
    )]
    pub token_vault_0: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        seeds::program = raydium_program,
        bump
    )]
    pub tick_array_bitmap: UncheckedAccount<'info>,

    #[account(mut)]
    pub fee_recipient: AccountInfo<'info>,
    pub amm_config: Box<Account<'info, AmmConfig>>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub raydium_program: Program<'info, AmmV3>,
    pub rent: Sysvar<'info, Rent>,

}

impl <'info>Migrate<'info>{
    pub fn process(ctx: Context<Migrate>) -> Result<()>{
        let bonding_curve = &mut ctx.accounts.bonding_curve;
        let config = &ctx.accounts.config;

        require!(
            config.authority == ctx.accounts.authority.key(),
            Error::UnAuthorizedAddress
        );

        let sol_balance = ctx.accounts.curve_sol_account.lamports();
        let token_balance = ctx.accounts.curve_token_account.amount();

        let migration_fee = sol_balance
            .checked_mul(config.migration_fee_percentage as u64)
            .ok_or(Error::MathOverflow)?
            .checked_div(100)
            .ok_or(Error::MathOverflow)?;

        let remaining_sol = sol_balance
            .checked_sub(migration_fee)
            .ok_or(Error::InsufficientSolBalance)?;

        let init_sqrt_price = tick_math::get_sqrt_price_at_tick(0)?;
        let open_time = Clock::get()?.unix_timestamp as u64;

        let create_pool_accounts = raydium_amm_v3::cpi::accounts::CreatePool {
            pool_creator: ctx.accounts.authority.to_account_info(),
            amm_config: ctx.accounts.amm_config.to_account_info(),
            pool_state: ctx.accounts.pool_state.to_account_info(),
            token_mint_0: ctx.accounts.wsol_mint.to_account_info(),
            token_mint_1: ctx.accounts.token_mint.to_account_info(),
            token_vault_0: ctx.accounts.token_vault_0.to_account_info(),
            token_vault_1: ctx.accounts.token_vault_1.to_account_info(),
            observation_state: ctx.accounts.observation_state.to_account_info(),
            tick_array_bitmap: ctx.accounts.tick_array_bitmap.to_account_info(),
            token_program_0: ctx.accounts.token_program.to_account_info(),
            token_program_1: ctx.accounts.token_program.to_account_info(),
            system_program: ctx .accounts.system_program.to_account_info(),
            rent: ctx.accounts.to_account_info(),
        };

        let create_pool_ctx = CpiContext::new(
            ctx.accounts.raydium_program.to_account_info(),
            create_pool_accounts,
        );

        raydium_amm_v3::cpi::create_pool(
            create_pool_ctx,
            init_sqrt_price,
            open_time
        )?;

        

    }
}