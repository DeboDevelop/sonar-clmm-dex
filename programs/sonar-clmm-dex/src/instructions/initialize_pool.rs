use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(tick_spacing: u16)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 200, // Adjust space as needed
        seeds = [
            b"pool",
            token_mint_a.key().as_ref(),
            token_mint_b.key().as_ref(),
            tick_spacing.to_le_bytes().as_ref()
        ],
        bump,
        constraint = token_mint_a.key() < token_mint_b.key() @ PoolError::InvalidTokenOrder
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        constraint = token_mint_a.is_initialized @ PoolError::MintNotInitialized
    )]
    pub token_mint_a: Account<'info, Mint>,
    #[account(
        constraint = token_mint_b.is_initialized @ PoolError::MintNotInitialized
    )]
    pub token_mint_b: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        token::mint = token_mint_a,
        token::authority = pool
    )]
    pub vault_a: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = payer,
        token::mint = token_mint_b,
        token::authority = pool
    )]
    pub vault_b: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
#[derive(Default, Debug)]
pub struct Pool {
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub tick_spacing: u16,
    pub fee_rate: u16,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current: i32,
    pub bump: u8,
}

#[error_code]
pub enum PoolError {
    #[msg("Token mints must be in sorted order")]
    InvalidTokenOrder,
    #[msg("Pool already exists for this token pair and tick spacing")]
    PoolAlreadyExists,
    #[msg("Token mint is not initialized")]
    MintNotInitialized,
}

pub fn handler(
    ctx: Context<InitializePool>,
    tick_spacing: u16,
    initial_sqrt_price: u128,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.token_mint_a = ctx.accounts.token_mint_a.key();
    pool.token_mint_b = ctx.accounts.token_mint_b.key();
    pool.tick_spacing = tick_spacing;
    pool.fee_rate = 300; // 0.3% (in basis points, 300 = 0.3%)
    pool.liquidity = 0;
    pool.sqrt_price = initial_sqrt_price;
    pool.tick_current = 0;
    pool.bump = ctx.bumps.pool;

    msg!("Pool initialized: {:?}", pool);
    Ok(())
}