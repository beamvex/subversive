use crate::network;
use anyhow::Result;
use igd::aio::Gateway;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// Represents the shutdown state of the server
pub struct ShutdownState {
    /// Port that the server is listening on
    port: u16,
    /// Gateway addresses
    gateways: Arc<Vec<Gateway>>,
    /// Whether shutdown has been initiated
    shutdown_initiated: Arc<AtomicBool>,
}

impl ShutdownState {
    /// Create a new shutdown state
    pub fn new(port: u16, gateways: Vec<Gateway>) -> Self {
        Self {
            port,
            gateways: Arc::new(gateways),
            shutdown_initiated: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Get the port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the gateways
    pub fn get_gateways(&self) -> Vec<Gateway> {
        (&self.gateways).to_vec()
    }

    /// Initiate shutdown
    pub fn initiate_shutdown(&self) {
        self.shutdown_initiated.store(true, Ordering::SeqCst);
    }

    /// Check if shutdown has been initiated
    pub fn is_shutdown_initiated(&self) -> bool {
        self.shutdown_initiated.load(Ordering::SeqCst)
    }

    /// Clean up UPnP mappings and exit
    pub async fn shutdown(&self) {
        info!("Cleaning up UPnP mappings...");
        if let Err(e) = network::cleanup_upnp(self.port, (&self.gateways).to_vec()).await {
            error!("Failed to clean up UPnP mappings: {}", e);
        }
        #[cfg(not(test))]
        std::process::exit(0);
    }

    /// Wait for shutdown to complete
    pub async fn wait_shutdown(
        &self,
        server_handle: JoinHandle<Result<(), anyhow::Error>>,
    ) -> Result<(), anyhow::Error> {
        #[cfg(test)]
        let ctrl_c = async {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok::<(), anyhow::Error>(())
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

impl Clone for ShutdownState {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            gateways: self.gateways.clone(),
            shutdown_initiated: self.shutdown_initiated.clone(),
        }
    }
}
