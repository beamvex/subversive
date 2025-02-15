// Import required dependencies and types
use axum::{
    extract::Json,
    routing::{post,get},
    Router,
    extract::State,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{AppState, ChatMessage, HeartbeatMessage, Message, PeerInfo};

/// Start the HTTP server
/// 
/// # Arguments
/// * `port` - Port to listen on
/// * `app_state` - Shared application state
pub async fn run_http_server(port: u16, app_state: Arc<AppState>) -> anyhow::Result<()> {
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with routes
    let app = Router::new()
        .route("/peers", get(list_peers))
        .route("/send", post(send_message))
        .route("/receive", post(receive_message))
        .route("/peer", post(add_peer))
        .route("/heartbeat", post(heartbeat))
        .layer(cors)
        .with_state(app_state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting HTTP server on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

/// List all connected peers
async fn list_peers(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<PeerInfo>>{
    let peers = state.peers.lock().unwrap();
    let peer_list = peers
        .keys()
        .map(|addr| PeerInfo {
            address: addr.clone(),
        })
        .collect::<Vec<_>>();
    Json(peer_list)
}

/// Send a message to all peers
pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<ChatMessage>,
) -> Json<&'static str> {
    let msg = Message::Chat {
        content: message.content,
    };

    if let Err(_) = state.tx.send((msg.clone(), "local".to_string())) {
        return axum::response::Json("Failed to send message locally");
    }

    if let Err(_) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        return axum::response::Json("Failed to broadcast message to peers");
    }

    axum::response::Json("Message sent")
}

/// Receive a message from a peer
async fn receive_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Json<String> {
    if let Err(_) = state.tx.send((message, "remote".to_string())) {
        return Json("Failed to process received message".to_string());
    }

    Json("Message received".to_string())
}

/// Add a new peer to the network
async fn add_peer(
    State(state): State<Arc<AppState>>,
    Json(peer): Json<PeerInfo>,
) -> Json<String> {
    if peer.address.is_empty() {
        return Json("Peer address cannot be empty".to_string());
    }

    let client = reqwest::Client::new();
    state.peers.lock().unwrap().insert(peer.address.clone(), client);

    let msg = Message::NewPeer {
        addr: peer.address.clone(),
    };

    if let Err(_) = state.tx.send((msg.clone(), "local".to_string())) {
        return Json("Failed to process new peer locally".to_string());
    }

    if let Err(_) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        return Json("Failed to broadcast new peer to network".to_string());
    }

    Json("Peer added".to_string())
}

/// Handle heartbeat from a peer
async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Json(heartbeat): Json<HeartbeatMessage>,
) -> Json<String> {
    let peers = state.peers.lock().unwrap();
    if !peers.contains_key(&format!("http://localhost:{}", heartbeat.port)) {
        return Json("Peer not found".to_string());
    }
    Json("Heartbeat received".to_string())
}
