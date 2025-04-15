use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info};

use serde::{Deserialize, Serialize};

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
/// * `state` - Shared application state
pub async fn connect_to_initial_peer(state: Arc<AppState>) -> Result<()> {
    let peer_addr = match &state.config.peer {
        Some(addr) => addr.clone(),
        None => return Ok(()),
    };

    let own_address = state.own_address.clone();

    info!("Connecting to initial peer: {}", peer_addr);
    let client = Client::new();

    // Acquire the lock to update peers
    {
        let mut peers = state.peers.lock().await;
        let peer_info = PeerInfo {
            address: own_address.clone(),
            port: state.actual_port,
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
                    if known_peer.address != own_address.clone()
                        && !peers.contains_key(&known_peer.address)
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

/// Broadcast a message to all connected peers
///
/// # Arguments
/// * `message` - The message to broadcast
/// * `source` - The source of the message (to avoid sending back to sender)
/// * `peers` - Map of peer addresses to their HTTP clients
pub async fn broadcast_to_peers(
    message: Message,
    source: &str,
    peers: &Arc<Mutex<HashMap<String, PeerHealth>>>,
) -> Result<()> {
    // Create a vector of (address, client) pairs that we need to send to
    let targets: Vec<(String, Client)> = {
        // Scope the lock to this block
        let peers_guard = peers.lock().await;
        peers_guard
            .iter()
            .filter(|(addr, _)| *addr != source)
            .map(|(addr, peer_health)| (addr.clone(), peer_health.client.clone()))
            .collect()
    }; // Lock is released here

    // Send the message to each peer
    for (addr, client) in targets {
        if let Err(e) = client
            .post(format!("{}/receive", addr))
            .json(&message)
            .send()
            .await
        {
            error!("Failed to send message to {}: {}", addr, e);
        }
    }

    Ok(())
}

/// Add a new peer to the network
pub async fn add_peer(state: Arc<AppState>, address: String) -> Result<(), String> {
    let mut peers = state.peers.lock().await;
    if !peers.contains_key(&address) {
        let client = Client::new();
        let peer_health = PeerHealth::new(client, address.clone());
        peers.insert(address, peer_health);
    }
    Ok(())
}

/// Add multiple peers to the network
pub async fn add_peers(state: Arc<AppState>, peer_addrs: Vec<String>) -> Result<(), String> {
    for peer_addr in peer_addrs {
        add_peer(state.clone(), peer_addr).await?;
    }
    Ok(())
}

/// Get all known peers
pub async fn get_peers(state: Arc<AppState>) -> Result<Vec<String>, String> {
    let peers = state.peers.lock().await;
    Ok(peers.keys().cloned().collect())
}

/// Remove a peer from the network
pub async fn remove_peer(state: Arc<AppState>, address: String) -> Result<(), String> {
    let mut peers = state.peers.lock().await;
    if peers.remove(&address).is_some() {
        info!("Removed peer: {}", address);
    } else {
        debug!("Peer {} not found", address);
    }
    Ok(())
}

/// Update a peer's last seen timestamp
pub async fn update_peer_last_seen(app_state: Arc<AppState>, peer_addr: String) {
    info!("Updating last seen for peer: {}", peer_addr);
    let mut peers = app_state.peers.lock().await;
    if let Some(peer_health) = peers.get_mut(&peer_addr) {
        peer_health.update_last_seen();
        debug!("Updated last seen for peer: {}", peer_addr);
    } else {
        debug!("Peer {} not found", peer_addr);
    }
}

/// Get peer info from address
pub async fn get_peer_info(address: &str) -> Result<PeerInfo, String> {
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

    use mockito::Server;
    use std::{collections::HashMap, sync::Arc};
    use subversive_database::context::DbContext;
    use subversive_utils::test_utils::init_test_tracing;
    use tokio::sync::Mutex;
    use tracing::info;

    use std::time::Duration;

    use crate::peer::{
        add_peer, add_peers, connect_to_initial_peer, get_peers, remove_peer, update_peer_last_seen,
    };

    async fn setup_test_state(own_address: &str) -> Arc<AppState> {
        let mut config = Config::default_config();
        config.hostname = Some(own_address.to_string());

        let port = 8080;

        Arc::new(AppState {
            config,
            own_address: own_address.to_string(),
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: port,
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

        let mut config = Config::default_config();
        config.peer = Some(peer_addr.clone());

        let state = Arc::new(AppState {
            config,
            own_address: own_addr.clone(),
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: 8080,
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

        let state = Arc::new(AppState {
            config,
            own_address: "https://localhost:8080".to_string(),
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: 8080,
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

        let mut config = Config::default_config();
        config.peer = Some(peer_addr.clone());

        let state = Arc::new(AppState {
            config,
            own_address: own_addr.clone(),
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: 8080,
        });

        let result = connect_to_initial_peer(state.clone()).await;
        assert!(result.is_ok());

        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 2); // Initial peer + 1 known peer (excluding own address)
        assert!(peers.contains_key(&peer_addr));
        assert!(peers.contains_key("https://peer1:8080"));
        assert!(!peers.contains_key(&own_addr));
    }

    #[tokio::test]
    async fn test_add_peer() {
        let state = setup_test_state("https://localhost:8080").await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add a peer
        add_peer(state.clone(), peer_addr.clone()).await.unwrap();

        // Verify peer was added
        let peers = state.peers.lock().await;
        assert!(peers.contains_key(&peer_addr));

        // Try adding same peer again
        drop(peers);
        add_peer(state.clone(), peer_addr.clone()).await.unwrap();

        // Verify no duplicate was added
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 1);
    }

    #[tokio::test]
    async fn test_add_peers() {
        let state = setup_test_state("https://localhost:8080").await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
            "http://localhost:8082".to_string(),
        ];

        // Add multiple peers
        add_peers(state.clone(), peer_addrs.clone()).await.unwrap();

        // Verify all peers were added
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 3);
        for addr in peer_addrs {
            assert!(peers.contains_key(&addr));
        }
    }

    #[tokio::test]
    async fn test_get_peers() {
        let state = setup_test_state("https://localhost:8080").await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
        ];

        // Add peers
        add_peers(state.clone(), peer_addrs.clone()).await.unwrap();

        // Get peers and verify
        let result = get_peers(state.clone()).await.unwrap();
        assert_eq!(result.len(), 2);
        for addr in peer_addrs {
            assert!(result.contains(&addr));
        }
    }

    #[tokio::test]
    async fn test_remove_peer() {
        let state = setup_test_state("https://localhost:8080").await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add and then remove a peer
        add_peer(state.clone(), peer_addr.clone()).await.unwrap();
        remove_peer(state.clone(), peer_addr.clone()).await.unwrap();

        // Verify peer was removed
        let peers = state.peers.lock().await;
        assert!(!peers.contains_key(&peer_addr));

        // Try removing non-existent peer
        drop(peers);
        remove_peer(state.clone(), "http://nonexistent:8080".to_string())
            .await
            .unwrap();
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 0);
    }

    #[tokio::test]
    async fn test_update_peer_last_seen() {
        init_test_tracing();
        let state = setup_test_state("https://localhost:8080").await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add a peer
        add_peer(state.clone(), peer_addr.clone()).await.unwrap();

        // Get initial last seen time
        let peers = state.peers.lock().await;
        let initial_last_seen = peers.get(&peer_addr).unwrap().last_seen;
        drop(peers);

        // Wait a moment
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Update last seen
        update_peer_last_seen(state.clone(), peer_addr.clone()).await;

        // Verify last seen was updated
        let peers = state.peers.lock().await;
        let new_last_seen = peers.get(&peer_addr).unwrap().last_seen;
        info!(
            "Initial last seen: {}, new last seen: {}",
            initial_last_seen, new_last_seen
        );
        assert!(new_last_seen > initial_last_seen);
    }

    #[tokio::test]
    async fn test_update_peer_last_seen_non_existent() {
        init_test_tracing();
        let state = setup_test_state("https://localhost:8080").await;
        // Try updating non-existent peer
        update_peer_last_seen(state.clone(), "http://nonexistent:8080".to_string()).await;
        // Should not panic or affect existing peers
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 0);
    }
}
