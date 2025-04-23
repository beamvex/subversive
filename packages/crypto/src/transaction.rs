use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::address::Address;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Sender's address
    from: String,
    /// List of outputs (recipient and amount pairs)
    outputs: Vec<Output>,
    /// Unix timestamp when the transaction was created
    timestamp: u64,
    /// Optional message/memo
    memo: Option<String>,
    /// Transaction signature
    signature: Option<String>,
}

impl Transaction {
    pub fn new(from: &str, outputs: Vec<Output>, memo: Option<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            from: from.to_string(),
            outputs,
            timestamp,
            memo,
            signature: None,
        }
    }

    /// Get the total amount of the transaction
    pub fn total_amount(&self) -> u64 {
        self.outputs.iter().map(|output| output.amount).sum()
    }

    /// Get the transaction data that will be signed
    fn get_signing_data(&self) -> String {
        let outputs_data: Vec<String> = self.outputs
            .iter()
            .map(|output| format!("{}:{}", output.to, output.amount))
            .collect();
        
        format!(
            "{}:{}:{}:{}",
            self.from,
            outputs_data.join(","),
            self.timestamp,
            self.memo.as_deref().unwrap_or("")
        )
    }

    /// Sign the transaction with the sender's address
    pub fn sign(&mut self, signer: &mut Address) -> Result<(), &'static str> {
        if self.signature.is_some() {
            return Err("Transaction already signed");
        }

        if signer.get_public_address() != self.from {
            return Err("Signer address doesn't match sender");
        }

        let signing_data = self.get_signing_data();
        let signature = signer.sign(&signing_data)?;
        self.signature = Some(signature);
        Ok(())
    }

    /// Verify the transaction signature
    pub fn verify(&self) -> bool {
        if let Some(signature) = &self.signature {
            // Create a public-only address from the sender's address
            if let Ok(from_address) = Address::from_public_address(&self.from) {
                let signing_data = self.get_signing_data();
                return from_address.verify(&signing_data, signature);
            }
        }
        false
    }

    // Getters
    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn outputs(&self) -> &[Output] {
        &self.outputs
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn memo(&self) -> Option<&str> {
        self.memo.as_deref()
    }

    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::address::Address;
    use subversive_utils::test_utils::init_test_tracing;
    use tracing::info;

    #[test]
    fn test_transaction_signing_and_verification() {
        init_test_tracing();

        // Create sender and receivers
        let mut sender = Address::new();
        let receiver1 = Address::new();
        let receiver2 = Address::new();

        // Create outputs
        let outputs = vec![
            Output::new(receiver1.get_public_address(), 1000),
            Output::new(receiver2.get_public_address(), 500),
        ];

        // Create a transaction with multiple outputs
        let mut tx = Transaction::new(
            sender.get_public_address(),
            outputs,
            Some("Multi-output transfer".to_string()),
        );
        info!("Created transaction: {:?}", tx);
        assert_eq!(tx.total_amount(), 1500);

        // Sign the transaction
        assert!(tx.sign(&mut sender).is_ok());
        info!("Signed transaction: {:?}", tx);

        // Verify the signature
        assert!(tx.verify());

        // Verify that tampering with any output invalidates the signature
        let mut tampered_tx = tx.clone();
        tampered_tx.outputs[0].amount = 2000;
        assert!(!tampered_tx.verify());
    }

    #[test]
    fn test_transaction_validation() {
        init_test_tracing();

        let mut sender = Address::new();
        let receiver = Address::new();
        let mut wrong_signer = Address::new();

        let outputs = vec![Output::new(receiver.get_public_address(), 1000)];

        let mut tx = Transaction::new(
            sender.get_public_address(),
            outputs,
            None,
        );

        // Test signing with wrong address
        assert!(tx.sign(&mut wrong_signer).is_err());

        // Test double signing
        assert!(tx.sign(&mut sender).is_ok());
        assert!(tx.sign(&mut sender).is_err());
    }
}
