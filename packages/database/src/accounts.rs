use anyhow::Result;
use rusqlite::Connection;
use std::sync::Arc;
use subversive_crypto::account::Account;
use tokio::sync::Mutex;

pub struct AccountStore {
    conn: Arc<Mutex<Connection>>,
}

impl AccountStore {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub async fn init_table(&self) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS accounts (
                address TEXT PRIMARY KEY,
                balance INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub async fn save_account(&self, account: &Account) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO accounts (address, balance) VALUES (?1, ?2)",
            [&account.address, &account.balance.to_string()],
        )?;
        Ok(())
    }

    pub async fn get_account(&self, address: String) -> Result<Option<Account>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT address, balance FROM accounts WHERE address = ?1")?;
        let mut rows = stmt.query([address])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Account::new(
                row.get(0)?,
                row.get::<_, i64>(1)? as u64,
            )))
        } else {
            Ok(None)
        }
    }
}
