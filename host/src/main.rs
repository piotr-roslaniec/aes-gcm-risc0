use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use risc0_zkvm::serde::from_slice;
use shared::{Block, Inputs, Stream};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Dummy test inputs
    let block: Block = [
        [0x32, 0x88, 0x31, 0xe0],
        [0x43, 0x5a, 0x31, 0x37],
        [0xf6, 0x30, 0x98, 0x07],
        [0xa8, 0x8d, 0xa2, 0x34],
    ];
    let key: Stream = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c
    ];
    let cipher: Block = [
        [0x39, 0x02, 0xdc, 0x19],
        [0x25, 0xdc, 0x11, 0x6a],
        [0x84, 0x09, 0x85, 0x0b],
        [0x1d, 0xfb, 0x97, 0x32],
    ];

    let inputs = Inputs { block, key, cipher };
    println!("{:?}", inputs);
    let serialized_inputs = inputs.to_bytes();

    let env = ExecutorEnv::builder()
        .write(&serialized_inputs)  // Pass the serialized input to the guest
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let prove_info = prover
        .prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF)
        .unwrap();

    let receipt = prove_info.receipt;
    receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID).unwrap();

    let journal = receipt.journal;
    let result: bool = from_slice(&journal.bytes).unwrap();
    println!("Result: {}", result)
}
