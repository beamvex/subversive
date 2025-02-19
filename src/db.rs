use anyhow::Result;
use polodb_core::{Database, db::Collection, DbResult};
use polodb_bson::{doc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};


#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDoc {
    pub content: String,
    pub source: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDoc {
    pub address: String,
    pub last_seen: i64,
}

pub struct DbContext {
    db: Arc<Mutex<Database>>,
}

impl DbContext {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = Database::open_file(path)?;
        Ok(Self { 
            db: Arc::new(Mutex::new(db))
        })
    }

    pub fn get_messages(&self) -> DbResult<Collection> {
        self.db.lock().unwrap().collection("messages")
    }

    pub fn get_peers(&self) -> DbResult<Collection> {
        self.db.lock().unwrap().collection("peers")
    }

    pub fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        let mut messages = self.get_messages()?;
        let message = MessageDoc {
            content: content.to_string(),
            source: source.to_string(),
            timestamp,
        };
        messages.insert(&doc! {
            "content": message.content,
            "source": message.source,
            "timestamp": message.timestamp,
        })?;
        Ok(())
    }

    pub fn save_peer(&self, address: &str, last_seen: i64) -> Result<()> {
        let mut peers = self.get_peers()?;
        let peer = PeerDoc {
            address: address.to_string(),
            last_seen,
        };
        peers.insert(&doc! {
            "address": peer.address,
            "last_seen": peer.last_seen,
        })?;
        Ok(())
    }

    pub fn get_recent_messages(&self, limit: i64) -> Result<Vec<MessageDoc>> {
        let mut messages = self.get_messages()?;
        let filter = doc! {};
        let results = messages.find(&filter)?;
        let mut docs = results.into_iter()
            .map(|doc| MessageDoc {
                content: doc.get_str("content").unwrap_or_default().to_string(),
                source: doc.get_str("source").unwrap_or_default().to_string(),
                timestamp: doc.get_i64("timestamp").unwrap_or_default(),
            })
            .collect::<Vec<_>>();
        docs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        if docs.len() > limit as usize {
            docs.truncate(limit as usize);
        }
        Ok(docs)
    }

    pub fn get_active_peers(&self, since: i64) -> Result<Vec<PeerDoc>> {
        let mut peers = self.get_peers()?;
        let filter = doc! {
            "last_seen": doc! { "$gt": since }
        };
        let results = peers.find(&filter)?;
        Ok(results.into_iter()
            .map(|doc| PeerDoc {
                address: doc.get_str("address").unwrap_or_default().to_string(),
                last_seen: doc.get_i64("last_seen").unwrap_or_default(),
            })
            .collect())
    }

    pub fn update_peer_last_seen(&self, address: &str, timestamp: i64) -> Result<()> {
        let mut peers = self.get_peers()?;
        let filter = doc! { 
            "address": address.to_string() 
        };
        let update = doc! {
            "address": address.to_string(),
            "last_seen": timestamp,
        };
        peers.update(&filter, &update)?;
        Ok(())
    }
}
