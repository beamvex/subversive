use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use subversive_utils::trace::types::{
    BuildHttpClient, PeerAddOwn, PeerAddRequest, PeerAlreadyConnected, PeerConnect,
    PeerConnectError, PeerConnected, PeerKnownCount, PeerLastSeen, PeerLastSeenCheck, PeerNotFound,
    PeerRemoved, PeerResponse,
};
use subversive_utils::{trace_debug, trace_error, trace_info, TraceId};

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
pub async fn connect_to_peer(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    initial_peer: Option<String>,
    own_address: String,
    own_port: u16,
    process: String,
) -> Result<()> {
    let peer_addr = match initial_peer {
        Some(addr) => addr,
        None => return Ok(()),
    };

    trace_info!(PeerConnect {
        addr: peer_addr.clone(),
        process: process.clone()
    });
    trace_info!(BuildHttpClient {
        process: process.clone()
    });
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    // Check if we're already connected to this peer and seen recently
    {
        let peers = peers.lock().await;
        if peers.contains_key(&peer_addr) {
            let peer = peers.get(&peer_addr).unwrap();
            let current_time = Utc::now().timestamp();
            trace_info!(PeerLastSeenCheck {
                addr: peer_addr.clone(),
                last_seen: peer.last_seen,
                process: process.clone()
            });

            // Only skip connection if peer was seen in the last 5 seconds
            if current_time - peer.last_seen < 5 {
                trace_info!(PeerAlreadyConnected {
                    addr: peer_addr.clone(),
                    process: process.clone()
                });
                return Ok(());
            }
        }
    }

    trace_info!(PeerAddOwn {
        addr: peer_addr.clone(),
        process: process.clone()
    });
    let peer_info = PeerInfo {
        address: own_address.clone(),
        port: own_port,
    };

    trace_info!(PeerAddRequest {
        addr: peer_addr.clone(),
        process: process.clone()
    });

    // Send connection request to peer
    let response = client
        .post(format!("{}/peer", peer_addr))
        .json(&peer_info)
        .send()
        .await?;

    trace_info!(PeerResponse {
        addr: peer_addr.clone(),
        status: response.status().to_string(),
        process: process.clone()
    });

    if !response.status().is_success() {
        trace_error!(PeerConnectError {
            addr: peer_addr.clone(),
            error: response.status().to_string(),
            process: process.clone()
        });
        return Ok(());
    }

    trace_info!(PeerConnected {
        addr: peer_addr.clone(),
        process: process.clone()
    });

    // Get the peer's known peers before acquiring the lock
    let known_peers = response.json::<Vec<PeerInfo>>().await.unwrap_or_default();
    trace_info!(PeerKnownCount {
        addr: peer_addr.clone(),
        count: known_peers.len(),
        process: process.clone()
    });

    // Now acquire the lock to update our peer list
    let mut peers = peers.lock().await;

    // Add the initial peer if we haven't already
    peers
        .entry(peer_addr.clone())
        .or_insert_with(|| PeerHealth::new(client.clone(), peer_addr.clone()));

    // Filter and collect new peers we haven't seen yet
    let new_peers: Vec<_> = known_peers
        .into_iter()
        .filter(|p| p.address != own_address && !peers.contains_key(&p.address))
        .collect();

    // Add all new peers
    for known_peer in new_peers {
        let peer_client = Client::new();
        let peer_health = PeerHealth::new(peer_client, known_peer.address.clone());
        peers.insert(known_peer.address.clone(), peer_health);
    }

    Ok(())
}

/// Add a new peer to the network
///
/// # Arguments
/// * `peers` - Map of peer addresses to their health status
/// * `address` - Address of the peer to add
/// * `process` - Process identifier
/// * `timestamp` - Optional timestamp for when the peer was added
pub async fn add_peer(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    address: String,
    process: String,
    timestamp: Option<DateTime<Utc>>,
) -> Result<(), String> {
    let mut peers = peers.lock().await;
    peers.entry(address.clone()).or_insert_with(|| {
        trace_debug!(BuildHttpClient {
            process: process.clone()
        });
        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let mut peer_health = PeerHealth::new(client, address.clone());
        if let Some(ts) = timestamp {
            peer_health.last_seen = ts.timestamp();
        }
        peer_health
    });
    Ok(())
}

/// Add multiple peers to the network
pub async fn add_peers(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    peer_addrs: Vec<String>,
    process: String,
) -> Result<(), String> {
    for peer_addr in peer_addrs {
        add_peer(peers.clone(), peer_addr, process.clone(), None).await?;
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
    process: String,
) -> Result<(), String> {
    let mut peers = peers.lock().await;
    if peers.remove(&address).is_some() {
        trace_info!(PeerRemoved {
            addr: address.clone(),
            process: process.clone()
        });
    } else {
        trace_debug!(PeerNotFound {
            addr: address.clone(),
            process: process.clone()
        });
    }
    Ok(())
}

/// Update a peer's last seen timestamp
pub async fn update_peer_last_seen(
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    peer_addr: String,
    process: String,
) {
    trace_info!(PeerLastSeen {
        addr: peer_addr.clone(),
        process: process.clone()
    });
    let mut peers = peers.lock().await;
    if let Some(peer_health) = peers.get_mut(&peer_addr) {
        peer_health.update_last_seen();
        trace_debug!(PeerLastSeen {
            addr: peer_addr.clone(),
            process: process.clone()
        });
    } else {
        trace_debug!(PeerNotFound {
            addr: peer_addr.clone(),
            process: process.clone()
        });
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
    async fn test_connect_to_peer_no_peer_configured() {
        let peers = setup_test_peers().await;
        let result = connect_to_peer(peers, None, "https://localhost:8080".to_string(), 8080).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connect_to_peer_success() {
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

        let result = connect_to_peer(
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
    async fn test_connect_to_peer_failure() {
        let mut mock_server = Server::new_async().await;
        let peer_addr = mock_server.url();
        let peers = setup_test_peers().await;

        let _m = mock_server
            .mock("POST", "/peer")
            .with_status(500)
            .create_async()
            .await;

        let result = connect_to_peer(
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
    async fn test_connect_to_peer_skip_own_address() {
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

        let result = connect_to_peer(
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
    async fn test_add_peer() -> Result<(), String> {
        let peers = setup_test_peers().await;
        let peer_addr = "http://localhost:8080";

        add_peer(
            peers.clone(),
            peer_addr.to_string(),
            "test".to_string(),
            None,
        )
        .await?;

        let peers = peers.lock().await;
        assert!(peers.contains_key(peer_addr));
        Ok(())
    }

    #[tokio::test]
    async fn test_add_peers() -> Result<(), String> {
        let peers = setup_test_peers().await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
            "http://localhost:8082".to_string(),
        ];

        add_peer(
            peers.clone(),
            peer_addrs[0].clone(),
            "test".to_string(),
            None,
        )
        .await?;
        add_peer(
            peers.clone(),
            peer_addrs[1].clone(),
            "test".to_string(),
            None,
        )
        .await?;
        add_peer(
            peers.clone(),
            peer_addrs[2].clone(),
            "test".to_string(),
            None,
        )
        .await?;

        let peers = peers.lock().await;
        for addr in peer_addrs {
            assert!(peers.contains_key(&addr));
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_peers() {
        let peers = setup_test_peers().await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
        ];

        add_peer(
            peers.clone(),
            peer_addrs[0].clone(),
            "test".to_string(),
            None,
        )
        .await
        .unwrap();
        add_peer(
            peers.clone(),
            peer_addrs[1].clone(),
            "test".to_string(),
            None,
        )
        .await
        .unwrap();

        let result = get_peers(peers.clone()).await.unwrap();
        assert_eq!(result.len(), 2);
        for addr in peer_addrs {
            assert!(result.contains(&addr));
        }
    }

    #[tokio::test]
    async fn test_remove_peer() -> Result<(), String> {
        let peers = setup_test_peers().await;
        let peer_addr = "http://localhost:8080";

        add_peer(
            peers.clone(),
            peer_addr.to_string(),
            "test".to_string(),
            None,
        )
        .await?;

        let peers_guard = peers.lock().await;
        assert!(peers_guard.contains_key(peer_addr));
        drop(peers_guard);

        remove_peer(peers.clone(), peer_addr.to_string(), "test".to_string()).await?;

        let peers = peers.lock().await;
        assert!(!peers.contains_key(peer_addr));
        Ok(())
    }

    #[tokio::test]
    async fn test_update_peer_last_seen() -> Result<(), String> {
        init_test_tracing();
        let peers = setup_test_peers().await;
        let peer_addr = "http://localhost:8080";

        add_peer(
            peers.clone(),
            peer_addr.to_string(),
            "test".to_string(),
            None,
        )
        .await?;

        let peers_guard = peers.lock().await;
        let initial_last_seen = peers_guard.get(peer_addr).unwrap().last_seen;
        drop(peers_guard);

        // Wait a bit to ensure the timestamp changes
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        update_peer_last_seen(peers.clone(), peer_addr.to_string(), "test".to_string()).await?;

        let peers = peers.lock().await;
        let updated_last_seen = peers.get(peer_addr).unwrap().last_seen;

        assert!(updated_last_seen > initial_last_seen);
        Ok(())
    }

    #[tokio::test]
    async fn test_update_peer_last_seen_non_existent() -> Result<(), String> {
        init_test_tracing();
        let peers = setup_test_peers().await;
        // Try updating non-existent peer
        update_peer_last_seen(peers.clone(), "http://nonexistent:8080".to_string()).await;
        // Should not panic or affect existing peers
        let peers_guard = peers.lock().await;
        assert_eq!(peers_guard.len(), 0);
    }
}
