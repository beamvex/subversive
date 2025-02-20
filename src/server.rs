use axum::{
    extract::Json,
    routing::{post,get},
    Router,
    extract::State,
    response::{IntoResponse, Response},
};
use std::{net::SocketAddr, sync::Arc, path::PathBuf};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::info;
use chrono::Utc;

use crate::{AppState, ChatMessage, HeartbeatMessage, Message, PeerInfo};
use axum_macros::debug_handler;

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

    // Set up static file serving from public directory
    let public_dir = PathBuf::from("public");
    let static_files_service = ServeDir::new(public_dir);

    // Create router with routes
    let app = Router::new()
        .route("/peers", get(list_peers))
        .route("/send", post(send_message))
        .route("/receive", post(receive_message))
        .route("/peer", post(add_peer))
        .route("/heartbeat", post(heartbeat))
        .route("/recent_messages", get(get_recent_messages))
        .nest_service("/", static_files_service)
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
) -> Response {
    let one_hour_ago = Utc::now().timestamp() - 3600;
    match state.db.get_active_peers(one_hour_ago) {
        Ok(active_peers) => {
            let peer_list = active_peers
                .into_iter()
                .map(|peer| PeerInfo {
                    address: peer.address,
                })
                .collect::<Vec<_>>();
            Json(peer_list).into_response()
        }
        Err(_) => Json(Vec::<PeerInfo>::new()).into_response()
    }
}

/// Get recent messages
async fn get_recent_messages(
    State(state): State<Arc<AppState>>,
) -> Response {
    match state.db.get_recent_messages(50) {
        Ok(messages) => Json(messages).into_response(),
        Err(_) => Json(Vec::<crate::db::MessageDoc>::new()).into_response()
    }
}

/// Send a message to all peers
#[debug_handler]
async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<ChatMessage>,
) -> Response {
    
    let msg = Message::Chat {
        content: message.content,
    };

    if let Err(_) = state.tx.send((msg.clone(), "local".to_string())) {
        return Json("Failed to send message locally").into_response();
    }

    /*
    if let Err(_) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        return Json("Failed to broadcast message to peers").into_response();
    }
    */

    Json("Message sent").into_response()
}

/// Receive a message from a peer
async fn receive_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Response {
    if let Err(_) = state.tx.send((message, "remote".to_string())) {
        return Json("Failed to process received message").into_response();
    }

    Json("Message received").into_response()
}

/// Add a new peer to the network
async fn add_peer(
    State(state): State<Arc<AppState>>,
    Json(peer): Json<PeerInfo>,
) -> Response {
    if peer.address.is_empty() {
        return Json("Peer address cannot be empty").into_response();
    }

    let client = reqwest::Client::new();
    state.peers.lock().unwrap().insert(peer.address.clone(), client);

    // Update peer's last seen timestamp in database
    if let Err(e) = state.db.update_peer_last_seen(&peer.address, Utc::now().timestamp()) {
        return Json(format!("Failed to update peer in database: {}", e)).into_response();
    }

    let msg = Message::NewPeer {
        addr: peer.address.clone(),
    };

    if let Err(_) = state.tx.send((msg.clone(), "local".to_string())) {
        return Json("Failed to process new peer locally").into_response();
    }

    /*
    if let Err(_) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        return Json("Failed to broadcast new peer to network").into_response();
    }
    */

    Json("Peer added").into_response()
}


/// Handle heartbeat from a peer
async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Json(heartbeat): Json<HeartbeatMessage>,
) -> Response {
    let peer_addr = format!("http://localhost:{}", heartbeat.port);
    let peers = state.peers.lock().unwrap();
    
    if !peers.contains_key(&peer_addr) {
        return Json("Peer not found").into_response();
    }

    // Update peer's last seen timestamp
    if let Err(e) = state.db.update_peer_last_seen(&peer_addr, Utc::now().timestamp()) {
        return Json(format!("Failed to update peer last seen: {}", e)).into_response();
    }

    Json("Heartbeat received").into_response()
}
