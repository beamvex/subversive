use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use crate::address::Address;
use crate::transaction::Transaction;

const PROCESSOR_FEE_PERCENTAGE: u64 = 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTransaction {
    /// The original transaction
    transaction: Transaction,
    /// Address of the processor
    processor: String,
    /// Processor's fee (10% of total transaction amount)
    fee: u64,
    /// Unix timestamp when the process transaction was created
    timestamp: u64,
    /// Processor's signature
    signature: Option<String>,
}

impl ProcessTransaction {
    pub fn new(transaction: Transaction, processor: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let fee = (transaction.total_amount() * PROCESSOR_FEE_PERCENTAGE) / 100;

        Self {
            transaction,
            processor: processor.to_string(),
            fee,
            timestamp,
            signature: None,
        }
    }

    /// Get the process transaction data that will be signed
    fn get_signing_data(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.transaction.get_signing_data(),
            self.processor,
            self.fee,
            self.timestamp,
        )
    }

    /// Sign the process transaction with the processor's address
    pub fn sign(&mut self, processor: &mut Address) -> Result<(), &'static str> {
        if self.signature.is_some() {
            return Err("Process transaction already signed");
        }

        if processor.get_public_address() != self.processor {
            return Err("Signer address doesn't match processor");
        }

        // Verify that the original transaction is signed
        if !self.transaction.verify() {
            return Err("Original transaction signature is invalid");
        }

        let signing_data = self.get_signing_data();
        let signature = processor.sign(&signing_data)?;
        self.signature = Some(signature);
        Ok(())
    }

    /// Verify both the original transaction and process transaction signatures
    pub fn verify(&self) -> bool {
        if let Some(signature) = &self.signature {
            // Verify original transaction first
            if !self.transaction.verify() {
                return false;
            }

            // Create a public-only address from the processor's address
            if let Ok(processor_address) = Address::from_public_address(&self.processor) {
                let signing_data = self.get_signing_data();
                return processor_address.verify(&signing_data, signature);
            }
        }
        false
    }

    // Getters
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    pub fn processor(&self) -> &str {
        &self.processor
    }

    pub fn fee(&self) -> u64 {
        self.fee
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn signature(&self) -> Option<&str> {
        self.signature.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::Output;
    use subversive_utils::test_utils::init_test_tracing;

    #[test]
    fn test_process_transaction() {
        init_test_tracing();

        // Create original transaction
        let mut sender = Address::new();
        let receiver = Address::new();
        let outputs = vec![Output::new(receiver.get_public_address(), 1000)];
        let mut tx = Transaction::new(sender.get_public_address(), outputs, None);
        assert!(tx.sign(&mut sender).is_ok());

        // Create and sign process transaction
        let mut processor = Address::new();
        let mut process_tx = ProcessTransaction::new(tx, processor.get_public_address());

        // Verify fee calculation
        assert_eq!(process_tx.fee(), 100); // 10% of 1000

        // Sign and verify
        assert!(process_tx.sign(&mut processor).is_ok());
        assert!(process_tx.verify());

        // Test validation rules
        let mut wrong_processor = Address::new();
        let mut process_tx2 = ProcessTransaction::new(
            process_tx.transaction().clone(),
            processor.get_public_address(),
        );
        assert!(process_tx2.sign(&mut wrong_processor).is_err());
    }
}
