use std::sync::Arc;
use tokio::sync::Mutex;
use crate::config::Config;
use crate::peer::PeerInfo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShutdownState {
    Running,
    ShuttingDown,
    Stopped,
}

#[derive(Debug)]
pub struct AppState {
    pub peers: Arc<Mutex<Vec<String>>>,
    pub config: Config,
    pub own_address: PeerInfo,
    pub shutdown: ShutdownState,
}
