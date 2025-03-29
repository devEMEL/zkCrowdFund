
use risc0_zkvm::guest::env;
use serde::{Serialize, Deserialize};

const MIN_DONATION_AMOUNT_CONSTANT: i64 =  50000000000000000;  // 0.05 × 10^18 wei = 5 × 10^16 wei, or 50,000,000,000,000,000 wei.

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Campaign {
    name: String,
    description: String,
    min_donation_amount: u64,
}

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let name = env::read::<i64>();
    let description = env::read::<i64>();
    let min_donation_amount = env::read::<i64>();

    // TODO: do something with the input

    let result = min_donation_amount >= MIN_DONATION_AMOUNT_CONSTANT;

    // write public output to the journal
    env::commit(&result);
}
