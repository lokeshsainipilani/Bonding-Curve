use anchor_lang::prelude::*;

declare_id!("7GBJtXN4gPCRxgAfBn7yYAQd46VU5TAfQosod1SU28qh");

#[program]
pub mod bonding_curve {
    

    use super::*;

    pub fn configure(ctx: Context<Configure>, new_config: ConfigSettings) -> Result<()>{
        ctx.accounts.process(new_config)
    }

    pub fn launch(ctx: Context<'_, '_, '_, 'info, Launch<'info>>,
        name:String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        ctx.accounts.process(name, uri, symbol, ctx.bumps.global_config)
    }

    pub fn swap(ctx: Context<'_, '_, '_, 'info, Swap<'info>>, amount:u64, direction:u8, min_out: u64) -> Result<()> {
        ctx.accounts.process(amount, direction, min_out, ctx.bumps.bonding_curve)
    }

    pub fn migrate(ctx: Context<Migrate>) -> Result<()> {
        Migrate::process(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
