use risc0_zkvm::guest::env;
use shared::Inputs;

fn main() {
    let serialized_inputs: Vec<u8> = env::read();
    let inputs = Inputs::from_bytes(&serialized_inputs);
    let is_valid = inputs.is_valid();
    assert!(is_valid);
    env::commit(&is_valid);
}
