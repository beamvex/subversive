use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};

use crate::{types::{state::AppState, message::HeartbeatMessage}};

/// Handle heartbeat from a peer
pub async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Json(heartbeat): Json<HeartbeatMessage>,
) -> Response {
    let peer_addr = format!("http://localhost:{}", heartbeat.port);
    let peers = state.peers.lock().unwrap();

    if !peers.contains_key(&peer_addr) {
        error!("Heartbeat received from unknown peer: {}", peer_addr);
        return Json("Peer not found").into_response();
    }

    // Update peer's last seen timestamp
    if let Err(e) = state
        .db
        .update_peer_last_seen(&peer_addr, Utc::now().timestamp())
    {
        error!("Failed to update peer last seen timestamp: {}", e);
        return Json("Failed to update peer timestamp").into_response();
    }

    Json("Heartbeat acknowledged").into_response()
}
