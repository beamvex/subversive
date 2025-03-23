use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

/// Represents a peer document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDoc {
    pub address: String,
    pub last_seen: i64,
}

/// Peer-related database operations
pub struct PeerStore {
    db: Arc<Surreal<Any>>,
}

impl PeerStore {
    pub(crate) fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }

    /// Saves a peer to the database.
    pub async fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        let peer = PeerDoc {
            address: address.to_string(),
            last_seen,
        };
        self.db.create(("peers", address)).content(peer).await?;
        Ok(())
    }

    /// Gets active peers from the database.
    pub async fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        let peers: Vec<PeerDoc> = self.db
            .query("SELECT * FROM peers WHERE last_seen > $since")
            .bind(("since", since))
            .await?
            .take(0)?;
        Ok(peers)
    }

    /// Updates the last seen timestamp of a peer.
    pub async fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        self.db
            .query("UPDATE peers SET last_seen = $timestamp WHERE address = $address")
            .bind(("timestamp", timestamp))
            .bind(("address", address))
            .await?;
        Ok(())
    }
}
