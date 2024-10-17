use crate::aes::aes;
use crate::gctr::{gctr, increment_32};
use crate::ghash::ghash;
use crate::utils::{block_to_stream, stream_to_block, Block, Stream};

// AES-GCM encryption
pub fn aes_gcm(key: Stream, plaintext: Stream, iv: [u8; 12], aad: Stream) -> (Stream, Stream) {
    // Step 1: Generate hash key as encryption of a zero block with AES
    let zero_block: Block = [[0; 4]; 4];
    let hashkey = block_to_stream(aes(zero_block, key));

    // Step 2: Generate j0 as iv || 0 ^{31} || 1, where || is concatenation
    let mut j0 = [0; 16];
    for i in 0..12 {
        j0[i] = iv[i];
    }
    j0[15] = 1; // Final byte of j0 is set to 1
    let j0_block = stream_to_block(j0);

    // Step 3: Perform GCTR on the incremented J0 and the plaintext
    let ciphertext = gctr(key, increment_32(j0_block), plaintext);

    // Step 4: Construct block S for GHASH
    let mut s_blocks = [[0u8; 16]; 3];
    s_blocks[0] = aad; // First block is AAD
    s_blocks[1] = ciphertext; // Second block is ciphertext
    s_blocks[2] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ]; // Padding and length encoding

    // GHASH computation
    let s_blocks_array: [[u8; 16]; 3] = [s_blocks[0], s_blocks[1], s_blocks[2]];
    let ghash_result = ghash(hashkey, s_blocks_array);

    // Step 5: Calculate the authentication tag T using GCTR on J0 and GHASH result
    let tag = gctr(key, j0_block, ghash_result);

    (ciphertext, tag)
}

// Test cases for AES-GCM
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm() {
        let key = [0x00u8; 16]; // 128-bit key
        let plaintext = [0x00u8; 16]; // Example plaintext
        let iv = [0x00u8; 12]; // 96-bit IV
        let aad = [0x00u8; 16]; // Example AAD
        let expected_output = [
            0x03, 0x88, 0xda, 0xce, 0x60, 0xb6, 0xa3, 0x92, 0xf3, 0x28, 0xc2, 0xb9, 0x71, 0xb2,
            0xfe, 0x78,
        ];

        let (ciphertext, _tag) = aes_gcm(key, plaintext, iv, aad);
        assert_eq!(ciphertext, expected_output);
    }

    #[test]
    fn test_aes_gcm_2() {
        let key = [0x31u8; 16]; // Another 128-bit key
        let iv = [0x31u8; 12]; // Another 96-bit IV
        let msg = [
            0x74, 0x65, 0x73, 0x74, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x30, 0x30, 0x30, 0x30, 0x30,
            0x30, 0x30,
        ]; // "testhello0000000"
        let aad = [0x00u8; 16]; // Example AAD
        let expected_ciphertext = [
            0x29, 0x29, 0xd2, 0xbb, 0x1a, 0xe9, 0x48, 0x04, 0x40, 0x2b, 0x8e, 0x77, 0x6e, 0x0d,
            0x33, 0x56,
        ]; // Expected ciphertext

        let (ciphertext, _tag) = aes_gcm(key, msg, iv, aad);
        assert_eq!(ciphertext, expected_ciphertext);
    }
}
