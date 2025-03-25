use super::*;
use crate::db::context::DbContext;
use crate::shutdown::ShutdownState;
use crate::types::{config::Config, peer::PeerInfo, state::AppState};
use mockito::Server;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

async fn setup_test_state(own_address: &str) -> Arc<AppState> {
    let mut config = Config::default_config();
    config.hostname = Some(own_address.to_string());

    let port = 8080;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    Arc::new(AppState {
        config,
        own_address: own_address.to_string(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: port,
        shutdown,
    })
}

#[tokio::test]
async fn test_connect_to_initial_peer_no_peer_configured() {
    let state = setup_test_state("https://localhost:8080").await;
    let result = connect_to_initial_peer(state).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_connect_to_initial_peer_success() {
    let mut mock_server = Server::new_async().await;
    let peer_addr = mock_server.url();
    let own_addr = "https://localhost:8080".to_string();

    // Mock the peer response with a list of known peers
    let known_peers = vec![
        PeerInfo {
            address: "https://peer1:8080".to_string(),
        },
        PeerInfo {
            address: "https://peer2:8080".to_string(),
        },
    ];

    let _m = mock_server
        .mock("POST", "/peer")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&known_peers).unwrap())
        .create_async()
        .await;

    let mut config = Config::default_config();
    config.peer = Some(peer_addr.clone());

    let port = 8080;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    let state = Arc::new(AppState {
        config,
        own_address: own_addr.clone(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: 8080,
        shutdown,
    });

    let result = connect_to_initial_peer(state.clone()).await;
    assert!(result.is_ok());

    let peers = state.peers.lock().await;
    assert_eq!(peers.len(), 3); // Initial peer + 2 known peers
    assert!(peers.contains_key(&peer_addr));
    assert!(peers.contains_key("https://peer1:8080"));
    assert!(peers.contains_key("https://peer2:8080"));
}

#[tokio::test]
async fn test_connect_to_initial_peer_failure() {
    let mut mock_server = Server::new_async().await;
    let peer_addr = mock_server.url();

    let _m = mock_server
        .mock("POST", "/peer")
        .with_status(500)
        .create_async()
        .await;

    let mut config = Config::default_config();
    config.peer = Some(peer_addr);

    let port = 8080;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    let state = Arc::new(AppState {
        config,
        own_address: "https://localhost:8080".to_string(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: 8080,
        shutdown,
    });

    let result = connect_to_initial_peer(state.clone()).await;
    assert!(result.is_ok()); // Function succeeds but peer not added

    let peers = state.peers.lock().await;
    assert_eq!(peers.len(), 0);
}

#[tokio::test]
async fn test_connect_to_initial_peer_skip_own_address() {
    let mut mock_server = Server::new_async().await;
    let peer_addr = mock_server.url();
    let own_addr = "https://localhost:8080".to_string();

    // Mock response includes our own address
    let known_peers = vec![
        PeerInfo {
            address: own_addr.clone(),
        },
        PeerInfo {
            address: "https://peer1:8080".to_string(),
        },
    ];

    let _m = mock_server
        .mock("POST", "/peer")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(serde_json::to_string(&known_peers).unwrap())
        .create_async()
        .await;

    let mut config = Config::default_config();
    config.peer = Some(peer_addr.clone());

    let port = 8080;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    let state = Arc::new(AppState {
        config,
        own_address: own_addr.clone(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: 8080,
        shutdown,
    });

    let result = connect_to_initial_peer(state.clone()).await;
    assert!(result.is_ok());

    let peers = state.peers.lock().await;
    assert_eq!(peers.len(), 2); // Initial peer + 1 known peer (excluding own address)
    assert!(peers.contains_key(&peer_addr));
    assert!(peers.contains_key("https://peer1:8080"));
    assert!(!peers.contains_key(&own_addr));
}
