use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};

use std::sync::Arc;
use tracing::error;

use crate::peer::broadcast_to_peers;
use crate::types::{message::Message, peer::PeerInfo, state::AppState, PeerHealth};

/// List all connected peers
pub async fn list_peers(State(state): State<Arc<AppState>>) -> Response {
    let peers = state.peers.lock().await;
    let peer_list = peers
        .keys()
        .map(|addr| PeerInfo {
            address: addr.clone(),
        })
        .collect::<Vec<_>>();

    Json(peer_list).into_response()
}

/// Add a new peer to the network
pub async fn add_peer(State(state): State<Arc<AppState>>, Json(peer): Json<PeerInfo>) -> Response {
    if peer.address.is_empty() {
        error!("Attempted to add peer with empty address");
        return Json("Peer address cannot be empty").into_response();
    }

    // Ensure we're using HTTPS
    let peer_address = if !peer.address.starts_with("https://") {
        peer.address.replace("http://", "https://")
    } else {
        peer.address
    };

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    let peer_health = PeerHealth::new(client.clone());
    state
        .peers
        .lock()
        .await
        .insert(peer_address.clone(), peer_health);

    // Create a message to broadcast the new peer
    let msg = Message::NewPeer {
        addr: peer_address.clone(),
    };

    if let Err(e) = broadcast_to_peers(msg, "local", &state.peers).await {
        error!("Failed to broadcast new peer to network: {}", e);
        return Json("Failed to broadcast new peer to network").into_response();
    }

    Json("Peer added successfully").into_response()
}
