use anyhow::Result;
use polodb_bson::doc;
use polodb_core::Database;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Represents a peer document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDoc {
    pub address: String,
    pub last_seen: i64,
}

/// Peer-related database operations
pub struct PeerStore {
    db: Arc<Mutex<Database>>,
}

impl PeerStore {
    pub(crate) fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
    }

    /// Saves a peer to the database.
    pub fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        let mut peers = db.collection("peers")?;
        let filter = doc! { "address": address };
        let update = doc! {
            "$set": doc!{
                "address": address.to_string(),
                "last_seen": last_seen,
            }
        };
        // Upsert - update if exists, insert if not
        peers.update(Some(&filter), &update)?;
        Ok(())
    }

    /// Gets active peers from the database.
    pub fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        let mut db = self.db.lock().unwrap();
        let mut peers = db.collection("peers")?;
        let filter = doc! {
            "last_seen": doc! {
                "$gt": since
            }
        }; // Get all peers
        let results = peers.find(&filter)?;
        Ok(results
            .into_iter()
            .map(|doc| PeerDoc {
                address: doc
                    .get("address")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                last_seen: doc
                    .get("last_seen")
                    .and_then(|v| v.to_string().parse().ok())
                    .unwrap_or_default(),
            })
            .filter(|peer| peer.last_seen > since)
            .collect())
    }

    /// Updates the last seen timestamp of a peer.
    pub fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        let mut peers = db.collection("peers")?;
        let filter = doc! { "address": address };
        let update = doc! {
            "$set": doc! {
                "last_seen": timestamp
            }
        };
        peers.update(Some(&filter), &update)?;
        Ok(())
    }
}
