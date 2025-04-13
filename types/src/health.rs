use reqwest::Client;
use std::time::SystemTime;

/// Health status of a peer
#[derive(Debug)]
pub struct PeerHealth {
    /// HTTP client for peer communication
    pub client: Client,
    /// Last time we received a message from this peer
    pub last_seen: i64,
}

impl PeerHealth {
    /// Create a new peer health tracker
    pub fn new(client: Client, _: String) -> Self {
        Self {
            client,
            last_seen: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        }
    }

    /// Update the last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
    }

    /// Get the last seen timestamp
    pub fn get_last_seen(&self) -> i64 {
        self.last_seen
    }

    /// Record a failure for this peer
    pub fn record_failure(&mut self) {
        // Set last_seen to 0 to mark as unhealthy
        self.last_seen = 0;
    }
}
