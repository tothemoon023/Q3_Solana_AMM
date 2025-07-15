use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Transfer, transfer, Mint, Token, TokenAccount, MintTo, mint_to}};
use constant_product_curve::ConstantProduct;

use crate::{errors::AmmError, state::Config};

// This struct defines all the accounts needed to deposit liquidity into the AMM
// Users can add both tokens to earn trading fees
#[derive(Accounts)]
pub struct Deposit<'info> {
    // The user who wants to add liquidity (pays for the transaction)
    #[account(mut)]
    pub user: Signer<'info>,
    
    // The two tokens that can be traded in this pool
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    
    // Pool configuration - contains all the pool settings
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    
    // LP token mint - represents the user's share of the pool
    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub mint_lp: Account<'info, Mint>,
    
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
    
    // User's token accounts - where their tokens come from
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>,
    
    // User's LP token account - where they receive their LP tokens
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_lp: Account<'info, TokenAccount>,
    
    // Required program accounts
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Deposit<'info> {
    // Main deposit function - adds liquidity to the pool
    pub fn deposit (
        &mut self,
        amount: u64, // Amount of LP tokens that the user wants to "claim"
        max_x: u64, // Maximum amount of token X that the user is willing to deposit
        max_y: u64, // Maximum amount of token Y that the user is willing to deposit
    ) -> Result<()> {
        // Check that the pool is not locked
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);

        // Calculate how much of each token to deposit
        let (x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 && self.vault_y.amount == 0 {
            // If this is the first deposit, use the maximum amounts
            true => (max_x, max_y),
            // Otherwise, calculate the correct ratio based on current pool state
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount, 
                    self.vault_y.amount, 
                    self.mint_lp.supply, 
                    amount, 
                    6
                ).unwrap();
                (amounts.x, amounts.y)
            }
        };

        // Check that the calculated amounts don't exceed user's limits
        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);

        // Transfer tokens from user to pool vaults
        self.deposit_tokens(true, x)?;  // Deposit token X
        self.deposit_tokens(false, y)?; // Deposit token Y
        
        // Give LP tokens to the user
        self.mint_lp_tokens(amount)
    }

    // Helper function to transfer tokens from user to pool vault
    pub fn deposit_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(ctx, amount)
    }

    // Helper function to mint LP tokens for the user
    pub fn mint_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };

        // Create the authority seeds for the config account
        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(ctx, amount)
    }
}