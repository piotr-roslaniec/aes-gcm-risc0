use crate::aes::aes;
use crate::utils::{block_to_stream, stream_to_block, Block, Stream};

// Starting with fixed size input
// TODO: Make this work for arbitrary length input
pub fn gctr(key: Stream, initial_counter_block: Block, plaintext: Stream) -> Stream {
    let mut ciphertext: Block = [[0; 4]; 4];
    let pt_blocks = [stream_to_block(plaintext)];
    let mut counter_block = initial_counter_block;

    for block in &pt_blocks {
        // Encrypt counter block
        let encrypted_counter_block = aes(counter_block, key);
        // XOR with plaintext
        ciphertext = add_cipher(&encrypted_counter_block, block);
        // Update counter block
        counter_block = increment_32(counter_block);
    }

    block_to_stream(ciphertext)
}

pub(crate) fn increment_32(counter_block: Block) -> Block {
    let mut stream = block_to_stream(counter_block);
    let word = [stream[12], stream[13], stream[14], stream[15]];
    let incremented = increment_word(&word);
    stream[12..16].clone_from_slice(&incremented);
    stream_to_block(stream)
}

fn add_cipher(state: &Block, key: &Block) -> Block {
    let mut new_state: Block = [[0; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            new_state[i][j] = state[i][j] ^ key[i][j];
        }
    }

    new_state
}

/// Carry adder on 4-byte words
fn increment_word(word: &[u8; 4]) -> [u8; 4] {
    let mut incremented = [word[3], word[2], word[1], word[0]];
    let mut carry = 1;

    for i in 0..4 {
        if incremented[i] == 0xFF {
            incremented[i] = 0x00;
        } else {
            incremented[i] += carry;
            carry = 0;
        }
    }

    [
        incremented[3],
        incremented[2],
        incremented[1],
        incremented[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_word() {
        let word = [0x00, 0x00, 0x00, 0x00];
        let expected_incremented_word = [0x00, 0x00, 0x00, 0x01];
        assert_eq!(increment_word(&word), expected_incremented_word);

        let word = [0x00, 0x00, 0x00, 0xFF];
        let expected_incremented_word = [0x00, 0x00, 0x01, 0x00];
        assert_eq!(increment_word(&word), expected_incremented_word);

        let word = [0x00, 0x00, 0xFF, 0xFF];
        let expected_incremented_word = [0x00, 0x01, 0x00, 0x00];
        assert_eq!(increment_word(&word), expected_incremented_word);

        let word = [0xFF, 0xFF, 0xFF, 0xFF];
        let expected_incremented_word = [0x00, 0x00, 0x00, 0x00];
        assert_eq!(increment_word(&word), expected_incremented_word);
    }

    #[test]
    fn test_gctr() {
        let key: Stream = [
            0xca, 0xaa, 0x3f, 0x6f, 0xd3, 0x18, 0x22, 0xed, 0x2d, 0x21, 0x25, 0xf2, 0x25, 0xb0,
            0x16, 0x9f,
        ];
        let initial_counter_block: Block = [
            [0x7f, 0x48, 0x12, 0x00],
            [0x6d, 0x3e, 0xfa, 0x00],
            [0x90, 0x8c, 0x55, 0x00],
            [0x41, 0x14, 0x2a, 0x02],
        ];
        let plaintext: Stream = [
            0x84, 0xc9, 0x07, 0xb1, 0x1a, 0xe3, 0xb7, 0x9f, 0xc4, 0x45, 0x1d, 0x1b, 0xf1, 0x7f,
            0x4a, 0x99,
        ];
        let expected_ciphertext: Stream = [
            0xfd, 0xb4, 0xaa, 0xfa, 0x35, 0x19, 0xd3, 0xc0, 0x55, 0xbe, 0x8b, 0x34, 0x77, 0x64,
            0xea, 0x33,
        ];
        let ciphertext = gctr(key, initial_counter_block, plaintext);
        assert_eq!(ciphertext, expected_ciphertext);
    }
}
