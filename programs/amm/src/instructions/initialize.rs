use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::state::Config;

// This struct defines all the accounts needed to initialize an AMM
// Each field represents an account that must be provided when calling initialize
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize <'info> {
    // The person who's creating the AMM (pays for the transaction)
    #[account(mut)]
    pub initializer: Signer<'info>,
    
    // The first token that can be traded (e.g., USDC)
    pub mint_x: Account<'info, Mint>,
    
    // The second token that can be traded (e.g., SOL)
    pub mint_y: Account<'info, Mint>,
    
    // LP token mint - represents liquidity provider shares
    // This will be created by the program
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp: Account<'info, Mint>,
    
    // Vault for storing Token X - where the AMM keeps its X tokens
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    
    // Vault for storing Token Y - where the AMM keeps its Y tokens
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    
    // Configuration account - stores all pool settings and state
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    
    // Required program accounts
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    // Initialize the AMM with the provided parameters
    pub fn init(&mut self, seed: u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps) -> Result<()> {
        // Store all the configuration data in the config account
        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false, // Pool starts unlocked
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
        });

        Ok(())
    }
}