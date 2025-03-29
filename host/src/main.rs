

// use serde::{Serialize, Deserialize};
// use methods::{
//     GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID
// };
// use risc0_zkvm::{default_prover, ExecutorEnv};
// use hex;
// use bincode;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ProofResponse{
//     pub result: bool,
//     pub inner_hex: String,
//     pub journal_hex: String,
//     pub image_id_hex: String,
// }

// fn main() {
//     tracing_subscriber::fmt()
//         .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
//         .init();

//     let min_donation_amount: u64 = 50_000_000_000_000_000;

//     let env = ExecutorEnv::builder()
//         .write(&min_donation_amount)
//         .unwrap()
//         .build()
//         .unwrap();

//     let prover = default_prover();
//     let receipt = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF).unwrap();
//     let verify_receipt = receipt.receipt;

//     let inner_hex = hex::encode(bincode::serialize(&verify_receipt.inner).unwrap());
//     let journal_hex = hex::encode(bincode::serialize(&verify_receipt.journal).unwrap());

//     let mut image_id_hex = String::new();
//     for &value in GUEST_CODE_FOR_ZK_PROOF_ELF {
//         image_id_hex.push_str(&format!("{:08x}", value.to_be()));
//     }

//     let result: bool = verify_receipt.journal.decode().unwrap();

//     let proof_output = ProofResponse {
//         result,
//         inner_hex,
//         journal_hex,
//         image_id_hex,
//     };

//     verify_receipt
//         .verify(GUEST_CODE_FOR_ZK_PROOF_ID)
//         .unwrap();

//     println!("{:?}", proof_output); 
// }
// // RISC0_DEV_MODE=0 cargo run --release





use serde::{Serialize, Deserialize};
use methods::{
    GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs; 
use hex;

#[derive(Serialize, Deserialize)]
pub struct ProofOutput{
    pub proof: String,
    pub pub_inputs: String,
    pub image_id: String,
}

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();


    let min_donation_amount: u64 = 50_000_000_000_000_000;

    let env = ExecutorEnv::builder()
        .write(&min_donation_amount)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover
        .prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)
        .unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    // TODO: Implement code for retrieving receipt journal here.

    // For example:
    let _output: u64 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt
        .verify(GUEST_CODE_FOR_ZK_PROOF_ID)
        .unwrap();


    let mut bin_receipt = Vec::new();
    ciborium::into_writer(&receipt, &mut bin_receipt).unwrap();
    let proof = hex::encode(&bin_receipt);

    fs::write("proof.txt", hex::encode(&bin_receipt)).unwrap();
    let receipt_journal_bytes_array = &receipt.journal.bytes.as_slice();
    let pub_inputs = hex::encode(&receipt_journal_bytes_array);
    
    let image_id_hex = hex::encode(
        GUEST_CODE_FOR_ZK_PROOF_ID
            .into_iter()
            .flat_map(|v| v.to_le_bytes().into_iter())
            .collect::<Vec<_>>(),
    );
    
    let proof_output = ProofOutput{
        proof: "0x".to_owned()+&proof,
        pub_inputs: "0x".to_owned()+&pub_inputs,
        image_id: "0x".to_owned()+&image_id_hex,
    };

    let proof_output_json = serde_json::to_string(&proof_output).unwrap();
    fs::write("proof.json", proof_output_json).unwrap();

}



