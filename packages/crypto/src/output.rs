use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    /// Recipient's address
    to: String,
    /// Amount to transfer
    amount: u64,
}

impl Output {
    pub fn new(to: &str, amount: u64) -> Self {
        Self {
            to: to.to_string(),
            amount,
        }
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn amount(&self) -> u64 {
        self.amount
    }
}
