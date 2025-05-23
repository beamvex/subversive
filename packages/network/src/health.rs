use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

use subversive_types::peer_health::PeerHealth;

/// Handle a health check result
async fn handle_health_check_result(
    peers: &mut HashMap<String, PeerHealth>,
    addr: String,
    result: Result<(), Box<dyn std::error::Error + Send + Sync>>,
    survival_mode: bool,
) {
    if let Some(peer_health) = peers.get_mut(&addr) {
        match result {
            Ok(_) => {
                peer_health.update_last_seen();
            }
            Err(e) => {
                debug!("Health check failed for {}: {}", addr, e);
                peers.remove(&addr);
            }
        }
    }

    // In survival mode, if we have no peers and no gateways, shut down
    if survival_mode && peers.is_empty() {
        info!("No peers or gateways available in survival mode, shutting down");
    }
}

/// Check the health of all peers
pub async fn check_peer_health(
    peers: &HashMap<String, PeerHealth>,
    address: &str,
) -> Result<(), String> {
    if !peers.contains_key(address) {
        return Err("Peer not found".to_string());
    }
    Ok(())
}

/// Check the health of a specific peer
pub async fn check_peer(
    addr: &str,
    peer_health: &mut PeerHealth,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match peer_health.client.get(addr).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!("Health check passed for {}", addr);
                peer_health.update_last_seen();
                info!("Peer {} last seen: {}", addr, peer_health.get_last_seen());
                Ok(())
            } else {
                debug!("Failed health check for {}: {}", addr, response.status());
                Err("Health check failed".into())
            }
        }
        Err(e) => {
            debug!("Failed to connect to {}: {}", addr, e);
            Err(e.into())
        }
    }
}

/// Check the health of all peers periodically
pub async fn start_health_checker(peers: Arc<Mutex<HashMap<String, PeerHealth>>>) {
    info!("Starting health checker");
    loop {
        let mut peers_guard = peers.lock().await;
        let peer_list = peers_guard.keys().cloned().collect::<Vec<String>>();

        for peer in peer_list {
            if let Some(peer_health) = peers_guard.get_mut(&peer) {
                let result = check_peer(&peer, peer_health).await;
                handle_health_check_result(&mut peers_guard, peer.clone(), result, false).await;
            }
        }
        drop(peers_guard);

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_handle_health_check_result() {
        let mut peers = HashMap::new();
        let client = reqwest::Client::new();
        peers.insert(
            "test_peer".to_string(),
            PeerHealth::new(client, "test_peer".to_string()),
        );

        // Test successful health check
        handle_health_check_result(&mut peers, "test_peer".to_string(), Ok(()), false).await;
        assert!(peers.contains_key("test_peer"));

        // Test failed health check
        handle_health_check_result(
            &mut peers,
            "test_peer".to_string(),
            Err("Test error".into()),
            false,
        )
        .await;
        assert!(!peers.contains_key("test_peer"));
    }
}
