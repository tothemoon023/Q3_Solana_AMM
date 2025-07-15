use anchor_lang::prelude::*;

// This struct stores all the configuration and state for an AMM pool
// It's stored on-chain and contains all the important pool information
#[account]
pub struct Config {
    pub seed: u64, // Unique identifier for this pool (allows multiple pools)
    pub authority: Option<Pubkey>, // Optional admin who can lock/unlock the pool
    pub mint_x: Pubkey, // Address of the first token (e.g., USDC)
    pub mint_y: Pubkey, // Address of the second token (e.g., SOL)
    pub fee: u16, // Trading fee in basis points (e.g., 500 = 0.5%)
    pub locked: bool, // Whether the pool is locked (prevents trading)
    pub config_bump: u8, // PDA bump for the config account
    pub lp_bump: u8, // PDA bump for the LP token mint
}

// Define how much space this account needs on-chain
// 8 bytes for discriminator + 8 for seed + 33 for authority + 32 for each mint + 2 for fee + 1 for locked + 1 for each bump
impl Space for Config {
    const INIT_SPACE: usize = 8 + 8 + (1 + 32) + 32 + 32 + 2 + 1 + 1 + 1;
}