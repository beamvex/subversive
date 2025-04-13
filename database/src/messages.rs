use anyhow::Result;
use rusqlite::Connection;
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
    conn: Arc<Mutex<Connection>>,
}

impl MessageStore {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// Save a message to the database
    pub async fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT INTO messages (content, source, timestamp) VALUES (?1, ?2, ?3)",
            [content, source, &timestamp.to_string()],
        )?;
        Ok(())
    }

    /// Get messages since a given timestamp
    pub async fn get_messages_since(&self, since: i64) -> Result<Vec<MessageDoc>> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT content, source, timestamp FROM messages WHERE timestamp > ?1 ORDER BY timestamp DESC",
        )?;
        let messages = stmt
            .query_map([since], |row| {
                Ok(MessageDoc {
                    content: row.get(0)?,
                    source: row.get(1)?,
                    timestamp: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(messages)
    }
}
