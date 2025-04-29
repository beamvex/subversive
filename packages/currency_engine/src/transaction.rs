use crate::{balance::AccountManager, CurrencyError, Result};
use std::sync::Arc;
use subversive_crypto::{address::Address, transaction::Transaction};

pub struct TransactionProcessor {
    account_manager: Arc<AccountManager>,
}

impl TransactionProcessor {
    pub fn new(account_manager: Arc<AccountManager>) -> Self {
        Self { account_manager }
    }

    pub async fn process_transaction(&self, transaction: &Transaction) -> Result<()> {
        let sender_address = Address::from_public_address(transaction.from()).map_err(|e| {
            CurrencyError::TransactionError(format!("Invalid sender address: {}", e))
        })?;
        
        let recipient_address = Address::from_public_address(transaction.outputs()[0].to()).map_err(|e| {
            CurrencyError::TransactionError(format!("Invalid recipient address: {}", e))
        })?;
        
        let amount = transaction.outputs()[0].amount();
        let sender_account = self.account_manager.get_account(&sender_address).await
            .map_err(|e| CurrencyError::TransactionError(format!("Failed to get sender account: {}", e)))?;
        
        if sender_account.balance < amount {
            return Err(CurrencyError::InsufficientBalance);
        }

        // Update sender balance
        self.account_manager
            .update_account(&sender_address, sender_account.balance - amount)
            .await
            .map_err(|e| CurrencyError::TransactionError(format!("Failed to update sender account: {}", e)))?;

        // Update recipient balance
        let recipient_account = self.account_manager.get_account(&recipient_address).await
            .map_err(|e| CurrencyError::TransactionError(format!("Failed to get recipient account: {}", e)))?;
        
        self.account_manager
            .update_account(&recipient_address, recipient_account.balance + amount)
            .await
            .map_err(|e| CurrencyError::TransactionError(format!("Failed to update recipient account: {}", e)))?;

        Ok(())
    }
}
