use tokio::signal;
use tracing::{error, info};

use crate::upnp;

/// Handle shutdown signals (Ctrl+C) and perform cleanup
///
/// # Arguments
/// * `port` - Port number to clean up UPnP mappings for
/// * `gateways` - UPnP gateways to clean up
pub async fn handle_shutdown(port: u16, gateways: Vec<igd::aio::Gateway>) {
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Received Ctrl+C, cleaning up UPnP mappings...");
                if let Err(e) = upnp::cleanup_upnp(port, gateways).await {
                    error!("Failed to clean up UPnP mappings: {}", e);
                }
                std::process::exit(0);
            }
            Err(err) => {
                error!("Error setting up Ctrl+C handler: {}", err);
            }
        }
    });
}
