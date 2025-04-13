use anyhow::Result;
use rusqlite::Connection;
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
    conn: Arc<Mutex<Connection>>,
}

impl PeerStore {
    pub(crate) fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Saves a peer to the database.
    pub async fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO peers (address, last_seen) VALUES (?1, ?2)
             ON CONFLICT(address) DO UPDATE SET last_seen = ?2",
            [address, &last_seen.to_string()],
        )?;
        Ok(())
    }

    /// Gets active peers from the database.
    pub async fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT address, last_seen FROM peers WHERE last_seen > ?1 ORDER BY last_seen DESC",
        )?;
        let peers = stmt
            .query_map([since], |row| {
                Ok(PeerDoc {
                    address: row.get(0)?,
                    last_seen: row.get(1)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(peers)
    }

    /// Updates the last seen timestamp of a peer.
    pub async fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "UPDATE peers SET last_seen = ?1 WHERE address = ?2",
            [&timestamp.to_string(), address],
        )?;
        Ok(())
    }
}
