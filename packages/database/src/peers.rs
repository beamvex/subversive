use anyhow::Result;
use rusty_leveldb::{LdbIterator, DB};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a peer document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDoc {
    pub address: String,
    pub last_seen: i64,
}

/// Peer-related database operations
pub struct PeerStore {
    db: Arc<Mutex<DB>>,
}

impl PeerStore {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self { db }
    }

    /// Initializes the peers table in the database.
    pub async fn init_table(&self) -> Result<()> {
        // No initialization needed for LevelDB
        Ok(())
    }

    /// Saves a peer to the database.
    pub async fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        let mut db = self.db.lock().await;
        let key = format!("peer:{}", address).into_bytes();
        let peer = PeerDoc {
            address: address.to_string(),
            last_seen,
        };
        let value = serde_json::to_vec(&peer)?;
        db.put(&key, &value)?;
        Ok(())
    }

    /// Gets active peers from the database.
    pub async fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        let mut db = self.db.lock().await;
        let mut peers = Vec::new();
        let mut iter = db.new_iter()?;

        // Iterate through all peers
        iter.seek(b"peer:");

        while iter.advance() {
            let mut key = Vec::new();
            let mut value = Vec::new();
            iter.current(&mut key, &mut value);

            let key_str = String::from_utf8_lossy(&key);
            if !key_str.starts_with("peer:") {
                break;
            }

            let peer: PeerDoc = serde_json::from_slice(&value)?;
            if peer.last_seen > since {
                peers.push(peer);
            }
        }

        peers.sort_by_key(|p| std::cmp::Reverse(p.last_seen));
        Ok(peers)
    }

    /// Updates the last seen timestamp of a peer.
    pub async fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        let mut db = self.db.lock().await;
        let key = format!("peer:{}", address).into_bytes();
        let peer = PeerDoc {
            address: address.to_string(),
            last_seen: timestamp,
        };
        let value = serde_json::to_vec(&peer)?;
        db.put(&key, &value)?;
        Ok(())
    }
}
