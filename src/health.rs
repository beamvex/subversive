use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time};
use tracing::{info, warn};

use crate::{shutdown::ShutdownState, types::health::PeerHealth, AppState, HeartbeatMessage};

const MAX_FAILED_CHECKS: u32 = 3;

/// Start a background task that periodically checks the health of all peers
pub async fn start_health_checker(app_state: Arc<AppState>) {
    let peers_clone = app_state.peers.clone();
    let own_address = app_state.own_address.clone();
    let shutdown_state = app_state.shutdown.clone();
    let survival_mode = app_state.config.survival_mode.unwrap_or(false);

    tokio::spawn(async move {
        loop {
            let (peers_to_check, known_peers) = {
                let peers = peers_clone.lock().await;
                let peer_count = peers.len();
                info!("Current peer count: {}", peer_count);

                if peer_count == 0 {
                    warn!("No peers remaining in network");
                    if !survival_mode {
                        warn!("Not in survival mode - initiating shutdown...");
                        shutdown_state.shutdown().await;
                    }
                }

                let known_peers = peers.keys().cloned().collect::<Vec<_>>();
                let peers_map = peers
                    .iter()
                    .map(|(addr, health)| {
                        let addr = if !addr.starts_with("https://") {
                            addr.replace("http://", "https://")
                        } else {
                            addr.clone()
                        };
                        (addr, health.client.clone())
                    })
                    .collect::<HashMap<String, reqwest::Client>>();
                (peers_map, known_peers)
            }; // Lock is dropped here

            check_peer_health(
                &peers_clone,
                &peers_to_check,
                &own_address,
                &known_peers,
                &shutdown_state,
                survival_mode,
            )
            .await;
            time::sleep(Duration::from_secs(30)).await;
        }
    });
}

/// Check the health of all connected peers by sending heartbeat requests
///
/// # Arguments
/// * `peers_state` - Shared state containing peer health information
/// * `peers` - Map of peer addresses to their HTTP clients
/// * `own_address` - Our own address that peers can use to connect to us
/// * `known_peers` - List of all known peer addresses
/// * `shutdown_state` - Shared state for handling shutdown
/// * `survival_mode` - Whether the server is running in survival mode
async fn check_peer_health(
    peers_state: &Arc<Mutex<HashMap<String, PeerHealth>>>,
    peers: &HashMap<String, reqwest::Client>,
    own_address: &str,
    known_peers: &[String],
    shutdown_state: &Arc<ShutdownState>,
    survival_mode: bool,
) {
    for (addr, client) in peers.iter() {
        // Extract port from address
        let port = addr
            .split(':')
            .last()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(0);

        let heartbeat = HeartbeatMessage {
            port,
            address: own_address.to_string(),
            known_peers: known_peers.to_vec(),
        };

        let result = client
            .post(format!("{}/heartbeat", addr))
            .json(&heartbeat)
            .send()
            .await;

        let mut peers = peers_state.lock().await;
        if let Some(peer_health) = peers.get_mut(addr) {
            match result {
                Ok(_) => {
                    // Reset failure counter on successful health check
                    peer_health.reset_failures();
                }
                Err(e) => {
                    // Increment failure counter and remove peer if it exceeds max failures
                    let failures = peer_health.record_failure();
                    if failures >= MAX_FAILED_CHECKS {
                        warn!(
                            "Removing peer {} after {} failed health checks",
                            addr, failures
                        );
                        peers.remove(addr);
                        let remaining_peers = peers.len();
                        info!("Peers remaining after removal: {}", remaining_peers);

                        if remaining_peers == 0 {
                            warn!("No peers remaining in network");
                            if !survival_mode {
                                warn!("Not in survival mode - initiating shutdown...");
                                drop(peers); // Drop the lock before shutdown
                                shutdown_state.shutdown().await;
                            }
                        }
                    } else {
                        warn!(
                            "Failed to send heartbeat to {} (attempt {}): {}",
                            addr, failures, e
                        );
                    }
                }
            }
        }
    }
}
