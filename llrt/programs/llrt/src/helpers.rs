// Helper function to calculate interest
pub fn calculate_interest(principal: u64, rate: u64, time: u64) -> u64 {
    // rate is annual interest rate with 6 decimal places
    // time is in seconds
    let interest = (principal as u128)
        .checked_mul(rate as u128)
        .unwrap()
        .checked_mul(time as u128)
        .unwrap()
        .checked_div(100_000_000 * 365 * 24 * 60 * 60)
        .unwrap();
    interest as u64
}