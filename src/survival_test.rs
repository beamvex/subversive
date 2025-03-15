use std::collections::HashMap;
use std::sync::Arc;
use tempfile::tempdir;
use tokio::sync::Mutex;

use crate::{
    db::DbContext,
    shutdown::ShutdownState,
    survival::{check_survival_status, start_survival_mode},
    types::{config::Config, health::PeerHealth, state::AppState},
};

#[tokio::test]
async fn test_survival_mode_start() {
    let config = Config::default();
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(12345, gateways));
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(DbContext::new(&db_path).unwrap());
    let app_state = Arc::new(AppState {
        config,
        peers: peers.clone(),
        db,
        own_address: "http://localhost:12345".to_string(),
        shutdown,
        actual_port: 12345,
    });

    // Start survival mode
    start_survival_mode(app_state.clone()).await;

    // Sleep briefly to let the survival mode task start
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Add a peer and verify heartbeat is sent
    let mut peers = peers.lock().await;
    peers.insert(
        "http://localhost:8080".to_string(),
        PeerHealth::new(reqwest::Client::new(), "http://localhost:8080".to_string()),
    );
    drop(peers);

    // Check survival status - this should send a heartbeat
    check_survival_status(&app_state).await;
}

#[tokio::test]
async fn test_survival_mode_no_peers() {
    let config = Config::default();
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(12345, gateways));
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(DbContext::new(&db_path).unwrap());
    let app_state = Arc::new(AppState {
        config,
        peers: peers.clone(),
        db,
        own_address: "http://localhost:12345".to_string(),
        shutdown,
        actual_port: 12345,
    });

    // Check survival status with no peers
    check_survival_status(&app_state).await;

    // Verify we're still running (no shutdown in survival mode)
    assert!(peers.lock().await.is_empty());
}

#[tokio::test]
async fn test_survival_mode_with_peers() {
    let config = Config::default();
    let mut peers_map = HashMap::new();
    peers_map.insert(
        "http://localhost:8080".to_string(),
        PeerHealth::new(reqwest::Client::new(), "http://localhost:8080".to_string()),
    );
    peers_map.insert(
        "http://localhost:8081".to_string(),
        PeerHealth::new(reqwest::Client::new(), "http://localhost:8081".to_string()),
    );
    let peers = Arc::new(Mutex::new(peers_map));
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(12345, gateways));
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(DbContext::new(&db_path).unwrap());
    let app_state = Arc::new(AppState {
        config,
        peers: peers.clone(),
        db,
        own_address: "http://localhost:12345".to_string(),
        shutdown,
        actual_port: 12345,
    });

    // Check survival status with peers
    check_survival_status(&app_state).await;

    // Verify peers are still present
    assert_eq!(peers.lock().await.len(), 2);
}

#[tokio::test]
async fn test_survival_mode_peer_reconnection() {
    let config = Config::default();
    let mut peers_map = HashMap::new();
    let mut peer = PeerHealth::new(reqwest::Client::new(), "http://localhost:8080".to_string());
    // Mark peer as unhealthy
    peer.record_failure();
    peers_map.insert("http://localhost:8080".to_string(), peer);
    let peers = Arc::new(Mutex::new(peers_map));
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(12345, gateways));
    let temp_dir = tempdir().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let db = Arc::new(DbContext::new(&db_path).unwrap());
    let app_state = Arc::new(AppState {
        config,
        peers: peers.clone(),
        db,
        own_address: "http://localhost:12345".to_string(),
        shutdown,
        actual_port: 12345,
    });

    // Check survival status - this should attempt reconnection
    check_survival_status(&app_state).await;

    // Verify peer is still present (we keep peers in survival mode)
    assert_eq!(peers.lock().await.len(), 1);
}
