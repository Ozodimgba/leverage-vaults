use anchor_lang::prelude::*;

#[account]
pub struct BorrowerPosition {
    pub owner: Pubkey,
    pub borrowed_amount: u64,
    pub collateral_amount: u64,
    pub creation_time: i64,
    pub bump: u8,
}