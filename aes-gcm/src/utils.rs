pub type Stream = [u8; 16];
pub type Block = [[u8; 4]; 4];

pub(crate) fn stream_xor(stream1: Stream, stream2: Stream) -> Stream {
    let mut result: Stream = [0; 16];
    for i in 0..16 {
        result[i] = stream1[i] ^ stream2[i];
    }
    result
}

pub(crate) fn word_xor(word1: [u8; 4], word2: [u8; 4]) -> [u8; 4] {
    let mut result: [u8; 4] = [0; 4];
    for i in 0..4 {
        result[i] = word1[i] ^ word2[i];
    }
    result
}

pub(crate) fn stream_to_block(stream: Stream) -> Block {
    let mut block: Block = [[0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            block[j][i] = stream[i * 4 + j];
        }
    }
    block
}

pub(crate) fn block_to_stream(block: Block) -> Stream {
    let mut stream: Stream = [0; 16];
    for i in 0..4 {
        for j in 0..4 {
            stream[i * 4 + j] = block[j][i];
        }
    }
    stream
}

#[test]
fn test_stream_to_block() {
    let stream: Stream = [0x00; 16];
    let block: Block = [[0x00; 4]; 4];
    assert_eq!(stream_to_block(stream), block);
    assert_eq!(block_to_stream(block), stream);

    let stream1: Stream = [
        0x32, 0x88, 0x31, 0xe0, 0x43, 0x5a, 0x31, 0x37, 0xf6, 0x30, 0x98, 0x07, 0xa8, 0x8d, 0xa2,
        0x34,
    ];
    let block1: Block = [
        [0x32, 0x43, 0xf6, 0xa8],
        [0x88, 0x5a, 0x30, 0x8d],
        [0x31, 0x31, 0x98, 0xa2],
        [0xe0, 0x37, 0x07, 0x34],
    ];
    assert_eq!(stream_to_block(stream1), block1);
    assert_eq!(block_to_stream(block1), stream1);
}

pub(crate) fn bitwise_right_shift(bits: [u8; 128]) -> [u8; 128] {
    let mut result: [u8; 128] = [0; 128];
    for i in 1..128 {
        result[i] = bits[i - 1];
    }
    result
}

pub(crate) fn bits_to_stream(bits: [u8; 128]) -> Stream {
    let mut bytes: Stream = [0; 16];
    for i in 0..16 {
        let mut byte_bits: [u8; 8] = [0; 8];
        for j in 0..8 {
            byte_bits[j] = bits[i * 8 + j];
        }
        bytes[i] = bits_to_byte(byte_bits);
    }
    bytes
}

pub(crate) fn stream_to_bits(bytes: Stream) -> [u8; 128] {
    let mut bits: [u8; 128] = [0; 128];
    for i in 0..16 {
        let byte_bits = byte_to_bits(bytes[i]);
        for j in 0..8 {
            bits[i * 8 + j] = byte_bits[j];
        }
    }
    bits
}

pub(crate) fn byte_to_bits(byte: u8) -> [u8; 8] {
    [
        (byte >> 7) & 1,
        (byte >> 6) & 1,
        (byte >> 5) & 1,
        (byte >> 4) & 1,
        (byte >> 3) & 1,
        (byte >> 2) & 1,
        (byte >> 1) & 1,
        byte & 1,
    ]
}

pub(crate) fn bits_to_byte(bits: [u8; 8]) -> u8 {
    let mut byte = 0;
    for j in 0..8 {
        byte |= bits[j] << (7 - j);
    }
    byte
}

#[test]
fn test_bitwise_right_shift() {
    let input1: [u8; 128] = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    let expected_output1: [u8; 128] = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    assert_eq!(bitwise_right_shift(input1), expected_output1);

    let input2 = [0; 128];
    let expected_output2 = [0; 128];
    assert_eq!(bitwise_right_shift(input2), expected_output2);

    let input3 = [1; 128];
    let expected_output3 = [
        0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1,
    ];
    assert_eq!(bitwise_right_shift(input3), expected_output3);
}

#[test]
fn test_stream_to_bits() {
    assert_eq!(stream_to_bits([0x00; 16]), [0; 128]);
    assert_eq!(bits_to_stream([0; 128]), [0x00; 16]);

    let expected_output1 = [
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1,
    ];

    assert_eq!(bits_to_stream(expected_output1), [0x01; 16]);
    assert_eq!(stream_to_bits([0x01; 16]), expected_output1);

    let expected_output2 = [
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 1, 0,
    ];

    assert_eq!(bits_to_stream(expected_output2), [0x02; 16]);
    assert_eq!(stream_to_bits([0x02; 16]), expected_output2);
}

#[test]
fn test_byte_to_bits() {
    assert_eq!(byte_to_bits(0x00), [0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(byte_to_bits(0x01), [0, 0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(byte_to_bits(0xFF), [1, 1, 1, 1, 1, 1, 1, 1]);
    assert_eq!(byte_to_bits(0x80), [1, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(bits_to_byte([1, 0, 0, 0, 0, 0, 0, 0]), 0x80);
}
