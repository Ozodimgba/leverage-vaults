use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub lsol_mint: Pubkey,
    pub bump: u8,
}