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
pub mod api;
pub mod db;
pub mod ddns;
pub mod health;
pub mod network;
pub mod peer;
pub mod server;
pub mod shutdown;
pub mod survival;
pub mod tls;
pub mod types;
pub mod upnp;

use db::DbContext;

/// Setup tracing subscriber
fn setup_tracing() {
    // Initialize the tracing subscriber with formatting options
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE)
        .init();
}

/// Initialize the application
///
/// Sets up logging, loads config, initializes network and creates application state
async fn initialize() -> Result<(Arc<AppState>, Arc<shutdown::ShutdownState>)> {
    setup_tracing();

    let config = Config::load();

    // Get port and database from config
    let port = config.get_port();
    let database = config.get_database();

    info!("Using port: {}", port);
    info!("Using database: {}", database);

    ddns::config_ddns(&config).await;

    // Set up network connectivity
    let (actual_port, gateways, own_address) = network::setup_network(port).await?;

    // Create shutdown state
    let shutdown_state = Arc::new(shutdown::ShutdownState::new(actual_port, gateways));

    // Initialize database
    let db: Arc<DbContext> = Arc::new(DbContext::new(&database).unwrap());

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
    let (app_state, shutdown_state) = initialize().await?;

    info!("Starting up");

    // Connect to initial peer if specified
    if app_state.config.peer.is_some() {
        peer::connect_to_initial_peer(app_state.clone()).await?;
    }

    // Start peer health checker
    info!("Starting peer health checker");
    health::start_health_checker(app_state.clone()).await;

    // Start survival mode if enabled
    if app_state.config.survival_mode.unwrap_or(false) {
        info!("Starting survival mode");
        survival::start_survival_mode(app_state.clone()).await;
    }

    // Start the HTTP server
    let server_handle = server::spawn_server(app_state.clone());

    Ok(shutdown_state.wait_shutdown(server_handle).await?)
}
