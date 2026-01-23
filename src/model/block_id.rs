use crate::model::hash::Hash;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[repr(C)]
#[derive(Debug, FromZeroes, FromBytes, AsBytes, Default)]
pub struct BlockId {
    hash: Hash,
    ts: u64,
}

impl BlockId {
    pub fn new(hash: Hash, ts: u64) -> Self {
        Self { hash, ts }
    }

    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }
}

impl From<&BlockId> for Vec<u8> {
    fn from(value: &BlockId) -> Vec<u8> {
        value.as_bytes().to_vec()
    }
}
