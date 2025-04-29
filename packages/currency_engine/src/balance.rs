use anyhow::Context;
use std::sync::Arc;
use subversive_crypto::{account::Account, address::Address};
use subversive_database::context::DbContext;

pub struct AccountManager {
    db: Arc<DbContext>,
}

impl AccountManager {
    pub fn new(db: Arc<DbContext>) -> Self {
        Self { db }
    }

    pub async fn get_account(&self, address: &Address) -> Result<Account, anyhow::Error> {
        let account = self
            .db
            .get_account(address.get_public_address().to_string())
            .await
            .context("Failed to get account from database")?
            .unwrap_or_else(|| Account::new(address.get_public_address().to_string(), 0));
        Ok(account)
    }

    pub async fn update_account(
        &self,
        address: &Address,
        amount: u64,
    ) -> Result<(), anyhow::Error> {
        let account = Account::new(address.get_public_address().to_string(), amount);
        self.db
            .save_account(&account)
            .await
            .context("Failed to save account to database")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use subversive_crypto::address::Address;
    use subversive_database::context::DbContext;
    use subversive_utils::test_utils::init_test_tracing;
    use tempfile::tempdir;
    use tracing::info;

    use crate::balance::AccountManager;

    #[tokio::test]
    async fn test_write_1000_accounts() {
        init_test_tracing();
        // Create a temporary directory for the database
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test.db");

        info!("test_write_1000_accounts {}", db_path.display());

        // Initialize database context
        let db = Arc::new(
            DbContext::new(&db_path)
                .await
                .expect("Failed to create database context"),
        );
        let account_manager = AccountManager::new(db);

        // Create and write 1000 accounts
        let mut addresses = Vec::with_capacity(1000);
        for recx in 0..1000 {
            if recx % 10 == 0 {
                info!("Creating account {}", recx);
            }
            let address = Address::new();
            addresses.push(address);
        }

        // Write accounts with random balances
        for address in &addresses {
            let balance = rand::random::<u64>() % 1_000; // Random balance between 0 and 999
            info!("Updating account balance {}", balance);
            account_manager
                .update_account(address, balance)
                .await
                .expect("Failed to update account");
        }

        // Verify all accounts were written correctly
        for (count, address) in addresses.iter().enumerate() {
            if count % 10 == 0 {
                info!("Verifying account {}", address.get_public_address());
            }
            let account = account_manager
                .get_account(address)
                .await
                .expect("Failed to get account");

            assert_eq!(account.address(), address.get_public_address());
        }
    }
}
