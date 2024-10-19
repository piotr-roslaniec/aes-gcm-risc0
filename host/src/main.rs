use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};
use prettytable::{row, Table};
use risc0_zkp::hal::tracker;
use risc0_zkvm::serde::from_slice;
use risc0_zkvm::{default_prover, ExecutorEnv};
use serde::Serialize;
use serde_with::{serde_as, DurationNanoSeconds};
use shared::{AesGcmNativeTestCase, AesGcmTestCase, AesTestCase, TestCase};
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

    let test_cases = vec![
        TestCase("AES".to_string(), AesTestCase::default_case().to_bytes()),
        TestCase(
            "AES-GCM".to_string(),
            AesGcmTestCase::default_case().to_bytes(),
        ),
        TestCase(
            "AES-GCM-native".to_string(),
            AesGcmNativeTestCase::default_case().to_bytes(),
        ),
    ];

    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Duration (ms)",
        "Cycles",
        "RAM (bytes)",
        "Seal (bytes)",
        "Throughput (Hz)"
    ]);

    for test_case in test_cases {
        let serialized_test_case = test_case.to_bytes();

        let env = ExecutorEnv::builder()
            .write(&serialized_test_case)
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
            name: test_case.0,
            duration,
            cycles,
            ram: ram_usage,
            seal: seal_size,
            throughput,
        };

        table.add_row(row![
            performance_data.name,
            format!("{:.2}", performance_data.duration.as_millis()),
            performance_data.cycles,
            performance_data.ram,
            performance_data.seal,
            format!("{:.2}", performance_data.throughput)
        ]);
    }

    println!("### Performance Data");
    table.printstd();
}
