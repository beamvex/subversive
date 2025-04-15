use crate::network;
use crate::network::upnp::Gateway2;
use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// Represents the shutdown state of the server
pub struct ShutdownState {
    /// Port that the server is listening on
    port: u16,
    /// Gateway addresses
    gateways: Arc<Vec<Gateway2>>,
    /// Whether shutdown has been initiated
    shutdown_initiated: Arc<AtomicBool>,
}

impl ShutdownState {
    /// Create a new shutdown state
    pub fn new(port: u16, gateways: Vec<Gateway2>) -> Self {
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
    pub fn gateways(&self) -> Arc<Vec<Gateway2>> {
        self.gateways.clone()
    }

    /// Get the shutdown initiated state
    pub fn shutdown_initiated(&self) -> Arc<AtomicBool> {
        self.shutdown_initiated.clone()
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
        if let Err(e) = network::upnp::cleanup_upnp(self.port, (*self.gateways).clone()).await {
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
            gateways: Arc::clone(&self.gateways),
            shutdown_initiated: Arc::clone(&self.shutdown_initiated),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::shutdown::ShutdownState;

    #[tokio::test]
    async fn test_new_shutdown_state() {
        let port = 12345;
        let gateways = Vec::new();
        let _shutdown = ShutdownState::new(port, gateways);
        // Can't test private fields directly, but we can test functionality
    }

    #[tokio::test]
    async fn test_wait_shutdown_server_error() {
        let port = 12345;
        let gateways = Vec::new();
        let shutdown = Arc::new(ShutdownState::new(port, gateways));

        // Create a server handle that will return an error
        let (tx, rx) = tokio::sync::oneshot::channel();
        let server_handle = tokio::spawn(async move {
            rx.await.unwrap(); // Wait for signal
            Err::<(), anyhow::Error>(anyhow::anyhow!("Server error"))
        });

        // Spawn a task to trigger server error
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            tx.send(()).unwrap();
        });

        // Wait for shutdown - in test mode this won't exit the process
        let result = shutdown.wait_shutdown(server_handle).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wait_shutdown_ctrl_c() {
        let port = 12345;
        let gateways = Vec::new();
        let shutdown = Arc::new(ShutdownState::new(port, gateways));

        // Create a server handle that will never complete
        let (_tx, rx) = tokio::sync::oneshot::channel::<()>();
        let server_handle = tokio::spawn(async move {
            rx.await.unwrap(); // This will never complete
            Ok::<(), anyhow::Error>(())
        });

        // In test mode, ctrl_c will be simulated after 100ms
        let result = shutdown.wait_shutdown(server_handle).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_shutdown() {
        let port = 12345;
        let gateways = Vec::new();
        let shutdown = ShutdownState::new(port, gateways);

        // In test mode, shutdown() should clean up UPnP but not exit
        shutdown.shutdown().await;
    }
}
