use anchor_lang::prelude::*;

#[account]
pub struct LendingPool {
    pub authority: Pubkey,
    pub lp_mint: Pubkey,
    pub vault: Pubkey,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub collected_fees: u64,
    pub creation_time: i64,
    pub interest_rate: u64, // Represented as a percentage with 6 decimal places (e.g., 5000000 = 5%)
}

impl LendingPool {
    pub fn get_current_interest_rate(&self) -> u64 {
        // In a real implementation, this could vary based on utilization rate
        self.interest_rate
    }
}