use crate::model::transaction::Transaction;

pub struct Block {
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}