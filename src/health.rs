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
            let (peers_to_check, known_peers) = {
                let peers = peers_clone.lock().unwrap();
                let known_peers = peers.keys().cloned().collect::<Vec<_>>();
                let peers_map = peers
                    .iter()
                    .map(|(addr, client)| {
                        let addr = if !addr.starts_with("https://") {
                            addr.replace("http://", "https://")
                        } else {
                            addr.clone()
                        };
                        (addr, client.clone())
                    })
                    .collect::<HashMap<String, reqwest::Client>>();
                (peers_map, known_peers)
            }; // Lock is dropped here

            check_peer_health(&peers_to_check, &own_address, &known_peers).await;
            tokio::time::sleep(Duration::from_secs(30)).await;
        }
    });
}

/// Check the health of all connected peers by sending heartbeat requests
///
/// # Arguments
/// * `peers` - Map of peer addresses to their HTTP clients
/// * `own_address` - Our own address that peers can use to connect to us
/// * `known_peers` - List of all known peer addresses
async fn check_peer_health(peers: &HashMap<String, reqwest::Client>, own_address: &str, known_peers: &[String]) {
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
