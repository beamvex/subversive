// Import required dependencies and types
use anyhow::Result;
use std::sync::Arc;
use tracing::info;

use subversive::{
    db::DbContext,
    server,
    shutdown::ShutdownState,
    types::{args::Args, config::Config, state::AppState},
};

// Module declarations
#[allow(unused)]
mod db;
mod network;

mod shutdown;
mod survival;
mod types;

/// Main entry point of the application
#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = types::config::Config::load().await;
    let port = config.port.unwrap_or(8080);

    // Initialize database
    let db = Arc::new(DbContext::new("subversive.db").await?);

    // Create application state
    let app_state = Arc::new(AppState {
        config: config.clone(),
        own_address: format!("https://localhost:{}", port),
        peers: Default::default(),
        db,
        actual_port: port,
        shutdown: Arc::new(ShutdownState::new(
            port,
            Vec::new(), // No gateways for now
        )),
    });

    // Start server
    let server_handle = tokio::spawn(server::spawn_server(app_state.clone()));

    // Wait for shutdown signal
    tokio::signal::ctrl_c().await?;
    info!("Shutting down...");

    // Wait for server to finish
    if let Err(e) = server_handle.await? {
        tracing::error!("Server error: {}", e);
        return Err(e.into());
    }

    Ok(())
}
