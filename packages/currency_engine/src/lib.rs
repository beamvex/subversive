use thiserror::Error;

pub mod balance;
pub mod transaction;
pub mod types;

#[derive(Error, Debug)]
pub enum CurrencyError {
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub type Result<T> = std::result::Result<T, CurrencyError>;
