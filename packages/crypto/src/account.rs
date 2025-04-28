use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// The address associated with this account
    pub address: String,
    /// The current balance of the account
    pub balance: u64,
}

impl Account {
    /// Create a new account with the given address and initial balance
    pub fn new(address: String, initial_balance: u64) -> Self {
        Self {
            address,
            balance: initial_balance,
        }
    }

    /// Get the account's address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get the current balance
    pub fn balance(&self) -> u64 {
        self.balance
    }
}
