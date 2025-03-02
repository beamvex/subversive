use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

use crate::{db::DbContext, types::{message::Message, health::PeerHealth}};

/// Shared application state
pub struct AppState {
    /// Map of peer addresses to their health status
    pub peers: Arc<Mutex<HashMap<String, PeerHealth>>>,
    /// Channel for sending messages within the application
    pub tx: broadcast::Sender<(Message, String)>,
    /// Database context
    pub db: Arc<DbContext>,
    /// Our own address that peers can use to connect to us
    pub own_address: String,
}
