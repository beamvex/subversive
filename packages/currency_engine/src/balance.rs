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
        let account = self.db
            .get_account(address.get_public_address().to_string())
            .await
            .context("Failed to get account from database")?
            .unwrap_or_else(|| Account::new(address.get_public_address().to_string(), 0));
        Ok(account)
    }

    pub async fn update_account(&self, address: &Address, amount: u64) -> Result<(), anyhow::Error> {
        let account = Account::new(address.get_public_address().to_string(), amount);
        self.db
            .save_account(&account)
            .await
            .context("Failed to save account to database")?;
        Ok(())
    }
}
