use anyhow::Result;
use polodb_bson::doc;
use polodb_core::Database;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Represents a message document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDoc {
    pub content: String,
    pub source: String,
    pub timestamp: i64,
}

/// Message-related database operations
pub struct MessageStore {
    db: Arc<Mutex<Database>>,
}

impl MessageStore {
    pub(crate) fn new(db: Arc<Mutex<Database>>) -> Self {
        Self { db }
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
}
