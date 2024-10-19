use risc0_zkvm::guest::env;
use shared::{AesGcmNativeTestCase, AesGcmTestCase, AesTestCase, TestCase};

fn main() {
    let serialized_inputs: Vec<u8> = env::read();
    let test_case = TestCase::from_bytes(&serialized_inputs);

    if test_case.0 == "AES" {
        let concrete_test_case = AesTestCase::from_bytes(&test_case.1.to_vec());
        let is_valid = concrete_test_case.is_valid();
        assert!(is_valid);
        env::commit(&is_valid);
    } else if test_case.0 == "AES-GCM" {
        let concrete_test_case = AesGcmTestCase::from_bytes(&test_case.1.to_vec());
        let is_valid = concrete_test_case.is_valid();
        assert!(is_valid);
        env::commit(&is_valid);
    } else if test_case.0 == "AES-GCM-native" {
        let concrete_test_case = AesGcmNativeTestCase::from_bytes(&test_case.1.to_vec());
        let is_valid = concrete_test_case.is_valid();
        assert!(is_valid);
        env::commit(&is_valid);
    } else {
        panic!("Not a valid test case: {}", test_case.0)
    }
}
