use crate::model::{block_header::BlockHeader, hash::Hash, transaction::Transaction};
use zerocopy::AsBytes;

pub struct BlockData {
    hash: Hash,
    header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl BlockData {
    pub fn new(hash: Hash, header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            hash,
            header,
            transactions,
        }
    }

    pub fn get_hash(&self) -> &Hash {
        &self.hash
    }

    pub fn get_header(&self) -> &BlockHeader {
        &self.header
    }

    pub fn get_transactions(&mut self) -> &mut Vec<Transaction> {
        &mut self.transactions
    }
}

impl From<&mut BlockData> for Vec<u8> {
    fn from(value: &mut BlockData) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(value.get_hash().as_bytes());
        result.extend_from_slice(value.get_header().as_bytes());
        result.extend_from_slice(value.get_transactions().as_bytes());
        result
    }
}
