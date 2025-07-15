//declare_id!("71Qrt2waYnddGjzb9jubTtjgiDBWSfy3cFPdeKHWrFhX");
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

// Import our custom modules
mod errors;
pub mod state;
mod instructions;

use instructions::*;

// This is the program ID - a unique identifier for our AMM program
declare_id!("71Qrt2waYnddGjzb9jubTtjgiDBWSfy3cFPdeKHWrFhX");

#[program]
pub mod amm {
    use super::*;

    // Initialize a new AMM (Automated Market Maker)
    // Creates the pool with two tokens and sets up the initial configuration
    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)
    }

    // Add liquidity to the pool
    // Users can deposit both tokens to earn trading fees
    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    // Remove liquidity from the pool
    // Users can withdraw their tokens and LP tokens
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, max_x, max_y)
    }

    // Swap one token for another
    // Users can trade tokens using the AMM's pricing formula
    pub fn swap(ctx: Context<Swap>, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }
}