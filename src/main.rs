// Import required dependencies and types
use anyhow::Result;

use std::{clone::Clone, collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;
use tracing_subscriber::{self, fmt::format::FmtSpan};

use crate::types::config::Config;
use crate::types::health::PeerHealth;
use crate::types::state::AppState;

// Module declarations
#[allow(unused)]
mod db;
mod network;
mod server;
mod shutdown;
mod survival;
mod types;

use db::DbContext;

/// Update the tracing level dynamically
pub fn update_tracing(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    // Reset the tracing subscriber with the new level
    let _ = tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .with_span_events(FmtSpan::CLOSE)
            .with_max_level(level)
            .finish(),
    );

    info!("Log level updated to: {}", log_level);
}

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
        shutdown: Arc::new(shutdown::ShutdownState::new(
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
