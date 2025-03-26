use subversive::{
    db::context::DbContext,
    server::api::peers::Peers,
    shutdown::ShutdownState,
    types::{config::Config, health::PeerHealth, peer::PeerInfo, state::AppState},
};
use axum::{
    body::to_bytes,
    extract::{Json, State},
    response::IntoResponse,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

async fn setup_test_state() -> Arc<AppState> {
    let config = Config::default_config();
    let port = 8080;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    Arc::new(AppState {
        config,
        own_address: "https://localhost:8080".to_string(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: port,
        shutdown,
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
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    let state = Arc::new(AppState {
        config,
        own_address: "https://localhost:8080".to_string(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: port,
        shutdown,
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
