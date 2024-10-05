use anchor_lang::prelude::*;

//Lending Error? change this name
#[error_code]
pub enum LendingError {
    #[msg("Insufficient liquidity in the vault")]
    InsufficientLiquidity,
    #[msg("Repayment amount exceeds debt")]
    ExcessiveRepayment,
    #[msg("Insufficient collateral for the requested borrow amount")]
    InsufficientCollateral,
}
