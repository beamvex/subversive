// Import required dependencies and types
use anyhow::Result;

use clap::Parser;
use std::{clone::Clone, collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::{self, fmt::format::FmtSpan};

// Re-export modules
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

use types::args::Args;
use types::config::Config;
use types::health::PeerHealth;
use types::message::HeartbeatMessage;
use types::state::AppState;

/// Setup tracing subscriber
fn setup_tracing() {
    // Initialize the tracing subscriber with formatting options
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .with_file(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE)
        .init();
}

/// Load configuration from file or default if not specified
fn load_config() -> Config {
    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let config = if let Some(config_path) = &args.config {
        info!("Loading configuration from {}", config_path);
        Config::from_file(config_path).unwrap_or_else(|e| {
            error!("Failed to load config file: {}", e);
            Config::default()
        })
    } else {
        Config::default()
    };

    // Merge config with command line arguments
    let config = config.merge_with_args(&args);

    config
}

async fn config_ddns(config: &Config) -> Result<(), anyhow::Error> {
    // Start Dynamic DNS updaters if configured
    let client = reqwest::Client::new();

    // Configure No-IP if all required settings are present
    if let (Some(hostname), Some(username), Some(password)) = (
        config.noip_hostname.clone(),
        config.noip_username.clone(),
        config.noip_password.clone(),
    ) {
        info!("Starting No-IP DNS updater for hostname: {}", hostname);
        return ddns::start_ddns_updater(
            ddns::DdnsProvider::NoIp {
                hostname,
                username,
                password,
            },
            client.clone(),
        )
        .await;
    }

    // Configure OpenDNS if all required settings are present
    if let (Some(hostname), Some(username), Some(password), Some(network)) = (
        config.opendns_hostname.clone(),
        config.opendns_username.clone(),
        config.opendns_password.clone(),
        config.opendns_network.clone(),
    ) {
        info!("Starting OpenDNS updater for hostname: {}", hostname);
        return ddns::start_ddns_updater(
            ddns::DdnsProvider::OpenDns {
                hostname,
                username,
                password,
                network,
            },
            client.clone(),
        )
        .await;
    }

    Ok(())
}

/// Main entry point of the application
#[tokio::main]
pub async fn main() -> Result<()> {
    setup_tracing();

    let config = load_config();

    // Get port and database from config
    let port = config.get_port();
    let database = config.get_database();

    info!("Using port: {}", port);
    info!("Using database: {}", database);

    let _ = config_ddns(&config).await;

    // Get external IP and log the full endpoint address
    let external_ip = network::get_external_ip().await?;
    let own_address = format!("https://{}:{}", external_ip, port);
    info!("Server listening on internet endpoint: {}", own_address);

    // Set up UPnP port mapping
    let (actual_port, gateways) = upnp::setup_upnp(port).await?;
    info!("Using port {}", actual_port);

    // After UPnP setup
    let own_address = format!("https://{}:{}", external_ip, actual_port);
    info!("Own address: {}", own_address);

    // Create shutdown state
    let shutdown_state = shutdown::ShutdownState::new(actual_port, gateways);

    // Initialize database
    let db: Arc<DbContext> = Arc::new(DbContext::new(&database).unwrap());

    // Initialize shared application state
    let app_state = Arc::new(AppState {
        peers: Arc::new(Mutex::new(HashMap::<String, PeerHealth>::new())),
        db: db.clone(),
        own_address: own_address.clone(),
        shutdown: Arc::new(shutdown_state.clone()),
        config: config.clone(),
    });
    info!("Starting up");

    // Connect to initial peer if specified
    if let Some(ref peer_addr) = config.peer.clone() {
        peer::connect_to_initial_peer(
            peer_addr.clone(),
            actual_port,
            app_state.peers.clone(),
            external_ip,
        )
        .await?;
    }

    // Start peer health checker
    info!("Starting peer health checker");
    health::start_health_checker(app_state.clone()).await;

    // Start survival mode if enabled
    if config.survival_mode.unwrap_or(false) {
        info!("Starting survival mode");
        survival::start_survival_mode(app_state.clone()).await;
    }

    // Start the HTTP server
    info!("Starting HTTP server");
    let server_handle = tokio::spawn(server::run_http_server(
        actual_port,
        app_state.clone(),
        config.name.clone().unwrap_or("p2pserver".to_string()),
    ));

    Ok(shutdown_state.wait_shutdown(server_handle).await?)
}
