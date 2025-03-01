use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio;
use tracing::info;

use crate::{AppState, HeartbeatMessage};

/// Start a background task that periodically checks the health of all peers
pub async fn start_health_checker(app_state: Arc<AppState>) {
    let peers_clone = app_state.peers.clone();
    let own_address = app_state.own_address.clone();
    tokio::spawn(async move {
        loop {
            let peers_to_check = {
                let peers = peers_clone.lock().unwrap();
                peers
                    .iter()
                    .map(|(addr, client)| {
                        let addr = if !addr.starts_with("https://") {
                            addr.replace("http://", "https://")
                        } else {
                            addr.clone()
                        };
                        (addr, client.clone())
                    })
                    .collect::<HashMap<String, reqwest::Client>>()
            }; // Lock is dropped here

            check_peer_health(&peers_to_check, &own_address).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

/// Check the health of all connected peers by sending heartbeat requests
///
/// # Arguments
/// * `peers` - Map of peer addresses to their HTTP clients
/// * `own_address` - Our own address that peers can use to connect to us
async fn check_peer_health(peers: &HashMap<String, reqwest::Client>, own_address: &str) {
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
        };

        if let Err(e) = client
            .post(format!("{}/heartbeat", addr))
            .json(&heartbeat)
            .send()
            .await
        {
            info!("Failed to send heartbeat to {}: {}", addr, e);
        }
    }
}
