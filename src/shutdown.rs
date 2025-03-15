use crate::network;
use anyhow::Ok;
use std::sync::Arc;
use tracing::{error, info};

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
        if let Err(e) = network::cleanup_upnp(self.port, self.gateways.to_vec()).await {
            error!("Failed to clean up UPnP mappings: {}", e);
        }
        #[cfg(not(test))]
        std::process::exit(0);
    }

    pub async fn wait_shutdown(
        &self,
        server_handle: tokio::task::JoinHandle<Result<(), anyhow::Error>>,
    ) -> Result<(), anyhow::Error> {
        #[cfg(test)]
        let ctrl_c = async {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok(())
        };

        #[cfg(not(test))]
        let ctrl_c = tokio::signal::ctrl_c();

        // Wait for server or Ctrl+C
        tokio::select! {
            _ = ctrl_c => {
                info!("Received Ctrl+C, shutting down...");
            }
            result = server_handle => {
                if let Err(e) = result {
                    error!("Server error: {}", e);
                }
            }
        }

        self.shutdown().await;

        info!("Shutdown complete");
        Ok(())
    }
}
