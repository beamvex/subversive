use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::Mutex;
use tracing::{error, info};

use crate::shutdown::ShutdownState;
use crate::types::health::PeerHealth;

const PEER_TIMEOUT: i64 = 3600; // 1 hour

/// Handle a health check result
async fn handle_health_check_result(
    peers: &Arc<Mutex<HashMap<String, PeerHealth>>>,
    addr: String,
    result: Result<(), Box<dyn std::error::Error + Send + Sync>>,
    survival_mode: bool,
    shutdown_state: &Arc<ShutdownState>,
) {
    let mut peers = peers.lock().await;
    if let Some(peer_health) = peers.get_mut(&addr) {
        match result {
            Ok(_) => {
                peer_health.update_last_seen();
            }
            Err(e) => {
                error!("Health check failed for {}: {}", addr, e);
                peers.remove(&addr);
            }
        }
    }

    // In survival mode, if we have no peers and no gateways, shut down
    if survival_mode && peers.is_empty() && shutdown_state.gateways().is_empty() {
        info!("No peers or gateways available in survival mode, shutting down");
        shutdown_state.initiate_shutdown();
    }
}

/// Check the health of all peers
pub async fn check_peer_health(
    peers: &Arc<Mutex<HashMap<String, PeerHealth>>>,
    survival_mode: bool,
    shutdown_state: &Arc<ShutdownState>,
) {
    let mut peers = peers.lock().await;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Remove peers that haven't been seen in a while
    let dead_peers: Vec<String> = peers
        .iter()
        .filter(|(_, health)| now - health.get_last_seen() > PEER_TIMEOUT)
        .map(|(addr, _)| addr.clone())
        .collect();

    for addr in dead_peers {
        info!("Removing dead peer: {}", addr);
        peers.remove(&addr);
    }

    // In survival mode, if we have no peers and no gateways, shut down
    if survival_mode && peers.is_empty() && shutdown_state.gateways().is_empty() {
        info!("No peers or gateways available in survival mode, shutting down");
        shutdown_state.initiate_shutdown();
    }
}

/// Check the health of a specific peer
pub async fn check_peer(
    addr: &str,
    peer_health: &mut PeerHealth,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match peer_health.client.get(addr).send().await {
        Ok(response) => {
            if response.status().is_success() {
                peer_health.update_last_seen();
                Ok(())
            } else {
                error!("Failed health check for {}: {}", addr, response.status());
                Err("Health check failed".into())
            }
        }
        Err(e) => {
            error!("Failed to connect to {}: {}", addr, e);
            Err(e.into())
        }
    }
}

/// Check the health of all peers periodically
pub async fn start_health_check_loop(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    survival_mode: bool,
    shutdown_state: Arc<ShutdownState>,
) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
    loop {
        interval.tick().await;

        let peers_clone = peers.clone();
        let mut peers_lock = peers_clone.lock().await;
        let addrs: Vec<String> = peers_lock.keys().cloned().collect();

        for addr in addrs {
            if let Some(peer_health) = peers_lock.get_mut(&addr) {
                let result = check_peer(&addr, peer_health).await;
                handle_health_check_result(
                    &peers_clone,
                    addr,
                    result,
                    survival_mode,
                    &shutdown_state,
                )
                .await;
            }
        }

        check_peer_health(&peers_clone, survival_mode, &shutdown_state).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    #[tokio::test]
    async fn test_handle_health_check_result_success() {
        let peers = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));
        let addr = "http://localhost:8080".to_string();

        // Add a peer
        peers
            .lock()
            .await
            .insert(addr.clone(), PeerHealth::new(Client::new(), addr.clone()));

        // Get initial last seen time
        let initial_last_seen = peers.lock().await.get(&addr).unwrap().get_last_seen();

        // Wait a moment to ensure time difference
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // Handle successful health check
        handle_health_check_result(&peers, addr.clone(), Ok(()), false, &shutdown_state).await;

        // Verify peer was updated but not removed
        let peers_lock = peers.lock().await;
        assert!(peers_lock.contains_key(&addr));
        let new_last_seen = peers_lock.get(&addr).unwrap().get_last_seen();
        assert!(new_last_seen > initial_last_seen);
    }

    #[tokio::test]
    async fn test_handle_health_check_result_failure() {
        let peers = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));
        let addr = "http://localhost:8080".to_string();

        // Add a peer
        peers
            .lock()
            .await
            .insert(addr.clone(), PeerHealth::new(Client::new(), addr.clone()));

        // Handle failed health check
        handle_health_check_result(
            &peers,
            addr.clone(),
            Err("Health check failed".into()),
            false,
            &shutdown_state,
        )
        .await;

        // Verify peer was removed
        assert!(!peers.lock().await.contains_key(&addr));
        assert!(!shutdown_state.is_shutdown_initiated());
    }

    #[tokio::test]
    async fn test_handle_health_check_result_survival_mode() {
        let peers = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));
        let addr = "http://localhost:8080".to_string();

        // Add a peer
        peers
            .lock()
            .await
            .insert(addr.clone(), PeerHealth::new(Client::new(), addr.clone()));

        // Handle failed health check in survival mode
        handle_health_check_result(
            &peers,
            addr.clone(),
            Err("Health check failed".into()),
            true,
            &shutdown_state,
        )
        .await;

        // Verify peer was removed and shutdown was initiated
        assert!(!peers.lock().await.contains_key(&addr));
        assert!(shutdown_state.is_shutdown_initiated());
    }

    #[tokio::test]
    async fn test_handle_health_check_result_nonexistent_peer() {
        let peers = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));
        let addr = "http://localhost:8080".to_string();

        // Handle health check for non-existent peer
        handle_health_check_result(&peers, addr.clone(), Ok(()), false, &shutdown_state).await;

        // Verify nothing changed
        assert!(peers.lock().await.is_empty());
        assert!(!shutdown_state.is_shutdown_initiated());
    }

    #[tokio::test]
    async fn test_handle_health_check_success() {
        let peers = Arc::new(Mutex::new(HashMap::new()));

        // Start a mock server that returns 200 OK
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .create_async()
            .await;

        let client = Client::new();
        let addr = server.url();

        // Add a peer
        peers
            .lock()
            .await
            .insert(addr.clone(), PeerHealth::new(client.clone(), addr.clone()));

        let _shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));

        // Test successful health check
        let result = check_peer(&addr, peers.lock().await.get_mut(&addr).unwrap()).await;

        assert!(result.is_ok()); // Should succeed with 200 OK
        assert!(peers.lock().await.contains_key(&addr));
        mock.assert();
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
        // Start a mock server that never responds to requests
        let server = mockito::Server::new_async().await;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(1))
            .build()
            .unwrap();
        let mut peer_health = PeerHealth::new(client, server.url());

        // Test timeout
        let result = check_peer(&server.url(), &mut peer_health).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_peer_success() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .create_async()
            .await;

        let client = Client::new();
        let mut peer_health = PeerHealth::new(client, server.url());

        let result = check_peer(&server.url(), &mut peer_health).await;
        assert!(result.is_ok());
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_check_peer_failure() -> anyhow::Result<()> {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .with_status(500)
            .create_async()
            .await;

        let client = Client::new();
        let mut peer_health = PeerHealth::new(client, server.url());

        let result = check_peer(&server.url(), &mut peer_health).await;
        assert!(result.is_err());
        mock.assert();

        Ok(())
    }

    #[tokio::test]
    async fn test_check_peer_connection_failure() {
        // Use an invalid port to force a connection failure
        let addr = "http://localhost:1";
        let client = Client::new();
        let mut peer_health = PeerHealth::new(client, addr.to_string());

        let result = check_peer(addr, &mut peer_health).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_health_check_loop() {
        let peers = Arc::new(Mutex::new(HashMap::new()));
        let shutdown_state = Arc::new(ShutdownState::new(8080, Vec::new()));

        // Start a mock server
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .expect(2) // Expect 2 health check call
            .create_async()
            .await;

        let client = Client::new();
        let addr = server.url();

        // Add a peer
        peers
            .lock()
            .await
            .insert(addr.clone(), PeerHealth::new(client.clone(), addr.clone()));

        // Create a channel to signal when to stop the health check loop
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let peers_clone = peers.clone();
        let shutdown_state_clone = shutdown_state.clone();

        // Spawn the health check loop in a separate task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(100));
            loop {
                interval.tick().await;

                let peers_clone = peers_clone.clone();
                let mut peers_lock = peers_clone.lock().await;
                let addrs: Vec<String> = peers_lock.keys().cloned().collect();

                for addr in addrs {
                    if let Some(peer_health) = peers_lock.get_mut(&addr) {
                        let result = check_peer(&addr, peer_health).await;
                        handle_health_check_result(
                            &peers_clone,
                            addr,
                            result,
                            false,
                            &shutdown_state_clone,
                        )
                        .await;
                    }
                }

                check_peer_health(&peers_clone, false, &shutdown_state_clone).await;

                // Check if we should stop
                if rx.try_recv().is_ok() {
                    break;
                }
            }
        });

        // Wait for a few health check intervals
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;

        // Signal the health check loop to stop
        tx.send(()).await.unwrap();

        // Verify the mock was called the expected number of times
        mock.assert();

        // Verify the peer is still present and healthy
        let peers_lock = peers.lock().await;
        assert!(peers_lock.contains_key(&addr));
        let peer = peers_lock.get(&addr).unwrap();
        let last_seen = peer.get_last_seen();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        assert!(now - last_seen < 1, "Peer was not seen recently");
    }
}
