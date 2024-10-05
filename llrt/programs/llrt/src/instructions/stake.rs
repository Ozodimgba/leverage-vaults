use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::state::Vault;
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [b"vault", lsol_mint.key().as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut, constraint = lsol_mint.key() == vault.lsol_mint)]
    pub lsol_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_lsol_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    // Transfer staked assets (SOL) to vault
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        anchor_lang::system_program::Transfer {
            from: ctx.accounts.authority.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );
    anchor_lang::system_program::transfer(cpi_context, amount)?;

    // Create longer-lived bindings
    let lsol_mint_key = ctx.accounts.lsol_mint.key();
    let authority_key = ctx.accounts.authority.key();
    let bump = ctx.bumps.vault;

    // Mint equivalent lSOL LRT to user
    let vault_seeds = &[
        b"vault",
        lsol_mint_key.as_ref(),
        authority_key.as_ref(),
        &[bump]
    ];
    let signer = &[&vault_seeds[..]];

    let cpi_accounts = token::MintTo {
        mint: ctx.accounts.lsol_mint.to_account_info(),
        to: ctx.accounts.user_lsol_account.to_account_info(),
        authority: ctx.accounts.vault.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::mint_to(cpi_ctx, amount)?;

    Ok(())
}