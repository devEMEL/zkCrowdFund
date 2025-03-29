

use risc0_zkvm::guest::env;

const MIN_DONATION_AMOUNT_CONSTANT: u64 = 50_000_000_000_000_000; // 0.05 ETH in wei

fn main() {
    // Read the input
    let min_donation_amount = env::read::<u64>();
    // Check if donation meets the minimum amount
    let result = min_donation_amount >= MIN_DONATION_AMOUNT_CONSTANT;

    // Write public output to the journal
    env::commit(&result);
}