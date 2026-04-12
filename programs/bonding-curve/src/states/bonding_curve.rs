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
    pub virtual_sol_reserve: u64
}