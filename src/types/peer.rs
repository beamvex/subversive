use serde::{Deserialize, Serialize};

/// Information about a peer in the network
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    /// Network address of the peer
    pub address: String,
    pub own_address: String,
}
