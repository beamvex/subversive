use crate::{balance::BalanceManager, types::*, CurrencyError, Result};
use subversive_crypto::{address::Address, transaction::Transaction};
use std::sync::Arc;

pub struct TransactionProcessor {
    balance_manager: Arc<BalanceManager>,
}

impl TransactionProcessor {
    pub fn new(balance_manager: Arc<BalanceManager>) -> Self {
        Self { balance_manager }
    }

    pub async fn process_transaction(&self, transaction: &Transaction) -> Result<()> {
        let sender_address = Address::from_str(transaction.from()).map_err(|e| {
            CurrencyError::TransactionError(format!("Invalid sender address: {}", e))
        })?;
        
        let recipient_address = Address::from_str(transaction.outputs()[0].address()).map_err(|e| {
            CurrencyError::TransactionError(format!("Invalid recipient address: {}", e))
        })?;
        
        let amount = transaction.outputs()[0].amount();
        let sender_balance = self.balance_manager.get_balance(&sender_address).await;
        
        if sender_balance.amount.0 < amount {
            return Err(CurrencyError::InsufficientBalance);
        }

        // Update sender balance
        self.balance_manager
            .update_balance(
                sender_address,
                Amount(sender_balance.amount.0 - amount),
            )
            .await?;

        // Update recipient balance
        let recipient_balance = self.balance_manager.get_balance(&recipient_address).await;
        self.balance_manager
            .update_balance(
                recipient_address,
                Amount(recipient_balance.amount.0 + amount),
            )
            .await?;

        Ok(())
    }
}
