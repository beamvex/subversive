use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

/// Represents a message document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDoc {
    pub content: String,
    pub source: String,
    pub timestamp: i64,
}

/// Message-related database operations
pub struct MessageStore {
    db: Arc<Surreal<Any>>,
}

impl MessageStore {
    pub(crate) fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }

    /// Saves a message to the database.
    pub async fn save_message(&self, content: &str, source: &str, timestamp: i64) -> Result<()> {
        let message = MessageDoc {
            content: content.to_string(),
            source: source.to_string(),
            timestamp,
        };
        self.db.create(("messages", timestamp.to_string())).content(message).await?;
        Ok(())
    }

    /// Gets recent messages from the database.
    pub async fn get_recent_messages(&self, limit: i64) -> Result<Vec<MessageDoc>> {
        let mut messages: Vec<MessageDoc> = self.db
            .query("SELECT * FROM messages ORDER BY timestamp DESC LIMIT $limit")
            .bind(("limit", limit))
            .await?
            .take(0)?;
        Ok(messages)
    }
}
