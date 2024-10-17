use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use risc0_zkp::hal::tracker;
use risc0_zkvm::serde::from_slice;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use serde_with::{serde_as, DurationNanoSeconds};
use shared::{Block, Inputs, Stream};
use std::time::{Duration, Instant};

#[serde_as]
#[derive(Debug, Serialize)]
struct PerformanceData {
    name: String,
    #[serde_as(as = "DurationNanoSeconds")]
    duration: Duration,
    cycles: u64,
    ram: usize,
    seal: usize,
    throughput: f32,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let block: Block = [
        [0x32, 0x88, 0x31, 0xe0],
        [0x43, 0x5a, 0x31, 0x37],
        [0xf6, 0x30, 0x98, 0x07],
        [0xa8, 0x8d, 0xa2, 0x34],
    ];
    let key: Stream = [
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ];
    let cipher: Block = [
        [0x39, 0x02, 0xdc, 0x19],
        [0x25, 0xdc, 0x11, 0x6a],
        [0x84, 0x09, 0x85, 0x0b],
        [0x1d, 0xfb, 0x97, 0x32],
    ];

    let inputs = Inputs { block, key, cipher };
    let serialized_inputs = inputs.to_bytes();

    let env = ExecutorEnv::builder()
        .write(&serialized_inputs)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();
    tracker().lock().unwrap().reset();
    let start = Instant::now();
    let prove_info = prover.prove(env, GUEST_CODE_FOR_ZK_PROOF_ELF).unwrap();
    let duration = start.elapsed();

    let receipt = prove_info.receipt;
    receipt.verify(GUEST_CODE_FOR_ZK_PROOF_ID).unwrap();

    let journal = receipt.journal;
    let result: bool = from_slice(&journal.bytes).unwrap();
    println!("Result from proof journal: {}", result);

    let tracker_lock = tracker().lock().unwrap();
    let ram_usage = tracker_lock.peak; // TODO: For some reason RAM is 0, fix
    let cycles = prove_info.stats.total_cycles;
    // TODO: How to get seal size?
    let seal_size = receipt
        .inner
        .composite()
        .expect("Receipt is not composite, are you in development mode?")
        .segments
        .iter()
        .map(|x| x.get_seal_bytes().len())
        .sum();
    let throughput = cycles as f32 / duration.as_secs_f32();
    let performance_data = PerformanceData {
        name: "naive_aes_gcm".to_string(),
        duration,
        cycles,
        ram: ram_usage,
        seal: seal_size,
        throughput,
    };
    println!(
        "Performance Data:\nName: {}\nDuration: {:.2?}ms\nCycles: {}\nRAM: {} bytes\nSeal: {} bytes\nThroughput: {} Hz",
        performance_data.name,
        performance_data.duration.as_millis(),
        performance_data.cycles,
        performance_data.ram,
        performance_data.seal,
        performance_data.throughput
    );
}
