use std::sync::Arc;
use tokio::sync::oneshot;

use crate::shutdown::ShutdownState;

#[tokio::test]
async fn test_new_shutdown_state() {
    let port = 12345;
    let gateways = Vec::new();
    let _shutdown = ShutdownState::new_test_mode(port, gateways);
    // Can't test private fields directly, but we can test functionality
}

#[tokio::test]
async fn test_wait_shutdown_server_error() {
    let port = 12345;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new_test_mode(port, gateways));
    
    // Create a server handle that will return an error
    let (tx, rx) = oneshot::channel();
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
    let shutdown = Arc::new(ShutdownState::new_test_mode(port, gateways));
    
    // Create a server handle that will never complete
    let (_tx, rx) = oneshot::channel::<()>();
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
    let shutdown = ShutdownState::new_test_mode(port, gateways);
    
    // In test mode, shutdown() should clean up UPnP but not exit
    shutdown.shutdown().await;
}
