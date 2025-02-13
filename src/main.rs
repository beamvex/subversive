use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::Ipv4Addr,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::broadcast, signal};
use tracing::{error, info};

mod upnp;
mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Peer to connect to
    #[arg(short, long)]
    peer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Message {
    Chat { content: String },
    NewPeer { addr: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PeerInfo {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HeartbeatMessage {
    port: u16,
}

type PeerMap = Arc<Mutex<HashMap<String, reqwest::Client>>>;
type MessageSender = broadcast::Sender<(Message, String)>;

pub struct AppState {
    peers: PeerMap,
    tx: MessageSender,
}

async fn broadcast_to_peers(
    msg: Message,
    sender: &str,
    peers: &PeerMap,
) -> Result<()> {
    let peers_guard = peers.lock().unwrap();
    for (addr, client) in peers_guard.iter() {
        if addr == sender {
            continue;
        }

        if let Err(e) = client
            .post(format!("{}/receive", addr))
            .json(&msg)
            .send()
            .await
        {
            error!("Error sending message to peer {}: {}", addr, e);
        }
    }
    Ok(())
}

async fn check_peer_health(peers: PeerMap) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        let peers_guard = peers.lock().unwrap();
        
        for (addr, client) in peers_guard.iter() {
            if let Err(e) = client
                .post(format!("{}/heartbeat", addr))
                .json(&HeartbeatMessage {
                    port: 0,
                })
                .send()
                .await
            {
                error!("Error sending heartbeat to peer {}: {}", addr, e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let args = Args::parse();
    let mut port = args.port;

    // Initialize state
    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _) = broadcast::channel(100);
    
    // Set up UPnP port mapping
    let (actual_port, gateways) = upnp::setup_upnp(port).await?;
    port = actual_port;
    let cleanup_port = port;

    // Set up cleanup on Ctrl+C
    let gateways = gateways.clone();
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C, cleaning up UPnP mappings...");
                upnp::cleanup_upnp(cleanup_port, gateways).await;
                std::process::exit(0);
            }
            Err(err) => {
                error!("Error setting up Ctrl+C handler: {}", err);
            }
        }
    });

    // Start peer health checker
    let peers_clone = peers.clone();
    tokio::spawn(async move {
        check_peer_health(peers_clone).await;
    });
    
    // Create app state
    let app_state = Arc::new(AppState {
        peers,
        tx,
    });

    // Run HTTP server
    server::run_http_server(port, app_state).await?;

    Ok(())
}

async fn get_external_ip() -> Result<String> {
    let response = reqwest::get("https://api.ipify.org")
        .await?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read external IP response: {}", e))?;
    Ok(response)
}

fn get_network_interfaces() -> Result<Vec<Ipv4Addr>> {
    let output = std::process::Command::new("ip")
        .args(["addr", "show"])
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run ip command: {}", e))?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut interfaces = Vec::new();
    
    for line in output_str.lines() {
        if line.contains("inet ") && !line.contains("127.0.0.1") {
            if let Some(ip_str) = line
                .split_whitespace()
                .find(|s| s.contains('.'))
                .and_then(|s| s.split('/').next())
            {
                if let Ok(ip) = ip_str.parse() {
                    interfaces.push(ip);
                }
            }
        }
    }
    
    Ok(interfaces)
}
