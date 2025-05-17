use axum::{
    extract::{Json, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use std::time::SystemTime;
use tracing::info;

use subversive_types::{message::Message, state::AppState};

use subversive_network::{health::PeerHealth, peer::PeerInfo};

/// Peers API module
pub struct Peers;

impl Peers {
    /// Add a new peer
    pub async fn add_peer(
        State(state): State<Arc<AppState>>,
        Json(peer): Json<PeerInfo>,
    ) -> impl IntoResponse {
        info!("Adding peer: {}", peer.address);
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
        let _msg = Message::NewPeer {
            addr: peer_addr.clone(),
        };

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

#[cfg(test)]
mod tests {
    use axum::{
        body::to_bytes,
        extract::{Json, State},
        response::IntoResponse,
    };
    use std::{collections::HashMap, sync::Arc};
    use subversive_database::context::DbContext;
    use subversive_network::{health::PeerHealth, peer::PeerInfo};
    use subversive_types::{config::Config, state::AppState};
    use tokio::sync::Mutex;

    use crate::api::peers::Peers;

    async fn setup_test_state() -> Arc<AppState> {
        let config = Config::default_config();
        let port = 8080;

        Arc::new(AppState {
            config,
            own_address: "https://localhost:8080".to_string(),
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: port,
        })
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_peers_empty() {
        let state = setup_test_state().await;
        let response = Peers::list_peers(State(state)).await.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
        assert!(peers.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_peers_with_peers() {
        let state = setup_test_state().await;
        let peer_addr = "https://peer1:8080".to_string();

        // Add a test peer
        let client = reqwest::Client::new();
        state.peers.lock().await.insert(
            peer_addr.clone(),
            PeerHealth::new(client, peer_addr.clone()),
        );

        let response = Peers::list_peers(State(state)).await.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();

        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address, peer_addr);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_list_peers() {
        let config = Config::default_config();
        let port = 8080;

        let state = Arc::new(AppState {
            config,
            own_address: "https://localhost:8080".to_string(),
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: port,
        });

        let response = Peers::list_peers(State(state)).await.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
        assert!(peers.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_add_peer_empty_address() {
        let state = setup_test_state().await;
        let peer = PeerInfo {
            address: "".to_string(),
            port: 0,
        };

        let response = Peers::add_peer(State(state), Json(peer))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let error_msg: &str = std::str::from_utf8(&body).unwrap();
        assert!(error_msg.contains("cannot be empty"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_add_peer_success() {
        let state = setup_test_state().await;
        let peer_addr = "http://peer1:8080".to_string();
        let peer = PeerInfo {
            address: peer_addr.clone(),
            port: 0,
        };

        let response = Peers::add_peer(State(state.clone()), Json(peer))
            .await
            .into_response();

        // Verify peer was added and HTTPS was enforced
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 1);
        assert!(peers.contains_key(&peer_addr.replace("http://", "https://")));

        // Verify response contains the list of peers
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peer_list: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
        assert_eq!(peer_list.len(), 1);
        assert_eq!(
            peer_list[0].address,
            peer_addr.replace("http://", "https://")
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_add_peer_already_https() {
        let state = setup_test_state().await;
        let peer_addr = "https://peer1:8080".to_string();
        let peer = PeerInfo {
            address: peer_addr.clone(),
            port: 0,
        };

        let response = Peers::add_peer(State(state.clone()), Json(peer))
            .await
            .into_response();

        // Verify peer was added without modification
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 1);
        assert!(peers.contains_key(&peer_addr));

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peer_list: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
        assert_eq!(peer_list.len(), 1);
        assert_eq!(peer_list[0].address, peer_addr);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_active_peers() {
        let state = setup_test_state().await;
        let peer_addr = "https://peer1:8080".to_string();

        // Add a peer to the database
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        state
            .db
            .peers
            .save_peer(&peer_addr, timestamp)
            .await
            .unwrap();

        let response = Peers::get_peers(State(state)).await.into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();

        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address, peer_addr);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_register_peer() {
        let state = setup_test_state().await;
        let peer_addr = "https://peer1:8080".to_string();
        let peer = PeerInfo {
            address: peer_addr.clone(),
            port: 0,
        };

        let response = Peers::register_peer(State(state.clone()), Json(peer))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();

        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address, peer_addr);

        // Verify peer was saved in the database
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let db_peers = state
            .db
            .peers
            .get_active_peers(timestamp - 3600)
            .await
            .unwrap();
        assert_eq!(db_peers.len(), 1);
        assert_eq!(db_peers[0].address, peer_addr);
    }
}
