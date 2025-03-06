use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};

use crate::types::{message::HeartbeatMessage, state::AppState, PeerHealth};
use super::ApiModule;

/// Health API module
pub struct Health;

impl Health {
    /// Health check endpoint
    pub async fn check(State(state): State<Arc<AppState>>) -> Response {
        let peers = state.peers.lock().await;
        let peer_count = peers.len();
        let response = format!("Healthy with {} peers", peer_count);
        Json(response).into_response()
    }

    /// Handle heartbeat from a peer
    pub async fn heartbeat(
        State(state): State<Arc<AppState>>,
        Json(heartbeat): Json<HeartbeatMessage>,
    ) -> Response {
        let peer_addr = &heartbeat.address;
        let mut peers = state.peers.lock().await;

        if !peers.contains_key(peer_addr) {
            error!("Heartbeat received from unknown peer: {}", peer_addr);
            return (StatusCode::BAD_REQUEST, "Peer not found").into_response();
        }

        // Update peer's last seen timestamp
        if let Err(e) = state
            .db
            .update_peer_last_seen(peer_addr, Utc::now().timestamp())
        {
            error!("Failed to update peer last seen timestamp: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update peer timestamp",
            )
                .into_response();
        }

        // Add any new peers we don't know about
        for peer_address in heartbeat.known_peers {
            if peer_address != state.own_address && !peers.contains_key(&peer_address) {
                info!("Discovered new peer from heartbeat: {}", peer_address);
                let client = reqwest::Client::builder()
                    .danger_accept_invalid_certs(true)
                    .build()
                    .expect("Failed to create HTTP client");
                peers.insert(
                    peer_address.clone(),
                    PeerHealth::new(client, peer_address.clone()),
                );
            }
        }

        (StatusCode::OK, "Heartbeat acknowledged").into_response()
    }
}

impl ApiModule for Health {
    fn register_routes() -> Router<Arc<AppState>> {
        Router::new().route("/health", get(Self::check))
    }
}
