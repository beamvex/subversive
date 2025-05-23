use std::collections::HashMap;
use std::sync::Arc;
use subversive_database::context::DbContext;

use crate::config::Config;
use crate::safe_map::SafeMap;
use subversive_network::health::PeerHealth;

/// Shared application state
pub struct AppState {
    /// Map of peer addresses to their health status
    pub peers: SafeMap<String, PeerHealth>,
    /// Database context
    pub db: Arc<DbContext>,
    /// Our own address that peers can use to connect to us
    pub own_address: String,

    /// Configuration
    pub config: Config,
    /// Actual port number
    pub actual_port: u16,
}
