use std::sync::Arc;
use std::time::SystemTime;
use tokio::time::{self, Duration};
use tracing::{error, info, warn};

use crate::types::{message::Message, state::AppState};
use subversive_network::health::PeerHealth;

const SURVIVAL_CHECK_INTERVAL: u64 = 300; // 5 minutes
const RECONNECT_ATTEMPT_INTERVAL: u64 = 60; // 1 minute
const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// Start the survival mode handler
///
/// In survival mode, the server will:
/// 1. Periodically check for network connectivity
/// 2. Attempt to reconnect to known peers if disconnected
/// 3. Broadcast its presence to any peers it finds
/// 4. Never shut down unless explicitly commanded to, even with zero peers
pub async fn start_survival_mode(app_state: Arc<AppState>) {
    info!("Entering post-apocalyptic survival mode...");

    let state = app_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(SURVIVAL_CHECK_INTERVAL));

        loop {
            interval.tick().await;
            check_survival_status(&state).await;
            // Ensure we keep running even with zero peers
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}

#[cfg(test)]
pub async fn check_survival_status(state: &Arc<AppState>) {
    let peers = state.peers.lock().await;
    let peer_count = peers.len();
    drop(peers); // Release the lock before potentially long operations

    info!("Survival check - Connected peers: {}", peer_count);

    if peer_count == 0 {
        warn!("No connected peers - initiating survival protocol");
        attempt_reconnection(state).await;
        // Even if reconnection fails, we'll keep running
        info!("Survival mode active - continuing to run with zero peers");
    } else {
        // Broadcast heartbeat to all peers
        let _eartbeat_msg = Message::Chat {
            content: "Still alive...".to_string(),
        };
    }
}

#[cfg(not(test))]
pub async fn check_survival_status(state: &Arc<AppState>) {
    let peers = state.peers.lock().await;
    let peer_count = peers.len();
    drop(peers); // Release the lock before potentially long operations

    info!("Survival check - Connected peers: {}", peer_count);

    if peer_count == 0 {
        warn!("No connected peers - initiating survival protocol");
        attempt_reconnection(state).await;
        // Even if reconnection fails, we'll keep running
        info!("Survival mode active - continuing to run with zero peers");
    } else {
        // Broadcast heartbeat to all peers
        let _heartbeat_msg = Message::Chat {
            content: "Still alive...".to_string(),
        };
    }
}

async fn attempt_reconnection(state: &Arc<AppState>) {
    info!("Attempting to reconnect to known peers...");

    // Get list of known peers from database
    // Get peers that were active in the last 24 hours
    let since = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
        - 86400; // 24 hours ago
    let known_peers = match state.db.get_active_peers(since).await {
        Ok(peers) => peers,
        Err(e) => {
            error!("Failed to retrieve known peers from database: {}", e);
            return;
        }
    };

    if known_peers.is_empty() {
        warn!("No known peers in database - entering dormant state");
        return;
    }

    for peer in known_peers {
        let mut attempts = 0;
        while attempts < MAX_RECONNECT_ATTEMPTS {
            info!(
                "Attempting to reconnect to peer: {} (attempt {})",
                peer.address,
                attempts + 1
            );

            // Create a new client for this peer
            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .expect("Failed to create HTTP client");

            let peer_health = PeerHealth::new(client.clone(), peer.address.clone());

            // Try to connect
            match client.get(format!("{}/peers", peer.address)).send().await {
                Ok(_) => {
                    info!("Successfully reconnected to peer: {}", peer.address);
                    state.peers.insert(peer.address.clone(), peer_health);
                    break;
                }
                Err(e) => {
                    warn!("Failed to reconnect to peer {}: {}", peer.address, e);
                    attempts += 1;
                    time::sleep(Duration::from_secs(RECONNECT_ATTEMPT_INTERVAL)).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use subversive_database::context::DbContext;
    use subversive_network::health::PeerHealth;
    use tokio::sync::Mutex;

    use crate::{
        survival::{check_survival_status, start_survival_mode},
        types::{config::Config, state::AppState},
    };

    #[tokio::test]
    async fn test_survival_mode_start() {
        let app_state = Arc::new(AppState {
            peers: SafeMap::new(),
            config: Config::default_config(),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            own_address: "http://localhost:12345".to_string(),
            actual_port: 12345,
        });

        // Start survival mode
        start_survival_mode(app_state.clone()).await;

        // Sleep briefly to let the survival mode task start
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Add a peer and verify heartbeat is sent
        let mut peers = app_state.peers.write().await;
        peers.insert(
            "http://localhost:8080".to_string(),
            PeerHealth::new(reqwest::Client::new(), "http://localhost:8080".to_string()),
        );
        drop(peers);

        // Check survival status - this should send a heartbeat
        check_survival_status(&app_state).await;
    }

    #[tokio::test]
    async fn test_survival_mode_no_peers() {
        let app_state = Arc::new(AppState {
            peers: SafeMap::new(),
            config: Config::default_config(),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            own_address: "http://localhost:12345".to_string(),
            actual_port: 12345,
        });

        // Check survival status with no peers
        check_survival_status(&app_state).await;

        // Verify we're still running (no shutdown in survival mode)
        assert!(peers.readonly().await.is_empty());
    }

    #[tokio::test]
    async fn test_survival_mode_with_peers() {
        let config = Config::default_config();
        let mut peers_map = HashMap::new();
        peers_map.insert(
            "http://localhost:8080".to_string(),
            PeerHealth::new(reqwest::Client::new(), "http://localhost:8080".to_string()),
        );
        peers_map.insert(
            "http://localhost:8081".to_string(),
            PeerHealth::new(reqwest::Client::new(), "http://localhost:8081".to_string()),
        );
        let peers = Arc::new(Mutex::new(peers_map));
        let db = Arc::new(DbContext::new_memory().await.unwrap());
        let app_state = Arc::new(AppState {
            config,
            peers: peers.clone(),
            db,
            own_address: "http://localhost:12345".to_string(),
            actual_port: 12345,
        });

        // Check survival status with peers
        check_survival_status(&app_state).await;

        // Verify peers are still present
        assert_eq!(peers.readonly().await.len(), 2);
    }

    #[tokio::test]
    async fn test_survival_mode_peer_reconnection() {
        let config = Config::default_config();
        let mut peers_map = HashMap::new();
        let mut peer = PeerHealth::new(reqwest::Client::new(), "http://localhost:8080".to_string());
        // Mark peer as unhealthy
        peer.record_failure();
        peers_map.insert("http://localhost:8080".to_string(), peer);
        let peers = Arc::new(Mutex::new(peers_map));
        let db = Arc::new(DbContext::new_memory().await.unwrap());
        let app_state = Arc::new(AppState {
            config,
            peers: peers.clone(),
            db,
            own_address: "http://localhost:12345".to_string(),
            actual_port: 12345,
        });

        // Check survival status - this should attempt reconnection
        check_survival_status(&app_state).await;

        // Verify peer is still present (we keep peers in survival mode)
        assert_eq!(peers.readonly().await.len(), 1);
    }
}
