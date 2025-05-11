use anyhow::Result;
use rusty_leveldb::DB;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a message document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDoc {
    pub content: String,
    pub source: String,
    pub timestamp: i64,
}

/// Store for managing messages in the database
pub struct MessageStore {
    db: Arc<Mutex<DB>>,
}

impl MessageStore {
    pub fn new(db: Arc<Mutex<DB>>) -> Self {
        Self { db }
    }

    /// Save a message to the database
    pub async fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        let mut db = self.db.lock().await;
        let key = format!("message:{}", timestamp).into_bytes();
        let message = MessageDoc {
            content: content.to_string(),
            source: source.to_string(),
            timestamp,
        };
        let value = serde_json::to_vec(&message)?;
        db.put(&key, &value)?;
        Ok(())
    }

    /// Get messages since a given timestamp
    pub async fn get_messages(&self, since: i64) -> Result<Vec<MessageDoc>> {
        let mut db = self.db.lock().await;
        let mut messages = Vec::new();
        let mut iter = db.new_iter()?;
        
        // Seek to the first message after the timestamp
        let seek_key = format!("message:{}", since).into_bytes();
        iter.seek(&seek_key);
        
        while iter.advance() {
            let mut key = Vec::new();
            let mut value = Vec::new();
            iter.current(&mut key, &mut value);
            
            let key_str = String::from_utf8_lossy(&key);
            if !key_str.starts_with("message:") {
                continue;
            }
            
            let message: MessageDoc = serde_json::from_slice(&value)?;
            messages.push(message);
        }
        
        messages.sort_by_key(|m| std::cmp::Reverse(m.timestamp));
        Ok(messages)
    }
}
