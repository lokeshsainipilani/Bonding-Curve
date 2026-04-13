use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::errors::Error;
use crate::utils::{
    sol_transfer_from_user, sol_transfer_with_signer, token_transfer_user,
    token_transfer_with_signer, TokenPurchased, TokenSold, TokenCompleted
};

#[account]
pub struct bonding_curve{
    pub virtual_token_reserve: u64,
    pub virtual_sol_reserve: u64,
    pub real_token_reserve: u64,
    pub real_sol_reserve: u64,
    pub token_total_supplt: u64,
    pub is_completed: bool,
    pub is_reserved: bool,
    pub reserved: [u8; 8]
}

impl <'info>BondingCurve{
    pub const SEED_PREFIX: &'static str = "bonding_curve";
    pub const LEN: usize = 8 * 5 + 1 + 1 + 8;
    
    pub fn get_signer<'a>(mint: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3]{
        [
            Self::SEED_PREFIX.as_bytes(),
            mint.as_ref(),
            std::slice::from_ref(bump),
        ]
    }

    pub fn update_reserves(&mut self, reserve_lamport:u64, reserve_token:u64) -> Result<bool>{
        self.virtual_sol_reserve = reserve_lamport;
        self.virtual_token_reserve = reserve_token;

        Ok(true)
    }

    pub fn buy(
        &mut self,
        token_mint: &Account<'info, Mint>,
        curve_limit: u64,
        user: &Signer<'info>,
        curve_pda: &mut AccountInfo<'info>,
        fee_recipient: &mut AccountInfo<'info>,
        user_ata: &mut AccountInfo<'info>,
        curve_ata: &AccountInfo<'info>,
        amount_in: u64,
        min_amount_out: u64,
        fee_percentage: f64,
        curve_bump: u8,
        system_program: &AccountInfo<'info>,
        token_program: &AccountInfo<'info>
    ) -> Result<bool>{
        
    }
}