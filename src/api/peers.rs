use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use chrono::Utc;
use std::sync::Arc;
use tracing::error;

use crate::{types::{state::AppState, message::Message, peer::PeerInfo}};

/// List all connected peers
pub async fn list_peers(State(state): State<Arc<AppState>>) -> Response {
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
        Err(e) => {
            error!("Failed to get active peers: {}", e);
            Json(Vec::<PeerInfo>::new()).into_response()
        }
    }
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
    state
        .peers
        .lock()
        .unwrap()
        .insert(peer_address.clone(), client);

    // Update peer's last seen timestamp in database
    if let Err(e) = state
        .db
        .update_peer_last_seen(&peer_address, Utc::now().timestamp())
    {
        error!("Failed to update peer in database: {}", e);
        return Json(format!("Failed to update peer in database: {}", e)).into_response();
    }

    let msg = Message::NewPeer {
        addr: peer_address.clone(),
    };

    if let Err(e) = state.tx.send((msg.clone(), "local".to_string())) {
        error!("Failed to process new peer locally: {}", e);
        return Json("Failed to process new peer locally").into_response();
    }

    if let Err(e) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        error!("Failed to broadcast new peer to network: {}", e);
        return Json("Failed to broadcast new peer to network").into_response();
    }

    Json("Peer added").into_response()
}
