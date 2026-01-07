use crate::model::{signed_transaction::SignedTransaction, transaction::Transaction};

pub struct Block {
    timestamp: u64,
    transactions: Vec<SignedTransaction>,
    previous_hash: String,
    hash: String,
}