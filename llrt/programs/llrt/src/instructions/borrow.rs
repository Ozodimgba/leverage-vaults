use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::{LendingPool, BorrowerPosition};
use crate::errors::LendingError;

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut, seeds = [b"pool"], bump)]
    pub pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub borrower_lrt_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = borrower,
        space = 8 + 32 + 8 + 8,
        seeds = [b"position", borrower.key().as_ref()],
        bump
    )]
    pub borrower_position: Account<'info, BorrowerPosition>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}


pub fn handler(ctx: Context<Borrow>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let lrt_amount = ctx.accounts.borrower_lrt_account.amount;

    // Check collateralization ratio (90%)
    let max_borrow = (lrt_amount as u128 * 90 / 100) as u64;
    require!(amount <= max_borrow, LendingError::InsufficientCollateral);

    // Update pool state
    pool.total_borrows = pool.total_borrows.checked_add(amount).unwrap();

    // Transfer SOL from vault to borrower
    let lp_mint_key = ctx.accounts.pool.lp_mint.key();
    let authority_key = ctx.accounts.pool.authority.key();
    let pool_seeds = &[
        b"pool".as_ref(),
        lp_mint_key.as_ref(),
        authority_key.as_ref(),
        &[ctx.bumps.pool],
    ];
    let signer = &[&pool_seeds[..]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.borrower.to_account_info(),
        },
        signer,
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    // Update borrower's loan state
    let borrower_position = &mut ctx.accounts.borrower_position;
    borrower_position.owner = ctx.accounts.borrower.key();
    borrower_position.borrowed_amount = borrower_position.borrowed_amount.checked_add(amount).unwrap();
    borrower_position.collateral_amount = lrt_amount;

    Ok(())
}