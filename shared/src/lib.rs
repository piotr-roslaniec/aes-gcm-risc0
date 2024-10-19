pub use aes_gcm_mini::{Block, Stream};

use aes_gcm_mini::aes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct Inputs {
    pub block: Block,
    pub key: Stream,
    pub cipher: Block,
}

impl Inputs {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}

impl Inputs {
    pub fn is_valid(&self) -> bool {
        aes(self.block, self.key) == self.cipher
    }
}

impl Inputs {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}
