use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHealth {
    pub last_seen: SystemTime,
    pub consecutive_failures: u32,
}
