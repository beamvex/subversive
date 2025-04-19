use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};

use crate::health::PeerHealth;

/// Information about a peer in the network
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Network address of the peer
    pub address: String,
    pub port: u16,
}

/// Initialize connection to an initial peer
///
/// # Arguments
/// * `peers` - Map of peer addresses to their health status
/// * `initial_peer` - Address of the initial peer to connect to
/// * `own_address` - Our own address that peers can use to connect to us
/// * `own_port` - Our own port number
pub async fn connect_to_initial_peer(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    initial_peer: Option<String>,
    own_address: String,
    own_port: u16,
) -> Result<()> {
    let peer_addr = match initial_peer {
        Some(addr) => addr,
        None => return Ok(()),
    };

    info!("Connecting to initial peer: {}", peer_addr);
    let client = Client::new();

    // Acquire the lock to update peers
    {
        let mut peers = peers.lock().await;
        let peer_info = PeerInfo {
            address: own_address.clone(),
            port: own_port,
        };

        // Send connection request to peer
        let response = client
            .post(format!("{}/peer", peer_addr))
            .json(&peer_info)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Successfully connected to peer: {}", peer_addr);

            // Add the initial peer
            let peer_health = PeerHealth::new(client.clone(), peer_addr.clone());
            peers.insert(peer_addr.clone(), peer_health);

            // Get and add the peer's known peers
            if let Ok(known_peers) = response.json::<Vec<PeerInfo>>().await {
                info!(
                    "Received {} known peers from {}",
                    known_peers.len(),
                    peer_addr
                );

                for known_peer in known_peers {
                    if known_peer.address != own_address && !peers.contains_key(&known_peer.address)
                    {
                        let peer_client = Client::new();
                        let peer_health = PeerHealth::new(peer_client, known_peer.address.clone());
                        peers.insert(known_peer.address.clone(), peer_health);
                    }
                }
            }
        } else {
            error!("Failed to connect to peer: {}", response.status());
        }
    }

    Ok(())
}

/// Add a new peer to the network
pub async fn add_peer(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    address: String,
) -> Result<(), String> {
    let mut peers = peers.lock().await;
    if !peers.contains_key(&address) {
        let client = Client::new();
        let peer_health = PeerHealth::new(client, address.clone());
        peers.insert(address, peer_health);
    }
    Ok(())
}

/// Add multiple peers to the network
pub async fn add_peers(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    peer_addrs: Vec<String>,
) -> Result<(), String> {
    for peer_addr in peer_addrs {
        add_peer(peers.clone(), peer_addr).await?;
    }
    Ok(())
}

/// Get all known peers
pub async fn get_peers(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
) -> Result<Vec<String>, String> {
    let peers = peers.lock().await;
    Ok(peers.keys().cloned().collect())
}

/// Remove a peer from the network
pub async fn remove_peer(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    address: String,
) -> Result<(), String> {
    let mut peers = peers.lock().await;
    if peers.remove(&address).is_some() {
        info!("Removed peer: {}", address);
    } else {
        debug!("Peer {} not found", address);
    }
    Ok(())
}

/// Update a peer's last seen timestamp
pub async fn update_peer_last_seen(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    peer_addr: String,
) {
    info!("Updating last seen for peer: {}", peer_addr);
    let mut peers = peers.lock().await;
    if let Some(peer_health) = peers.get_mut(&peer_addr) {
        peer_health.update_last_seen();
        debug!("Updated last seen for peer: {}", peer_addr);
    } else {
        debug!("Peer {} not found", peer_addr);
    }
}

/// Get peer info from address
pub fn get_peer_info(address: &str) -> Result<PeerInfo, String> {
    let parts: Vec<&str> = address.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid address format".to_string());
    }

    let port = parts[1].parse::<u16>().map_err(|e| e.to_string())?;
    Ok(PeerInfo {
        address: parts[0].to_string(),
        port,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use std::time::Duration;
    use subversive_utils::test_utils::init_test_tracing;

    async fn setup_test_peers() -> Arc<Mutex<HashMap<String, PeerHealth>>> {
        Arc::new(Mutex::new(HashMap::new()))
    }

    #[tokio::test]
    async fn test_connect_to_initial_peer_no_peer_configured() {
        let peers = setup_test_peers().await;
        let result =
            connect_to_initial_peer(peers, None, "https://localhost:8080".to_string(), 8080).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connect_to_initial_peer_success() {
        let mut mock_server = Server::new_async().await;
        let peer_addr = mock_server.url();
        let own_addr = "https://localhost:8080".to_string();
        let peers = setup_test_peers().await;

        // Mock the peer response with a list of known peers
        let known_peers = vec![
            PeerInfo {
                address: "https://peer1:8080".to_string(),
                port: 8080,
            },
            PeerInfo {
                address: "https://peer2:8080".to_string(),
                port: 8080,
            },
        ];

        let _m = mock_server
            .mock("POST", "/peer")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&known_peers).unwrap())
            .create_async()
            .await;

        let result = connect_to_initial_peer(
            peers.clone(),
            Some(peer_addr.clone()),
            own_addr.clone(),
            8080,
        )
        .await;
        assert!(result.is_ok());

        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 3); // Initial peer + 2 known peers
        assert!(peers_guard.contains_key(&peer_addr));
        assert!(peers_guard.contains_key("https://peer1:8080"));
        assert!(peers_guard.contains_key("https://peer2:8080"));
    }

    #[tokio::test]
    async fn test_connect_to_initial_peer_failure() {
        let mut mock_server = Server::new_async().await;
        let peer_addr = mock_server.url();
        let peers = setup_test_peers().await;

        let _m = mock_server
            .mock("POST", "/peer")
            .with_status(500)
            .create_async()
            .await;

        let result = connect_to_initial_peer(
            peers.clone(),
            Some(peer_addr.clone()),
            "https://localhost:8080".to_string(),
            8080,
        )
        .await;
        assert!(result.is_ok()); // Function succeeds but peer not added

        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 0);
    }

    #[tokio::test]
    async fn test_connect_to_initial_peer_skip_own_address() {
        let mut mock_server = Server::new_async().await;
        let peer_addr = mock_server.url();
        let own_addr = "https://localhost:8080".to_string();
        let peers = setup_test_peers().await;

        // Mock response includes our own address
        let known_peers = vec![
            PeerInfo {
                address: own_addr.clone(),
                port: 8080,
            },
            PeerInfo {
                address: "https://peer1:8080".to_string(),
                port: 8080,
            },
        ];

        let _m = mock_server
            .mock("POST", "/peer")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(serde_json::to_string(&known_peers).unwrap())
            .create_async()
            .await;

        let result = connect_to_initial_peer(
            peers.clone(),
            Some(peer_addr.clone()),
            own_addr.clone(),
            8080,
        )
        .await;
        assert!(result.is_ok());

        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 2); // Initial peer + 1 known peer (excluding own address)
        assert!(peers_guard.contains_key(&peer_addr));
        assert!(peers_guard.contains_key("https://peer1:8080"));
        assert!(!peers_guard.contains_key(&own_addr));
    }

    #[tokio::test]
    async fn test_add_peer() {
        let peers = setup_test_peers().await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add a peer
        add_peer(peers.clone(), peer_addr.clone()).await.unwrap();

        // Verify peer was added
        let peers_guard = peers.lock().await;
        assert!(peers_guard.contains_key(&peer_addr));

        // Try adding same peer again
        drop(peers_guard);
        add_peer(peers.clone(), peer_addr.clone()).await.unwrap();

        // Verify no duplicate was added
        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 1);
    }

    #[tokio::test]
    async fn test_add_peers() {
        let peers = setup_test_peers().await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
            "http://localhost:8082".to_string(),
        ];

        // Add multiple peers
        add_peers(peers.clone(), peer_addrs.clone()).await.unwrap();

        // Verify all peers were added
        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 3);
        for addr in peer_addrs {
            assert!(peers_guard.contains_key(&addr));
        }
    }

    #[tokio::test]
    async fn test_get_peers() {
        let peers = setup_test_peers().await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
        ];

        // Add peers
        add_peers(peers.clone(), peer_addrs.clone()).await.unwrap();

        // Get peers and verify
        let result = get_peers(peers.clone()).await.unwrap();
        assert_eq!(result.len(), 2);
        for addr in peer_addrs {
            assert!(result.contains(&addr));
        }
    }

    #[tokio::test]
    async fn test_remove_peer() {
        let peers = setup_test_peers().await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add and then remove a peer
        add_peer(peers.clone(), peer_addr.clone()).await.unwrap();
        remove_peer(peers.clone(), peer_addr.clone()).await.unwrap();

        // Verify peer was removed
        let peers_guard = peers.lock().await;
        assert!(!peers_guard.contains_key(&peer_addr));

        // Try removing non-existent peer
        drop(peers_guard);
        remove_peer(peers.clone(), "http://nonexistent:8080".to_string())
            .await
            .unwrap();
        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 0);
    }

    #[tokio::test]
    async fn test_update_peer_last_seen() {
        init_test_tracing();
        let peers = setup_test_peers().await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add a peer
        add_peer(peers.clone(), peer_addr.clone()).await.unwrap();

        // Get initial last seen time
        let peers_guard = peers.lock().await;
        let initial_last_seen = peers_guard.get(&peer_addr).unwrap().last_seen;
        drop(peers_guard);

        // Wait a moment
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Update last seen
        update_peer_last_seen(peers.clone(), peer_addr.clone()).await;

        // Verify last seen was updated
        let peers_guard = peers.lock().await;
        let new_last_seen = peers_guard.get(&peer_addr).unwrap().last_seen;
        info!(
            "Initial last seen: {}, new last seen: {}",
            initial_last_seen, new_last_seen
        );
        assert!(new_last_seen > initial_last_seen);
    }

    #[tokio::test]
    async fn test_update_peer_last_seen_non_existent() {
        init_test_tracing();
        let peers = setup_test_peers().await;
        // Try updating non-existent peer
        update_peer_last_seen(peers.clone(), "http://nonexistent:8080".to_string()).await;
        // Should not panic or affect existing peers
        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 0);
    }
}
