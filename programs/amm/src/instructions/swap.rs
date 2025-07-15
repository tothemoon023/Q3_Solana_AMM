use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount, Transfer, transfer}};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{errors::AmmError, state::Config};

// This struct defines all the accounts needed to swap tokens in the AMM
// Users can trade one token for another using the constant product formula
#[derive(Accounts)]
pub struct Swap<'info> {
    // The user who wants to swap tokens (pays for the transaction)
    #[account(mut)]
    pub user: Signer<'info>,
    
    // The two tokens that can be traded in this pool
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    
    // User's token accounts - where their tokens are stored
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>,
    
    // Pool vaults - where the AMM stores the tokens
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    
    // Pool configuration - contains all the pool settings
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    
    // Required program accounts
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    // Main swap function - trades one token for another
    pub fn swap(&mut self, is_x: bool, amount: u64, min: u64) -> Result<()> {
        // Check that the pool is not locked
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount > 0, AmmError::InvalidAmount);

        // Create the constant product curve with current pool state
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
             self.vault_y.amount, 
             self.vault_x.amount, 
             self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

        // Determine which token is being swapped
        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        // Calculate the swap using the constant product formula
        let res = curve.swap(p, amount, min).map_err(AmmError::from)?;

        // Verify the swap amounts are valid
        require!(res.deposit != 0, AmmError::InvalidAmount);
        require!(res.withdraw != 0, AmmError::InvalidAmount);

        // Execute the swap by transferring tokens
        self.deposit_tokens(is_x, res.deposit)?;  // User deposits input token
        self.withdraw_tokens(is_x, res.withdraw)?; // User receives output token
        
        // Note: Fee is automatically calculated and kept in the pool

        Ok(())
    }

    // Helper function to transfer tokens from user to pool vault
    pub fn deposit_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info() , self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, accounts);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    // Helper function to transfer tokens from pool vault to user
    pub fn withdraw_tokens(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.vault_y.to_account_info() , self.user_y.to_account_info()),
            false => (self.vault_x.to_account_info(), self.user_x.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let accounts = Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: self.config.to_account_info(),
        };

        // Create the authority seeds for the config account
        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}