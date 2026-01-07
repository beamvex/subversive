use crate::model::address::Address;

pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    /// Unix timestamp in seconds
    pub timestamp: u64,
}
