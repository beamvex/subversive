/*
#[path = "model/transaction.rs"]
pub mod transaction;

#[path = "model/block.rs"]
pub mod block;

#[path = "model/signature.rs"]
pub mod signature;

#[path = "model/signed_transaction.rs"]
pub mod signed_transaction;

#[path = "model/signed_block.rs"]
pub mod signed_block;


#[path = "model/private_address.rs"]
pub mod private_address;
*/
#[path = "model/address.rs"]
pub mod address;

/*
pub use transaction::Transaction;
pub use block::Block;
pub use signature::Signature;
pub use signed_transaction::SignedTransaction;
pub use signed_block::SignedBlock;
*/

pub use address::Address;
//pub use private_address::PrivateAddress;
