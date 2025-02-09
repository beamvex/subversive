use anyhow::Result;
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use clap::Parser;
use igd::{aio::search_gateway, PortMappingProtocol, SearchOptions};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::{SocketAddr, SocketAddrV4},
    sync::Arc,
    time::Duration,
};
use tokio::{
    sync::{broadcast, Mutex},
    time,
};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// HTTP port to listen on
    #[arg(short = 'l', long, default_value_t = 3000)]
    port: u16,

    /// Optional peer to connect to (e.g., http://localhost:3000)
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

struct AppState {
    peers: PeerMap,
    tx: MessageSender,
    port: u16,
}

async fn broadcast_to_peers(
    msg: Message,
    sender: &str,
    peers: &PeerMap,
) -> Result<()> {
    let peers = peers.lock().await;
    for (addr, client) in peers.iter() {
        if addr != sender {
            if let Err(e) = client
                .post(format!("{}/receive", addr))
                .json(&msg)
                .send()
                .await
            {
                error!("Failed to send message to {}: {}", addr, e);
            }
        }
    }
    Ok(())
}

async fn list_peers(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<String>> {
    let peers = state.peers.lock().await;
    Json(peers.keys().cloned().collect())
}

async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<ChatMessage>,
) -> Json<&'static str> {
    let msg = Message::Chat {
        content: message.content,
    };
    
    if let Err(e) = broadcast_to_peers(msg.clone(), "", &state.peers).await {
        error!("Failed to broadcast message: {}", e);
        return Json("Failed to send message");
    }
    
    if let Err(e) = state.tx.send((msg, String::new())) {
        error!("Failed to broadcast message locally: {}", e);
        return Json("Failed to send message");
    }
    
    Json("Message sent")
}

async fn receive_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Json<&'static str> {
    if let Err(e) = state.tx.send((message, String::new())) {
        error!("Failed to broadcast received message: {}", e);
        return Json("Failed to process message");
    }
    
    Json("Message received")
}

async fn add_peer(
    State(state): State<Arc<AppState>>,
    Json(peer_info): Json<PeerInfo>,
) -> Json<&'static str> {
    let client = reqwest::Client::new();
    
    // Try to connect to the peer
    match client
        .post(format!("{}/heartbeat", peer_info.address))
        .json(&HeartbeatMessage { port: state.port })
        .send()
        .await
    {
        Ok(_) => {
            let mut peers = state.peers.lock().await;
            peers.insert(peer_info.address.clone(), client);
            
            // Notify all peers about the new connection
            let msg = Message::NewPeer {
                addr: peer_info.address.clone(),
            };
            
            drop(peers); // Release the lock before broadcasting
            
            if let Err(e) = broadcast_to_peers(msg.clone(), &peer_info.address, &state.peers).await {
                error!("Failed to broadcast new peer: {}", e);
            }
            
            if let Err(e) = state.tx.send((msg, peer_info.address)) {
                error!("Failed to broadcast new peer locally: {}", e);
            }
            
            Json("Peer connection established")
        }
        Err(e) => {
            error!("Failed to connect to peer {}: {}", peer_info.address, e);
            Json("Failed to connect to peer")
        }
    }
}

async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Json(heartbeat): Json<HeartbeatMessage>,
) -> Json<&'static str> {
    info!("Received heartbeat from peer on port {}", heartbeat.port);
    
    // Add the peer if it's not already in our list
    let peer_addr = format!("http://localhost:{}", heartbeat.port);
    let mut peers = state.peers.lock().await;
    
    if !peers.contains_key(&peer_addr) {
        peers.insert(peer_addr.clone(), reqwest::Client::new());
        drop(peers);
        
        let msg = Message::NewPeer {
            addr: peer_addr.clone(),
        };
        
        if let Err(e) = broadcast_to_peers(msg.clone(), &peer_addr, &state.peers).await {
            error!("Failed to broadcast new peer: {}", e);
        }
        
        if let Err(e) = state.tx.send((msg, peer_addr)) {
            error!("Failed to broadcast new peer locally: {}", e);
        }
    }
    
    Json("Heartbeat acknowledged")
}

async fn setup_upnp(port: u16) -> Result<()> {
    // Search for IGD (Internet Gateway Device) with increased timeout
    let gateway = search_gateway(SearchOptions {
        timeout: Some(Duration::from_secs(5)), // Increase timeout to 5 seconds
        ..SearchOptions::default()
    })
    .await
    .map_err(|e| anyhow::anyhow!("Failed to find UPnP gateway: {}", e))?;

    // Add port mapping
    gateway
        .add_port(
            PortMappingProtocol::TCP,
            port,
            SocketAddrV4::new([0, 0, 0, 0].into(), port),
            0,  // lease duration (0 = unlimited)
            "P2P Network HTTP Server",
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to add UPnP port mapping: {}", e))?;

    info!("Successfully set up UPnP port mapping for port {}", port);
    Ok(())
}

async fn run_http_server(
    port: u16,
    app_state: Arc<AppState>,
) -> Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/peers", get(list_peers))
        .route("/message", post(send_message))
        .route("/receive", post(receive_message))
        .route("/peer", post(add_peer))
        .route("/heartbeat", post(heartbeat))
        .layer(cors)
        .with_state(app_state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("HTTP server listening on {}", addr);
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn check_peer_health(peers: PeerMap) {
    let mut interval = time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        let mut peers_to_remove = Vec::new();
        
        {
            let peers_guard = peers.lock().await;
            for (addr, client) in peers_guard.iter() {
                match client.get(format!("{}/peers", addr)).send().await {
                    Ok(_) => {
                        info!("Peer {} is healthy", addr);
                    }
                    Err(e) => {
                        error!("Peer {} is unreachable: {}", addr, e);
                        peers_to_remove.push(addr.clone());
                    }
                }
            }
        }
        
        if !peers_to_remove.is_empty() {
            let mut peers_guard = peers.lock().await;
            for addr in peers_to_remove {
                peers_guard.remove(&addr);
                info!("Removed unreachable peer: {}", addr);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    
    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _) = broadcast::channel(100);
    
    // Set up UPnP port mapping
    if let Err(e) = setup_upnp(args.port).await {
        error!("Failed to set up UPnP: {}. Continuing without port forwarding...", e);
    }

    // Start peer health checker
    let peers_clone = peers.clone();
    tokio::spawn(async move {
        check_peer_health(peers_clone).await;
    });
    
    // Connect to peer if specified
    if let Some(peer_addr) = args.peer {
        let client = reqwest::Client::new();
        match client
            .post(format!("{}/heartbeat", peer_addr))
            .json(&HeartbeatMessage { port: args.port })
            .send()
            .await
        {
            Ok(_) => {
                let mut peers_guard = peers.lock().await;
                peers_guard.insert(peer_addr.clone(), client);
                info!("Connected to peer: {}", peer_addr);
            }
            Err(e) => error!("Failed to connect to peer {}: {}", peer_addr, e),
        }
    }
    
    // Set up and run HTTP server
    let app_state = Arc::new(AppState {
        peers,
        tx,
        port: args.port,
    });
    
    run_http_server(args.port, app_state).await?;
    
    Ok(())
}
