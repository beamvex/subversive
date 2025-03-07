use serde::{Deserialize, Serialize};

/// Represents a message document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDoc {
    pub content: String,
    pub source: String,
    pub timestamp: i64,
}

/// Represents a peer document in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDoc {
    pub address: String,
    pub last_seen: i64,
}
