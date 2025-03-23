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
pub mod db;
pub mod ddns;

pub mod network;
pub mod server;
pub mod shutdown;
#[cfg(test)]
mod shutdown_test;
pub mod survival;
#[cfg(test)]
mod survival_test;
pub mod types;

use db::DbContext;

/// Setup tracing subscriber
fn setup_tracing(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => tracing::Level::TRACE,
        "debug" => tracing::Level::DEBUG,
        "info" => tracing::Level::INFO,
        "warn" => tracing::Level::WARN,
        "error" => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    };

    // Initialize the tracing subscriber with formatting options
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE)
        .with_max_level(level)
        .init();
}

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

/// Initialize the application
///
/// Sets up logging, loads config, initializes network and creates application state
async fn initialize() -> Result<(Arc<AppState>, Arc<shutdown::ShutdownState>)> {
    setup_tracing("info");
    let config = Config::load().await;
    update_tracing(&config.get_log_level());

    // Get port and database from config
    let port = config.get_port();
    let database = config.get_database();
    let hostname = config.get_hostname();

    info!("Using port: {}", port);
    info!("Using database: {}", database);
    info!("Using hostname: {}", hostname.unwrap_or_default());
    info!("Using log level: {}", config.get_log_level());

    ddns::config_ddns(&config).await;

    // Set up network connectivity
    let (actual_port, gateways, own_address) = network::setup_network(port, &config).await?;

    // Create shutdown state
    let shutdown_state = Arc::new(shutdown::ShutdownState::new(actual_port, gateways));

    // Initialize database
    let db: Arc<DbContext> = Arc::new(DbContext::new(&database).await?);

    // Initialize shared application state
    let app_state = Arc::new(AppState {
        peers: Arc::new(Mutex::new(HashMap::<String, PeerHealth>::new())),
        db: db.clone(),
        own_address: own_address.clone(),
        shutdown: shutdown_state.clone(),
        config: config.clone(),
        actual_port,
    });

    Ok((app_state, shutdown_state))
}

/// Main entry point of the application
#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load config
    let config = types::config::Config::load().await;

    // Initialize database
    let db = Arc::new(DbContext::new("subversive.db").await?);

    // Create application state
    let app_state = Arc::new(AppState {
        config,
        own_address: format!("https://localhost:{}", config.port.unwrap_or(8080)),
        peers: Default::default(),
        db,
        actual_port: config.port.unwrap_or(8080),
        shutdown: Arc::new(shutdown::ShutdownState::new(
            config.port.unwrap_or(8080),
            config.gateways.unwrap_or_default(),
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
