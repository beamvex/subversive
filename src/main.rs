// Import required dependencies and types
use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::{MutexGuard, broadcast}, signal};
use tracing::{error, info, Level};
use tracing_subscriber;
use chrono;
use rand::Rng;

// Re-export modules
pub mod server;
pub mod upnp;
mod db;

use db::DbContext;

/// Command line arguments for the P2P network application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Port to listen on for P2P communication (defaults to random port between 10000-65535)
    #[arg(short, long)]
    port: Option<u16>,

    /// Initial peer to connect to
    #[arg(short('e'), long)]
    peer: Option<String>,
}

/// Message types that can be exchanged between peers
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    /// Regular chat message
    Chat {
        content: String,
    },
    /// Message indicating a new peer has joined
    NewPeer {
        addr: String,
    },
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

/// Main entry point of the application
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // Parse command line arguments
    let args = Args::parse();
    
    // Generate random port between 10000-65535 if not specified
    let port = args.port.unwrap_or_else(|| {
        let mut rng = rand::thread_rng();
        rng.gen_range(10000..=65535)
    });
    
    info!("Using port: {}", port);

    // Create a channel for message passing
    let (tx, mut rx) = broadcast::channel(32);

    // Initialize database
    let db = Arc::new(DbContext::new("p2p_network.db")?);

    // Initialize shared application state
    let app_state = Arc::new(AppState {
        peers: Arc::new(Mutex::new(HashMap::new())),
        tx: tx.clone(),
        db: db.clone(),
    });
    info!("Starting up");

    // Set up UPnP port mapping
    let (actual_port, gateways) = upnp::setup_upnp(port).await?;
    info!("Using port {}", actual_port);

    // After UPnP setup
    if crate::upnp::is_wsl() {
        info!("Manual port forwarding required:");
        info!("1. On Windows host, run PowerShell as Admin");
        info!("2. Execute: netsh interface portproxy add v4tov4 listenport={} listenaddress=0.0.0.0 connectport={} connectaddress=127.0.0.1", actual_port, actual_port);
        info!("3. Allow the port in Windows Defender Firewall:");
        info!("   New-NetFirewallRule -DisplayName 'P2P Port' -Direction Inbound -Action Allow -Protocol TCP -LocalPort {}", actual_port);
    }

    // Spawn a task to handle message processing
    let _state = app_state.clone();
    tokio::spawn(async move {
        while let Ok((message, source)) = rx.recv().await {
            match message {
                Message::Chat { content } => {
                    info!("Received chat message from {}: {}", source, content);
                    if let Err(e) = db.save_message(&content, &source, chrono::Utc::now().timestamp()) {
                        error!("Failed to save message: {}", e);
                    }
                }
                Message::NewPeer { addr } => {
                    info!("Received new peer from {}: {}", source, addr);
                    if let Err(e) = db.save_peer(&addr, chrono::Utc::now().timestamp()) {
                        error!("Failed to save peer: {}", e);
                    }
                }
            }
        }
    });

    // Set up cleanup on Ctrl+C
    let gateways = gateways.clone();
    let cleanup_port = actual_port;
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C, cleaning up UPnP mappings...");
                let _ =upnp::cleanup_upnp(cleanup_port, gateways).await;
                std::process::exit(0);
            }
            Err(err) => {
                error!("Error setting up Ctrl+C handler: {}", err);
            }
        }
    });

    // Start peer health checker
    /*
    let peers_clone = app_state.peers.clone();
    tokio::spawn(async move {
        let peers = peers_clone.lock();
        //check_peer_health(&mut peers).await;
    });
    */

    // Start the HTTP server
    server::run_http_server(actual_port, app_state).await?;

    Ok(())
}

/// Broadcast a message to all connected peers
/// 
/// # Arguments
/// * `message` - The message to broadcast
/// * `source` - The source of the message (to avoid sending back to sender)
/// * `peers` - Map of peer addresses to their HTTP clients
pub async fn broadcast_to_peers(
    message: Message,
    source: &str,
    peers: &Arc<Mutex<HashMap<String, reqwest::Client>>>,
) -> Result<()> {
    // Get a lock on the peers map
    let peers_guard = peers.lock().unwrap();

    // Send the message to each peer except the source
    for (addr, client) in peers_guard.iter() {
        if addr == source {
            continue;
        }

        if let Err(e) = client
            .post(format!("{}/receive", addr))
            .json(&message)
            .send()
            .await
        {
            info!("Error sending message to {}: {}", addr, e);
        }
    }

    Ok(())
}

/// Check the health of all connected peers
/// 
/// # Arguments
/// * `peers` - Map of peer addresses to their HTTP clients
pub async fn check_peer_health(peers: &mut MutexGuard<'_, HashMap<String, reqwest::Client>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        for (addr, client) in peers.iter() {
            if let Err(e) = client
                .post(format!("{}/heartbeat", addr))
                .send()
                .await
            {
                info!("Error sending heartbeat to {}: {}", addr, e);
            }
        }
    }
}

/// Get the external IP address of the machine
/// 
/// # Returns
/// The external IP address as a string
pub async fn get_external_ip() -> Result<String> {
    let response = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await?;
    Ok(response)
}

/// Get the network interfaces of the machine
/// 
/// # Returns
/// A vector of IPv4 addresses of the network interfaces
pub fn get_network_interfaces() -> Result<Vec<Ipv4Addr>> {
    let output = std::process::Command::new("ip")
        .args(["addr", "show"])
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut addresses = Vec::new();
    
    for line in stdout.lines() {
        if line.contains("inet ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(addr_str) = parts.get(1) {
                if let Some(addr_str) = addr_str.split('/').next() {
                    if let Ok(addr) = addr_str.parse() {
                        addresses.push(addr);
                    }
                }
            }
        }
    }
    
    Ok(addresses)
}
