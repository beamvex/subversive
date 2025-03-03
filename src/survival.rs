use std::sync::Arc;
use std::time::SystemTime;
use tokio::time::{self, Duration};
use tracing::{error, info, warn};

use crate::{
    peer::broadcast_to_peers,
    types::{health::PeerHealth, message::Message, state::AppState},
};

const SURVIVAL_CHECK_INTERVAL: u64 = 300; // 5 minutes
const RECONNECT_ATTEMPT_INTERVAL: u64 = 60; // 1 minute
const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// Start the survival mode handler
///
/// In survival mode, the server will:
/// 1. Periodically check for network connectivity
/// 2. Attempt to reconnect to known peers if disconnected
/// 3. Broadcast its presence to any peers it finds
/// 4. Never shut down unless explicitly commanded to
pub async fn start_survival_mode(app_state: Arc<AppState>) {
    info!("Entering post-apocalyptic survival mode...");

    let state = app_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(SURVIVAL_CHECK_INTERVAL));

        loop {
            interval.tick().await;
            check_survival_status(&state).await;
        }
    });
}

async fn check_survival_status(state: &Arc<AppState>) {
    let peers = state.peers.lock().await;
    let peer_count = peers.len();

    info!("Survival check - Connected peers: {}", peer_count);

    if peer_count == 0 {
        warn!("No connected peers - initiating survival protocol");
        attempt_reconnection(state).await;
    } else {
        // Broadcast heartbeat to all peers
        let heartbeat_msg = Message::Chat {
            content: "Still alive...".to_string(),
        };

        if let Err(e) = broadcast_to_peers(heartbeat_msg, "survival", &state.peers).await {
            error!("Failed to broadcast survival message: {}", e);
        }
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
    let known_peers = match state.db.get_active_peers(since) {
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

            let peer_health = PeerHealth::new(client.clone());

            // Try to connect
            match client.get(&format!("{}/peers", peer.address)).send().await {
                Ok(_) => {
                    info!("Successfully reconnected to peer: {}", peer.address);
                    let mut peers = state.peers.lock().await;
                    peers.insert(peer.address.clone(), peer_health);
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
