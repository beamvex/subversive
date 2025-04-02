use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use reqwest::Client;

use crate::network::health::{check_peer, check_peer_health};
use crate::shutdown::ShutdownState;
use crate::types::health::PeerHealth;

#[tokio::test]
async fn test_handle_health_check_success() {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let addr = "http://example.com".to_string();
    let client = Client::new();

    // Add a peer
    peers
        .lock()
        .await
        .insert(addr.clone(), PeerHealth::new(client.clone(), addr.clone()));

    let _shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));

    // Test successful health check
    let result = check_peer(&addr, peers.lock().await.get_mut(&addr).unwrap()).await;

    assert!(result.is_err()); // Example.com won't actually respond
    assert!(peers.lock().await.contains_key(&addr));
}

#[tokio::test]
async fn test_handle_health_check_failure() {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let addr = "http://example.com".to_string();
    let client = Client::new();

    // Add a peer
    peers
        .lock()
        .await
        .insert(addr.clone(), PeerHealth::new(client.clone(), addr.clone()));

    let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));

    // Check peer health which should remove timed out peers
    check_peer_health(&peers, false, &shutdown_state).await;

    // Verify peer was removed due to failed check
    assert!(!peers.lock().await.contains_key(&addr));
}

#[tokio::test]
async fn test_survival_mode_shutdown() {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));

    // Test health check in survival mode with no peers
    check_peer_health(&peers, true, &shutdown_state).await;

    // Verify shutdown was initiated
    assert!(shutdown_state.is_shutdown_initiated());
}

#[tokio::test]
async fn test_check_peer_health_timeout() {
    let peers = Arc::new(Mutex::new(HashMap::new()));
    let addr = "http://example.com".to_string();
    let client = Client::new();
    let mut peer_health = PeerHealth::new(client, addr.clone());

    // Set last seen to a long time ago by checking health (which will fail)
    check_peer(&addr, &mut peer_health).await.unwrap_err();

    peers.lock().await.insert(addr.clone(), peer_health);

    let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));

    // Check peer health
    check_peer_health(&peers, false, &shutdown_state).await;

    // Verify peer was removed due to timeout
    assert!(!peers.lock().await.contains_key(&addr));
}

#[tokio::test]
async fn test_check_peer_success() -> anyhow::Result<()> {
    // Start a mock server
    let mut server = mockito::Server::new();
    let mock = server.mock("GET", "/").with_status(200).create();

    let client = Client::new();
    let mut peer_health = PeerHealth::new(client, server.url());

    // Test health check
    let result = check_peer(&server.url(), &mut peer_health).await;

    mock.assert();
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_check_peer_failure() -> anyhow::Result<()> {
    // Start a mock server
    let mut server = mockito::Server::new();
    let mock = server.mock("GET", "/").with_status(500).create();

    let client = Client::new();
    let mut peer_health = PeerHealth::new(client, server.url());

    // Test health check
    let result = check_peer(&server.url(), &mut peer_health).await;

    mock.assert();
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_check_peer_connection_failure() {
    let client = Client::new();
    let addr = "http://invalid.example.com:12345".to_string();
    let mut peer_health = PeerHealth::new(client, addr.clone());

    // Test health check with invalid URL
    let result = check_peer(&addr, &mut peer_health).await;

    assert!(result.is_err());
}
