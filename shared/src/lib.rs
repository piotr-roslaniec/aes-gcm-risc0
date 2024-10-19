pub use aes_gcm_mini::{Block, Stream};

use aes_gcm_mini::{aes, aes_gcm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct AesTestCase {
    pub block: Block,
    pub key: Stream,
    pub cipher: Block,
}

impl AesTestCase {
    pub fn default_case() -> Self {
        let block: Block = [
            [0x32, 0x88, 0x31, 0xe0],
            [0x43, 0x5a, 0x31, 0x37],
            [0xf6, 0x30, 0x98, 0x07],
            [0xa8, 0x8d, 0xa2, 0x34],
        ];
        let key: Stream = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let cipher: Block = [
            [0x39, 0x02, 0xdc, 0x19],
            [0x25, 0xdc, 0x11, 0x6a],
            [0x84, 0x09, 0x85, 0x0b],
            [0x1d, 0xfb, 0x97, 0x32],
        ];
        Self { block, cipher, key }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        aes(self.block, self.key) == self.cipher
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct AesGcmTestCase {
    pub key: Stream,
    pub iv: [u8; 12],
    pub aad: Stream,
    pub plaintext: Stream,
    expected_output: Stream,
}

impl AesGcmTestCase {
    pub fn default_case() -> Self {
        let key = [0x00u8; 16]; // 128-bit key
        let plaintext = [0x00u8; 16]; // Example plaintext
        let iv = [0x00u8; 12]; // 96-bit IV
        let aad = [0x00u8; 16]; // Example AAD
        let expected_output = [
            0x03, 0x88, 0xda, 0xce, 0x60, 0xb6, 0xa3, 0x92, 0xf3, 0x28, 0xc2, 0xb9, 0x71, 0xb2,
            0xfe, 0x78,
        ];
        Self {
            key,
            plaintext,
            iv,
            aad,
            expected_output,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        let (ciphertext, _tag) = aes_gcm(self.key, self.plaintext, self.iv, self.aad);
        ciphertext == self.expected_output
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct TestCase(pub String, pub Vec<u8>);

impl TestCase {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_aes() {
        let aes_test_case = AesTestCase::default_case();
        assert!(aes_test_case.is_valid());

        let serialized = aes_test_case.to_bytes();
        let deserialized = AesTestCase::from_bytes(&serialized);
        assert_eq!(aes_test_case, deserialized);
    }

    #[test]
    fn test_aes_gcm() {
        let aes_gcm_test_case = AesGcmTestCase::default_case();
        assert!(aes_gcm_test_case.is_valid());

        let serialized = aes_gcm_test_case.to_bytes();
        let deserialized = AesGcmTestCase::from_bytes(&serialized);
        assert_eq!(aes_gcm_test_case, deserialized);
    }
}
