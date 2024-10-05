use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::{LendingPool, BorrowerPosition};
use crate::helpers::calculate_interest;

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub pool: Account<'info, LendingPool>,
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"position", borrower.key().as_ref()],
        bump = borrower_position.bump,
    )]
    pub borrower_position: Account<'info, BorrowerPosition>,
    #[account(mut)]
    pub borrower: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Repay>, amount: u64) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let borrower_position = &mut ctx.accounts.borrower_position;
    let clock = Clock::get()?;

    // Calculate accrued interest
    let time_elapsed = clock.unix_timestamp - borrower_position.creation_time;
    let interest_rate = pool.get_current_interest_rate();
    let interest = calculate_interest(
        borrower_position.borrowed_amount,
        interest_rate,
        time_elapsed as u64,
    );

    let total_owed = borrower_position.borrowed_amount.checked_add(interest).unwrap();
    let amount_to_repay = amount.min(total_owed);

    // Transfer SOL from borrower to vault
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.borrower.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, amount_to_repay)?;

    // Update borrower position
    borrower_position.borrowed_amount = borrower_position.borrowed_amount
        .checked_sub(amount_to_repay)
        .unwrap();
    borrower_position.creation_time = clock.unix_timestamp;

    // Update pool state
    pool.total_borrows = pool.total_borrows.checked_sub(amount_to_repay).unwrap();
    pool.total_deposits = pool.total_deposits.checked_add(amount_to_repay).unwrap();

    // Collect protocol fees (e.g., 10% of interest)
    let protocol_fee = interest.checked_mul(10).unwrap().checked_div(100).unwrap();
    pool.collected_fees = pool.collected_fees.checked_add(protocol_fee).unwrap();

    Ok(())
}