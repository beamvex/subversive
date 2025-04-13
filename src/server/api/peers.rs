use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::time::SystemTime;

use crate::types::{health::PeerHealth, message::Message, peer::PeerInfo, state::AppState};
use subversive_network::peer::broadcast_to_peers;

/// Peers API module
pub struct Peers;

impl Peers {
    /// Add a new peer
    pub async fn add_peer(
        State(state): State<Arc<AppState>>,
        Json(peer): Json<PeerInfo>,
    ) -> impl IntoResponse {
        if peer.address.is_empty() {
            return "Peer address cannot be empty".into_response();
        }

        // Enforce HTTPS
        let peer_addr = if peer.address.starts_with("http://") {
            peer.address.replace("http://", "https://")
        } else {
            peer.address
        };

        // Add peer to in-memory state
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();

        let peer_health = PeerHealth::new(client, peer_addr.clone());
        state
            .peers
            .lock()
            .await
            .insert(peer_addr.clone(), peer_health);

        // Save peer to database
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        if let Err(e) = state.db.peers.save_peer(&peer_addr, timestamp).await {
            tracing::error!("Failed to save peer to database: {}", e);
        }

        // Broadcast to other peers
        let msg = Message::NewPeer {
            addr: peer_addr.clone(),
        };
        if let Err(e) = broadcast_to_peers(msg, "local", &state.peers).await {
            tracing::error!("Failed to broadcast new peer: {}", e);
        }

        // Return list of known peers
        let peers = state.peers.lock().await;
        let peer_list: Vec<PeerInfo> = peers
            .keys()
            .map(|addr| PeerInfo {
                address: addr.clone(),
                port: 0,
            })
            .collect();

        Json(peer_list).into_response()
    }

    /// List all known peers
    pub async fn list_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let peers = state.peers.lock().await;
        let peer_list: Vec<PeerInfo> = peers
            .keys()
            .map(|addr| PeerInfo {
                address: addr.clone(),
                port: 0,
            })
            .collect();

        Json(peer_list).into_response()
    }

    /// Get active peers
    pub async fn get_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let since = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
            - 3600; // Active in the last hour

        let peers = state
            .db
            .peers
            .get_active_peers(since)
            .await
            .unwrap_or_default();
        let peer_info: Vec<PeerInfo> = peers
            .into_iter()
            .map(|p| PeerInfo {
                address: p.address,
                port: 0,
            })
            .collect();

        Json(peer_info)
    }

    /// Register a new peer
    pub async fn register_peer(
        State(state): State<Arc<AppState>>,
        Json(peer): Json<PeerInfo>,
    ) -> impl IntoResponse {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Save peer to database
        if let Err(e) = state.db.peers.save_peer(&peer.address, timestamp).await {
            tracing::error!("Failed to save peer to database: {}", e);
            return Json(vec![]);
        }

        // Return list of known peers
        let peers = state
            .db
            .peers
            .get_active_peers(timestamp - 3600)
            .await
            .unwrap_or_default();
        let peer_info: Vec<PeerInfo> = peers
            .into_iter()
            .map(|p| PeerInfo {
                address: p.address,
                port: 0,
            })
            .collect();

        Json(peer_info)
    }
}

impl super::ApiModule for Peers {
    fn register_routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/peer", post(Self::add_peer))
            .route("/peers", get(Self::list_peers))
            .route("/active_peers", get(Self::get_peers))
            .route("/register_peer", post(Self::register_peer))
    }
}
