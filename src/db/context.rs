use anyhow::Result;
use polodb_core::Database;
use std::sync::{Arc, Mutex};
use std::{fs, path::Path};

use super::messages::MessageStore;
use super::peers::PeerStore;
use super::{MessageDoc, PeerDoc};

/// Represents a database context.
pub struct DbContext {
    pub messages: MessageStore,
    pub peers: PeerStore,
}

impl DbContext {
    /// Creates a new database context from a file path.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Ensure db directory exists
        let db_dir = Path::new("db");
        if !db_dir.exists() {
            fs::create_dir_all(db_dir)?;
        }

        // Create full path in db directory
        let db_path = db_dir.join(path.as_ref());
        let db = Arc::new(Mutex::new(Database::open_file(db_path)?));

        Ok(Self {
            messages: MessageStore::new(db.clone()),
            peers: PeerStore::new(db.clone()),
        })
    }

    /// Saves a message to the database.
    pub fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        self.messages.save_message(content, source, timestamp)
    }

    /// Saves a peer to the database.
    pub fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        self.peers.save_peer(address, last_seen)
    }

    /// Gets recent messages from the database.
    pub fn get_recent_messages(&self, limit: i64) -> Result<Vec<MessageDoc>> {
        self.messages.get_recent_messages(limit)
    }

    /// Gets active peers from the database.
    pub fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        self.peers.get_active_peers(since)
    }

    /// Updates the last seen timestamp of a peer.
    pub fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        self.peers.update_peer_last_seen(address, timestamp)
    }
}
