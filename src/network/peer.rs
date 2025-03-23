use crate::types::{health::PeerHealth, peer::PeerInfo, state::AppState};
use anyhow::Result;
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, info};

/// Initialize connection to an initial peer
///
/// # Arguments
/// * `state` - Shared application state
pub async fn connect_to_initial_peer(state: Arc<AppState>) -> Result<()> {
    let peer_addr = match &state.config.peer {
        Some(addr) => addr.clone(),
        None => return Ok(()),
    };

    let own_address = state.own_address.clone();

    info!("Connecting to initial peer: {}", peer_addr);
    let client = Client::new();

    // Acquire the lock to update peers
    {
        let mut peers = state.peers.lock().await;
        let peer_info = PeerInfo {
            address: own_address.clone(),
        };

        // Send connection request to peer
        let response = client
            .post(format!("{}/peer", peer_addr))
            .json(&peer_info)
            .send()
            .await?;

        if response.status().is_success() {
            info!("Successfully connected to peer: {}", peer_addr);

            // Add the initial peer
            let peer_health = PeerHealth::new(client.clone(), peer_addr.clone());
            peers.insert(peer_addr.clone(), peer_health);

            // Get and add the peer's known peers
            if let Ok(known_peers) = response.json::<Vec<PeerInfo>>().await {
                info!(
                    "Received {} known peers from {}",
                    known_peers.len(),
                    peer_addr
                );

                for known_peer in known_peers {
                    if known_peer.address != own_address.clone()
                        && !peers.contains_key(&known_peer.address)
                    {
                        let peer_client = Client::new();

                        let peer_health = PeerHealth::new(peer_client, known_peer.address.clone());
                        peers.insert(known_peer.address.clone(), peer_health);
                    }
                }
            }
        } else {
            error!("Failed to connect to peer: {}", response.status());
        }
    }

    Ok(())
}

/// Broadcast a message to all connected peers
///
/// # Arguments
/// * `message` - The message to broadcast
/// * `source` - The source of the message (to avoid sending back to sender)
/// * `peers` - Map of peer addresses to their HTTP clients
pub async fn broadcast_to_peers(
    message: crate::types::message::Message,
    source: &str,
    peers: &Arc<Mutex<HashMap<String, PeerHealth>>>,
) -> Result<()> {
    // Create a vector of (address, client) pairs that we need to send to
    let targets: Vec<(String, Client)> = {
        // Scope the lock to this block
        let peers_guard = peers.lock().await;
        peers_guard
            .iter()
            .filter(|(addr, _)| *addr != source)
            .map(|(addr, peer_health)| (addr.clone(), peer_health.client.clone()))
            .collect()
    }; // Lock is released here

    // Send the message to each peer
    for (addr, client) in targets {
        if let Err(e) = client
            .post(format!("{}/receive", addr))
            .json(&message)
            .send()
            .await
        {
            error!("Failed to send message to {}: {}", addr, e);
        }
    }

    Ok(())
}

/// Add a new peer to the network
pub async fn add_peer(app_state: Arc<AppState>, peer_addr: String) {
    let mut peers = app_state.peers.lock().await;

    // Skip if we already know about this peer
    if peers.contains_key(&peer_addr) {
        debug!("Peer {} already known", peer_addr);
        return;
    }

    info!("Adding new peer: {}", peer_addr);

    // Create a new HTTP client for this peer
    let client = Client::new();

    // Create a new peer health tracker
    let peer_health = PeerHealth::new(client, peer_addr.clone());

    // Add the peer to our list
    peers.insert(peer_addr.clone(), peer_health);

    info!("Successfully added peer: {}", peer_addr);
}

/// Add multiple peers to the network
pub async fn add_peers(app_state: Arc<AppState>, peer_addrs: Vec<String>) {
    for peer_addr in peer_addrs {
        add_peer(app_state.clone(), peer_addr).await;
    }
}

/// Get all known peers
pub async fn get_peers(app_state: Arc<AppState>) -> Vec<String> {
    let peers = app_state.peers.lock().await;
    peers.keys().cloned().collect()
}

/// Remove a peer from the network
pub async fn remove_peer(app_state: Arc<AppState>, peer_addr: String) {
    let mut peers = app_state.peers.lock().await;
    if peers.remove(&peer_addr).is_some() {
        info!("Removed peer: {}", peer_addr);
    } else {
        debug!("Peer {} not found", peer_addr);
    }
}

/// Update a peer's last seen timestamp
pub async fn update_peer_last_seen(app_state: Arc<AppState>, peer_addr: String) {
    let mut peers = app_state.peers.lock().await;
    if let Some(peer_health) = peers.get_mut(&peer_addr) {
        peer_health.update_last_seen();
        debug!("Updated last seen for peer: {}", peer_addr);
    } else {
        debug!("Peer {} not found", peer_addr);
    }
}
