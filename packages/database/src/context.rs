use anyhow::Result;
use rusqlite::{Connection, OpenFlags};
use std::sync::Arc;
use std::{fs, path::Path};
use tokio::sync::Mutex;

use super::accounts::AccountStore;
use super::messages::MessageStore;
use super::peers::PeerStore;
use super::{MessageDoc, PeerDoc};
use subversive_crypto::account::Account;

/// Represents a database context.
pub struct DbContext {
    pub messages: MessageStore,
    pub peers: PeerStore,
    pub accounts: AccountStore,
}

impl DbContext {
    /// Creates a new database context from a file path.
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Create db directory if it doesn't exist
        let db_dir = Path::new("db");
        if !db_dir.exists() {
            fs::create_dir_all(db_dir)?;
        }

        // Create full path in db directory
        let db_path = db_dir.join(path.as_ref());
        let conn = Arc::new(Mutex::new(Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )?));

        Self::init_tables(&conn).await?;

        Ok(Self {
            messages: MessageStore::new(conn.clone()),
            peers: PeerStore::new(conn.clone()),
            accounts: AccountStore::new(conn),
        })
    }

    /// Creates a new database context in memory.
    pub async fn new_memory() -> Result<Self> {
        let conn = Arc::new(Mutex::new(Connection::open_in_memory()?));
        Self::init_tables(&conn).await?;

        Ok(Self {
            messages: MessageStore::new(conn.clone()),
            peers: PeerStore::new(conn.clone()),
            accounts: AccountStore::new(conn),
        })
    }

    /// Initialize database tables.
    async fn init_tables(conn: &Arc<Mutex<Connection>>) -> Result<()> {
        let message_store = MessageStore::new(conn.clone());
        message_store.init_table().await?;

        let peer_store = PeerStore::new(conn.clone());
        peer_store.init_table().await?;

        let account_store = AccountStore::new(conn.clone());
        account_store.init_table().await?;

        Ok(())
    }

    /// Saves a message to the database.
    pub async fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        self.messages.save_message(content, source, timestamp).await
    }

    /// Saves a peer to the database.
    pub async fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        self.peers.save_peer(address, last_seen).await
    }

    /// Gets messages since a certain timestamp from the database.
    pub async fn get_messages_since(&self, since: i64) -> Result<Vec<MessageDoc>> {
        self.messages.get_messages_since(since).await
    }

    /// Gets active peers from the database.
    pub async fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        self.peers.get_active_peers(since).await
    }

    /// Updates the last seen timestamp of a peer.
    pub async fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        self.peers.update_peer_last_seen(address, timestamp).await
    }

    /// Gets an account from the database.
    pub async fn get_account(&self, address: String) -> Result<Option<Account>> {
        self.accounts.get_account(address).await
    }

    /// Saves an account to the database.
    pub async fn save_account(&self, account: &Account) -> Result<()> {
        self.accounts.save_account(account).await
    }
}
