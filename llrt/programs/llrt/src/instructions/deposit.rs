use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::LendingPool;
use crate::errors::LendingError;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"pool"], bump)]
    pub pool: Account<'info, LendingPool>,
    #[account(mut, constraint = lp_mint.key() == pool.lp_mint)]
    pub lp_mint: Account<'info, Mint>,
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub depositor_lp_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // Transfer SOL to the vault
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.depositor.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    // Create longer-lived bindings
    let lp_mint_key = ctx.accounts.lp_mint.key();
    let authority_key = ctx.accounts.pool.authority;
    let bump = ctx.bumps.pool;

    // Update pool state
    {
        let pool = &mut ctx.accounts.pool;
        pool.total_deposits = pool.total_deposits.checked_add(amount).ok_or(ProgramError::ArithmeticOverflow)?;
    }

    // Mint LP tokens to depositor
    let pool_seeds = &[
        b"pool".as_ref(),
        lp_mint_key.as_ref(),
        authority_key.as_ref(),
        &[bump],
    ];
    let signer = &[&pool_seeds[..]];

    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.lp_mint.to_account_info(),
        to: ctx.accounts.depositor_lp_account.to_account_info(),
        authority: ctx.accounts.pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::mint_to(cpi_ctx, amount)?;

    Ok(())
}