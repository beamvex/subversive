use std::sync::Arc;
use tokio::signal;
use tracing::{error, info};

use crate::upnp;

/// Shared shutdown state to ensure cleanup happens only once
#[derive(Clone)]
pub struct ShutdownState {
    port: u16,
    gateways: Arc<Vec<igd::aio::Gateway>>,
}

impl ShutdownState {
    pub fn new(port: u16, gateways: Vec<igd::aio::Gateway>) -> Self {
        Self {
            port,
            gateways: Arc::new(gateways),
        }
    }

    /// Clean up UPnP mappings and exit
    pub async fn shutdown(&self) {
        info!("Cleaning up UPnP mappings...");
        if let Err(e) = upnp::cleanup_upnp(self.port, self.gateways.to_vec()).await {
            error!("Failed to clean up UPnP mappings: {}", e);
        }
        std::process::exit(0);
    }
}

/// Handle shutdown signals (Ctrl+C) and perform cleanup
///
/// # Arguments
/// * `state` - Shared shutdown state containing port and gateway information
pub async fn handle_shutdown(state: ShutdownState) {
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C");
                state.shutdown().await;
            }
            Err(err) => {
                error!("Error setting up Ctrl+C handler: {}", err);
            }
        }
    });
}
