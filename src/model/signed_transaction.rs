
use crate::model::{signature::Signature, transaction::Transaction};

pub struct SignedTransaction {
    pub transaction: Transaction,
    pub signature: Signature,
}