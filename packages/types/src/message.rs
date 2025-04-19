use serde::{Deserialize, Serialize};

/// Message types that can be exchanged between peers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    /// Regular chat message
    Chat { content: String },
    /// Message indicating a new peer has joined
    NewPeer { addr: String },
}

/// Message containing chat content
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Content of the chat message
    pub content: String,
}

/// Message for peer heartbeat
#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatMessage {
    /// Address of the peer sending the heartbeat
    pub address: String,
    /// List of known peer addresses
    pub known_peers: Vec<String>,
}
