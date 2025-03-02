use crate::types::health::PeerHealth;
use crate::types::peer::PeerInfo;
use anyhow::Result;
use reqwest::Client;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tracing::{error, info};

/// Initialize connection to an initial peer
///
/// # Arguments
/// * `peer_addr` - Address of the peer to connect to
/// * `actual_port` - The port we're running on
/// * `peers` - Shared map of peer connections
/// * `external_ip` - Our external IP address
pub async fn connect_to_initial_peer(
    peer_addr: String,
    actual_port: u16,
    peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    external_ip: String,
) -> Result<()> {
    info!("Connecting to initial peer: {}", peer_addr);
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

    let my_addr = format!("https://{}:{}", external_ip, actual_port);

    // Acquire the lock to update peers
    {
        let mut peers = peers.lock().unwrap();
        peers.insert(peer_addr.clone(), PeerHealth::new(client.clone()));
    } // Lock is dropped here

    if let Err(e) = client
        .post(format!("{}/peer", peer_addr))
        .json(&PeerInfo { address: my_addr })
        .send()
        .await
    {
        error!("Failed to connect to initial peer {}: {}", peer_addr, e);
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
        let peers_guard = peers.lock().unwrap();
        peers_guard
            .iter()
            .filter(|(addr, _)| *addr != source)
            .map(|(addr, client)| (addr.clone(), client.client.clone()))
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
