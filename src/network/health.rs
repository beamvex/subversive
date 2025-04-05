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
