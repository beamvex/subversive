use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{sync::Mutex, time};
use tracing::{info, warn};

use crate::{
    shutdown::ShutdownState, types::health::PeerHealth, types::message::HeartbeatMessage, AppState,
};

const MAX_FAILED_CHECKS: u32 = 3;

/// Ensure a URL uses HTTPS
fn ensure_https_url(url: &str) -> String {
    if !url.starts_with("https://") {
        url.replace("http://", "https://")
    } else {
        url.to_string()
    }
}

/// Handle peer count checking and potential shutdown
async fn handle_peer_count(
    peer_count: usize,
    survival_mode: bool,
    shutdown_state: &Arc<ShutdownState>,
) {
    info!("Current peer count: {}", peer_count);

    if peer_count == 0 {
        warn!("No peers remaining in network");
        if !survival_mode {
            warn!("Not in survival mode - initiating shutdown...");
            shutdown_state.shutdown().await;
        }
    }
}

/// Handle a peer health check failure
async fn handle_peer_failures(
    peers: &mut HashMap<String, PeerHealth>,
    survival_mode: bool,
    shutdown_state: &Arc<ShutdownState>,
) {
    // Collect peers to remove to avoid borrowing issues
    let peers_to_remove: Vec<String> = peers
        .iter()
        .filter(|(_, health)| health.failed_checks >= MAX_FAILED_CHECKS)
        .map(|(addr, health)| {
            warn!(
                "Removing peer {} after {} failed health checks",
                addr, health.failed_checks
            );
            addr.clone()
        })
        .collect();

    // Remove the failed peers
    for addr in peers_to_remove {
        peers.remove(&addr);
    }

    let remaining_peers = peers.len();
    info!("Peers remaining after removal: {}", remaining_peers);

    if remaining_peers == 0 {
        warn!("No peers remaining in network");
        if !survival_mode {
            warn!("Not in survival mode - initiating shutdown...");
            shutdown_state.shutdown().await;
        }
    }
}

/// Handle the result of a peer health check
async fn handle_health_check_result(
    peers_state: &Arc<Mutex<HashMap<String, PeerHealth>>>,
    addr: &str,
    result: Result<reqwest::Response, reqwest::Error>,
    survival_mode: bool,
    shutdown_state: &Arc<ShutdownState>,
) {
    let mut peers = peers_state.lock().await;
    if let Some(peer_health) = peers.get_mut(addr) {
        match result {
            Ok(_) => {
                // Reset failure counter on successful health check
                peer_health.reset_failures();
            }
            Err(e) => {
                let failures = peer_health.record_failure();
                warn!(
                    "Failed to send heartbeat to {} (attempt {}): {}",
                    addr, failures, e
                );
            }
        }
    }

    handle_peer_failures(&mut peers, survival_mode, shutdown_state).await;
}

/// Get the current peers and their clients for health checking
async fn get_peers_for_health_check(
    peers_state: &Arc<Mutex<HashMap<String, PeerHealth>>>,
    survival_mode: bool,
    shutdown_state: &Arc<ShutdownState>,
) -> (HashMap<String, reqwest::Client>, Vec<String>) {
    let peers = peers_state.lock().await;
    let peer_count = peers.len();
    handle_peer_count(peer_count, survival_mode, shutdown_state).await;

    let known_peers = peers.keys().cloned().collect::<Vec<_>>();
    let peers_map = peers
        .iter()
        .map(|(addr, health)| {
            let addr = ensure_https_url(addr);
            (addr, health.client.clone())
        })
        .collect::<HashMap<String, reqwest::Client>>();
    (peers_map, known_peers)
}

/// Start a background task that periodically checks the health of all peers
pub async fn start_health_checker(app_state: Arc<AppState>) {
    let peers_clone = app_state.peers.clone();
    let own_address = app_state.own_address.clone();
    let shutdown_state = app_state.shutdown.clone();
    let survival_mode = app_state.config.survival_mode.unwrap_or(false);

    tokio::spawn(async move {
        loop {
            let (peers_to_check, known_peers) =
                get_peers_for_health_check(&peers_clone, survival_mode, &shutdown_state).await;

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

        handle_health_check_result(peers_state, addr, result, survival_mode, shutdown_state).await;
    }
}
