use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

use crate::db::DbContext;

/// Command line arguments for the P2P network application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on for P2P communication (defaults to random port between 10000-65535)
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Initial peer to connect to
    #[arg(short('e'), long)]
    pub peer: Option<String>,

    /// Database file name (defaults to p2p_network.db)
    #[arg(short, long, default_value = "p2p_network.db")]
    pub database: String,

    /// Custom name for HTTP access logs
    #[arg(short, long, default_value = "p2p_network")]
    pub name: String,
}

/// Message types that can be exchanged between peers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    /// Regular chat message
    Chat { content: String },
    /// Message indicating a new peer has joined
    NewPeer { addr: String },
}

/// Information about a peer in the network
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Network address of the peer
    pub address: String,
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
    /// Port number of the peer sending the heartbeat
    pub port: u16,
}

/// Shared application state
pub struct AppState {
    /// Map of peer addresses to their HTTP clients
    pub peers: Arc<Mutex<HashMap<String, reqwest::Client>>>,
    /// Channel for sending messages within the application
    pub tx: broadcast::Sender<(Message, String)>,
    /// Database context
    pub db: Arc<DbContext>,
}
