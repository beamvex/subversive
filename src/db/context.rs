use anyhow::Result;
use polodb_bson::doc;
use polodb_core::Database;
use std::{fs, path::Path};
use std::sync::{Arc, Mutex};

use super::types::{MessageDoc, PeerDoc};

/// Represents a database context.
pub struct DbContext {
    db: Arc<Mutex<Database>>,
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
        let db = Database::open_file(db_path)?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }

    /// Saves a message to the database.
    pub fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        let mut db = self.db.lock().unwrap();
        let mut messages = db.collection("messages")?;
        let message = MessageDoc {
            content: content.to_string(),
            source: source.to_string(),
            timestamp,
        };
        messages.insert(&mut doc! {
            "content": message.content,
            "source": message.source,
            "timestamp": message.timestamp,
        })?;
        Ok(())
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

    /// Gets recent messages from the database.
    pub fn get_recent_messages(&self, limit: i64) -> Result<Vec<MessageDoc>> {
        let mut db = self.db.lock().unwrap();
        let mut messages = db.collection("messages")?;
        let filter = doc! {};
        let results = messages.find(&filter)?;
        let mut docs = results
            .into_iter()
            .map(|doc| MessageDoc {
                content: doc
                    .get("content")
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
                source: doc.get("source").map(|v| v.to_string()).unwrap_or_default(),
                timestamp: doc
                    .get("timestamp")
                    .and_then(|v| v.to_string().parse().ok())
                    .unwrap_or_default(),
            })
            .collect::<Vec<_>>();
        docs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        if docs.len() > limit as usize {
            docs.truncate(limit as usize);
        }
        Ok(docs)
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
