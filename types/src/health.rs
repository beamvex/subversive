use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHealth {
    pub last_seen: SystemTime,
    pub consecutive_failures: u32,
    #[serde(skip)]
    pub client: Client,
}

impl PeerHealth {
    pub fn new(client: Client, base_url: String) -> Self {
        Self {
            last_seen: SystemTime::now(),
            consecutive_failures: 0,
            client,
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }

    pub fn get_last_seen(&self) -> SystemTime {
        self.last_seen
    }
}
