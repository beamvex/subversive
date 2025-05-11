use anyhow::Result;
use rusty_leveldb::DB;
use serde_json;
use std::sync::Arc;
use subversive_crypto::account::Account;
use tokio::sync::Mutex;

pub struct AccountStore {
    db: Arc<Mutex<DB>>,
}

impl AccountStore {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self { db }
    }

    pub async fn save_account(&self, account: &Account) -> Result<()> {
        let mut db = self.db.lock().await;
        let key = format!("account:{}", account.address).into_bytes();
        let value = serde_json::to_vec(account)?;
        db.put(&key, &value)?;
        Ok(())
    }

    pub async fn get_account(&self, address: String) -> Result<Option<Account>> {
        let mut db = self.db.lock().await;
        let key = format!("account:{}", address).into_bytes();

        if let Some(value) = db.get(&key) {
            let account: Account = serde_json::from_slice(&value)?;
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }
}
