use crate::model::{block_header::BlockHeader, hash::Hash, transaction::Transaction};

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
