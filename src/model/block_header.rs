use crate::model::hash::Hash;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct BlockHeader {
    version: [u8; 1],
    algo: [u8; 8],
    id: Hash,
    timestamp: [u8; 8],
    previous_hash: Hash,
}

impl BlockHeader {
    pub fn new(
        version: [u8; 1],
        algo: [u8; 8],
        id: Hash,
        timestamp: u64,
        previous_hash: Hash,
    ) -> Self {
        Self {
            version,
            algo,
            id,
            timestamp: timestamp.to_le_bytes(),
            previous_hash,
        }
    }

    pub fn get_timestamp(&self) -> u64 {
        u64::from_le_bytes(self.timestamp)
    }

    pub fn get_previous_hash(&self) -> &Hash {
        &self.previous_hash
    }

    pub fn get_id(&self) -> &Hash {
        &self.id
    }

    pub fn get_version(&self) -> &[u8; 1] {
        &self.version
    }

    pub fn get_algo(&self) -> &[u8; 8] {
        &self.algo
    }
}
