use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Burn, Mint, Token, TokenAccount, Transfer, burn}};
use constant_product_curve::ConstantProduct;

use crate::{errors::AmmError, state::Config};

// This struct defines all the accounts needed to withdraw liquidity from the AMM
// Users can burn their LP tokens to get back their original tokens
#[derive(Accounts)]
pub struct Withdraw<'info> {
    // The user who wants to remove liquidity (pays for the transaction)
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
    
    // User's token accounts - where they receive their tokens back
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
    
    // User's LP token account - where their LP tokens are burned from
    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = config,
    )]
    pub user_lp: Account<'info, TokenAccount>,
    
    // Required program accounts
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Withdraw<'info> {
    // Main withdraw function - removes liquidity from the pool
    pub fn withdraw(
        &mut self,
        amount: u64, // Amount of LP tokens that the user wants to "burn"
        min_x: u64,  // Minimum amount of token X that the user wants to receive
        min_y: u64,  // Minimum amount of token Y that the user wants to receive
    ) -> Result<()> {
        // Check that the pool is not locked
        require!(self.config.locked == false, AmmError::PoolLocked);
        require!(amount != 0, AmmError::InvalidAmount);
        require!(min_x != 0 || min_y != 0, AmmError::InvalidAmount);

        // Calculate how much of each token the user should receive
        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount, 
            self.vault_y.amount, 
            self.mint_lp.supply, 
            amount, 
            6,
        )
        .map_err(AmmError::from)?;

        // Check that the calculated amounts meet the user's minimum requirements
        require!(min_x <= amounts.x && min_y <= amounts.y, AmmError::SlippageExceeded);

        // Transfer tokens from pool vaults to user
        self.withdraw_tokens(true, amounts.x)?;  // Transfer token X
        self.withdraw_tokens(false, amounts.y)?; // Transfer token Y
        
        // Burn the user's LP tokens
        self.burn_lp_tokens(amount)?;
        
        Ok(())
    }

    // Helper function to transfer tokens from pool vault to user
    pub fn withdraw_tokens(&self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_x {
            true => (self.vault_x.to_account_info(), self.user_x.to_account_info()),
            false => (self.vault_y.to_account_info(), self.user_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.config.to_account_info(),
        };

        // Create the authority seeds for the config account
        let seeds = &[
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }

    // Helper function to burn LP tokens from the user
    pub fn burn_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_context, amount)?;

        Ok(())
    }
}