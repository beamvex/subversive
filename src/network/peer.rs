use crate::types::{health::PeerHealth, peer::PeerInfo, state::AppState};
use anyhow::Result;
use reqwest::Client;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{error, info};

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
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");

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
            peers.insert(
                peer_addr.clone(),
                PeerHealth {
                    address: peer_addr.clone(),
                    client: client.clone(),
                    failed_checks: 0,
                },
            );

            // Get and add the peer's known peers
            if let Ok(known_peers) = response.json::<Vec<PeerInfo>>().await {
                info!("Received {} known peers from {}", known_peers.len(), peer_addr);
                
                for known_peer in known_peers {
                    if known_peer.address != own_address.clone() && !peers.contains_key(&known_peer.address) {
                        let peer_client = Client::builder()
                            .danger_accept_invalid_certs(true)
                            .build()
                            .expect("Failed to create HTTP client");
                            
                        peers.insert(
                            known_peer.address.clone(),
                            PeerHealth {
                                address: known_peer.address,
                                client: peer_client,
                                failed_checks: 0,
                            },
                        );
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
