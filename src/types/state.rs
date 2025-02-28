use std::{collections::HashMap, sync::{Arc, Mutex}};
use tokio::sync::broadcast;

use crate::{db::DbContext, types::message::Message};

/// Shared application state
pub struct AppState {
    /// Map of peer addresses to their HTTP clients
    pub peers: Arc<Mutex<HashMap<String, reqwest::Client>>>,
    /// Channel for sending messages within the application
    pub tx: broadcast::Sender<(Message, String)>,
    /// Database context
    pub db: Arc<DbContext>,
}
