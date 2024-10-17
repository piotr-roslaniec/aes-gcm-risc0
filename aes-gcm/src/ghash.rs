use crate::utils::{bits_to_stream, bitwise_right_shift, stream_to_bits, stream_xor, Stream};

// GHASH computes the authentication tag for AES-GCM.
// Inputs:
// - `hash_key`: the hash key
// - `message`: the input blocks
//
// Outputs:
// - `tag`: the authentication tag
//
// Computes:
// Y_0 = 0^128
// Y_{i+1} = (Y_i xor X_{i-1}) * H
// output: Y_{n+1} where n is the number of blocks.
// GHASH Process
//
//           X1                      X2          ...          XM
//           │                       │                        │
//           │                       ▼                        ▼
//           │                  ┌──────────┐             ┌──────────┐
//           │           ┌─────▶│   XOR    │      ┌─────▶│   XOR    │
//           │           │      └────┬─────┘      │      └────┬─────┘
//           │           │           │            │           │
//           ▼           │           ▼            │           ▼
//  ┌────────────────┐   │   ┌────────────────┐   │   ┌────────────────┐
//  │ multiply by H  │   │   │ multiply by H  │   │   │ multiply by H  │
//  └────────┬───────┘   │   └───────┬────────┘   │   └───────┬────────┘
//           │           │           │            │           |
//           ▼           │           ▼            │           ▼
//      ┌─────────┐      │      ┌─────────┐       │      ┌─────────┐
//      │  TAG1   │ ─────┘      │   TAG2  │ ──────┘      │   TAGM  │
//      └─────────┘             └─────────┘              └─────────┘

// TODO: handle arbitrary number of blocks
pub fn ghash<const N: usize>(hash_key: Stream, message: [Stream; N]) -> Stream {
    let mut tag: Stream = [0; 16];
    for i in 0..N {
        tag = stream_xor(tag, message[i]);
        tag = gmul(hash_key, tag);
    }
    tag
}

#[test]
fn ghash_01() {
    // https://datatracker.ietf.org/doc/html/rfc8452#appendix-A
    let hash_key: Stream = [
        0x25, 0x62, 0x93, 0x47, 0x58, 0x92, 0x42, 0x76, 0x1d, 0x31, 0xf8, 0x26, 0xba, 0x4b, 0x75,
        0x7b,
    ];
    let message: [Stream; 2] = [
        [
            0x4f, 0x4f, 0x95, 0x66, 0x8c, 0x83, 0xdf, 0xb6, 0x40, 0x17, 0x62, 0xbb, 0x2d, 0x01,
            0xa2, 0x62,
        ],
        [
            0xd1, 0xa2, 0x4d, 0xdd, 0x27, 0x21, 0xd0, 0x06, 0xbb, 0xe4, 0x5f, 0x20, 0xd3, 0xc9,
            0xf3, 0x62,
        ],
    ];
    let expected: Stream = [
        0xbd, 0x9b, 0x39, 0x97, 0x04, 0x67, 0x31, 0xfb, 0x96, 0x25, 0x1b, 0x91, 0xf9, 0xc9, 0x9d,
        0x7a,
    ];

    assert_eq!(ghash(hash_key, message), expected);
}

fn gmul(x: Stream, y: Stream) -> Stream {
    let mut result: Stream = [0; 16];
    let mut accumulator: Stream = y;
    let x_bits = stream_to_bits(x);

    for i in 0..128 {
        if x_bits[i] == 1 {
            result = stream_xor(result, accumulator);
        }
        accumulator = mulx(accumulator);
    }

    result
}

#[test]
fn gmul00() {
    let x: Stream = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let y: Stream = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    let expected: Stream = [
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    assert_eq!(gmul(x, y), expected);
}

#[test]
fn gmul01() {
    let x: Stream = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ];
    let y: Stream = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ];
    let expected: Stream = [
        0xe6, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x03,
    ];
    assert_eq!(gmul(x, y), expected);
}

#[test]
fn gmul02() {
    let x: Stream = [
        0xaa, 0xe0, 0x69, 0x92, 0xac, 0xbf, 0x52, 0xa3, 0xe8, 0xf4, 0xa9, 0x6e, 0xc9, 0x30, 0x0b,
        0xd7,
    ];
    let y: Stream = [
        0x98, 0xe7, 0x24, 0x7c, 0x07, 0xf0, 0xfe, 0x41, 0x1c, 0x26, 0x7e, 0x43, 0x84, 0xb0, 0xf6,
        0x00,
    ];
    let expected: Stream = [
        0x90, 0xe8, 0x73, 0x15, 0xfb, 0x7d, 0x4e, 0x1b, 0x40, 0x92, 0xec, 0x0c, 0xbf, 0xda, 0x5d,
        0x7d,
    ];
    assert_eq!(gmul(x, y), expected);
}

/// Multiplication of the binary extension field by x
/// right shifts a block by one bit
/// if the msb is one, then the 8 most LSBs are XORed with 0xE1
/// this 0xE1 is the hex representation of 11100001, which represents
/// the polynomial 1 + x + x^2 + x^7
fn mulx(block: Stream) -> Stream {
    let bits = stream_to_bits(block);
    let result_bits = bitwise_right_shift(bits);
    let mut bytes = bits_to_stream(result_bits);

    if bits[127] == 1 {
        bytes[0] ^= 0xE1;
    }

    bytes
}

#[test]
fn test_mulx() {
    let input: Stream = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ];
    let expected: Stream = [
        0xE1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];
    assert_eq!(mulx(input), expected);
}
