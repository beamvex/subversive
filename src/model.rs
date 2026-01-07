#[path = "model/transaction.rs"]
pub mod transaction;

#[path = "model/block.rs"]
pub mod block;

pub use transaction::Transaction;
pub use block::Block;   