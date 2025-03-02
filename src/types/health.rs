use reqwest::Client;

/// Health status of a peer
#[derive(Debug)]
pub struct PeerHealth {
    /// HTTP client for the peer
    pub client: Client,
    /// Number of consecutive failed health checks
    pub failed_checks: u32,
}

impl PeerHealth {
    /// Create a new PeerHealth instance
    pub fn new(client: Client) -> Self {
        Self {
            client,
            failed_checks: 0,
        }
    }

    /// Record a failed health check
    pub fn record_failure(&mut self) -> u32 {
        self.failed_checks += 1;
        self.failed_checks
    }

    /// Reset failed health checks counter
    pub fn reset_failures(&mut self) {
        self.failed_checks = 0;
    }
}
