use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};
use subversive_types::{
    health::PeerHealth,
    state::{AppState, ShutdownState},
};

const PEER_TIMEOUT: i64 = 3600; // 1 hour

/// Handle a health check result
async fn handle_health_check_result(
    state: &Arc<AppState>,
    addr: String,
    result: Result<(), Box<dyn std::error::Error + Send + Sync>>,
    survival_mode: bool,
) {
    let mut peers = state.peers.lock().await;
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
    if survival_mode && peers.is_empty() && state.shutdown == ShutdownState::Running {
        info!("No peers or gateways available in survival mode, shutting down");
        state.shutdown = ShutdownState::ShuttingDown;
    }
}

/// Check the health of all peers
pub async fn check_peer_health(
    state: Arc<AppState>,
    address: String,
) -> Result<(), String> {
    let peers = state.peers.lock().await;
    if !peers.contains_key(&address) {
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
pub async fn start_health_checker(state: Arc<AppState>) {
    info!("Starting health checker");
    loop {
        let peers = state.peers.lock().await;
        let peer_list = peers.keys().cloned().collect::<Vec<String>>();
        drop(peers);

        for peer in peer_list {
            if let Some(peer_health) = state.peers.lock().await.get_mut(&peer) {
                let result = check_peer(&peer, peer_health).await;
                handle_health_check_result(state.clone(), peer.clone(), result, false).await;
            }
        }

        if state.shutdown == ShutdownState::ShuttingDown {
            break;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
    info!("Health checker stopped");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_peer_health() {
        let state = Arc::new(AppState {
            peers: Arc::new(Mutex::new(vec!["127.0.0.1:8080".to_string()])),
            config: Default::default(),
            own_address: Default::default(),
            shutdown: ShutdownState::Running,
        });

        // Test valid peer
        let result = check_peer_health(state.clone(), "127.0.0.1:8080".to_string()).await;
        assert!(result.is_ok());

        // Test invalid peer
        let result = check_peer_health(state.clone(), "invalid:8080".to_string()).await;
        assert!(result.is_err());
    }
}
