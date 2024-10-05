use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::LendingPool;
#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 8 + 8 + 8,
        seeds = [b"pool"],
        bump
    )]
    pub pool: Account<'info, LendingPool>,
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = pool,
    )]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializePool>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    pool.authority = ctx.accounts.authority.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();
    pool.vault = ctx.accounts.vault.key();
    pool.total_deposits = 0;
    pool.total_borrows = 0;
    pool.creation_time = Clock::get()?.unix_timestamp;
    pool.interest_rate = 5000000;
    Ok(())
}