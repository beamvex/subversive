use crate::server::api::peers::Peers;
use crate::types::{config::Config, state::AppState, health::PeerHealth, PeerInfo};
use axum::{
    body::Bytes,
    extract::{Json, State},
    http::StatusCode,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, broadcast};
use crate::db::context::DbContext;
use crate::shutdown::ShutdownState;

fn setup_test_state() -> Arc<AppState> {
    let config = Config::default();
    let port = 8080;
    let gateways = Vec::new();
    let (shutdown_tx, _) = broadcast::channel(1);
    let shutdown = Arc::new(ShutdownState::new(shutdown_tx, port, gateways));
    
    Arc::new(AppState {
        config,
        own_address: "https://localhost:8080".to_string(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_in_memory().unwrap()),
        actual_port: port,
        shutdown,
    })
}

#[tokio::test]
async fn test_list_peers_empty() {
    let state = setup_test_state();
    let response = Peers::list_peers(State(state)).await;
    let body = Bytes::from_request(response.into_body()).await.unwrap();
    let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
    assert!(peers.is_empty());
}

#[tokio::test]
async fn test_list_peers_with_peers() {
    let state = setup_test_state();
    let peer_addr = "https://peer1:8080".to_string();
    
    // Add a test peer
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    state.peers.lock().await.insert(
        peer_addr.clone(),
        PeerHealth::new(client, peer_addr.clone()),
    );

    let response = Peers::list_peers(State(state)).await;
    let body = Bytes::from_request(response.into_body()).await.unwrap();
    let peers: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
    
    assert_eq!(peers.len(), 1);
    assert_eq!(peers[0].address, peer_addr);
}

#[tokio::test]
async fn test_add_peer_empty_address() {
    let state = setup_test_state();
    let peer = PeerInfo {
        address: "".to_string(),
    };
    
    let response = Peers::add_peer(
        State(state),
        Json(peer),
    ).await;
    
    assert_eq!(response.status(), StatusCode::OK);
    let body = Bytes::from_request(response.into_body()).await.unwrap();
    let error_msg: &str = std::str::from_utf8(&body).unwrap();
    assert!(error_msg.contains("cannot be empty"));
}

#[tokio::test]
async fn test_add_peer_success() {
    let state = setup_test_state();
    let peer_addr = "http://peer1:8080".to_string();
    let peer = PeerInfo {
        address: peer_addr.clone(),
    };
    
    let response = Peers::add_peer(
        State(state.clone()),
        Json(peer),
    ).await;
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Verify peer was added and HTTPS was enforced
    let peers = state.peers.lock().await;
    assert_eq!(peers.len(), 1);
    assert!(peers.contains_key(&peer_addr.replace("http://", "https://")));
    
    // Verify response contains the list of peers
    let body = Bytes::from_request(response.into_body()).await.unwrap();
    let peer_list: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
    assert_eq!(peer_list.len(), 1);
    assert_eq!(peer_list[0].address, peer_addr.replace("http://", "https://"));
}

#[tokio::test]
async fn test_add_peer_already_https() {
    let state = setup_test_state();
    let peer_addr = "https://peer1:8080".to_string();
    let peer = PeerInfo {
        address: peer_addr.clone(),
    };
    
    let response = Peers::add_peer(
        State(state.clone()),
        Json(peer),
    ).await;
    
    assert_eq!(response.status(), StatusCode::OK);
    
    // Verify peer was added without modification
    let peers = state.peers.lock().await;
    assert_eq!(peers.len(), 1);
    assert!(peers.contains_key(&peer_addr));
    
    let body = Bytes::from_request(response.into_body()).await.unwrap();
    let peer_list: Vec<PeerInfo> = serde_json::from_slice(&body).unwrap();
    assert_eq!(peer_list.len(), 1);
    assert_eq!(peer_list[0].address, peer_addr);
}
