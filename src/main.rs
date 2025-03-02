// Import required dependencies and types
use anyhow::Result;
use clap::Parser;
use rand::Rng;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tracing::{error, info};
use tracing_subscriber::{self, fmt::format::FmtSpan};

// Re-export modules
pub mod api;
pub mod db;
pub mod health;
pub mod network;
pub mod peer;
pub mod processor;
pub mod server;
pub mod shutdown;
pub mod tls;
pub mod types;
pub mod upnp;

use db::DbContext;

use types::args::Args;
use types::config::Config;
use types::message::HeartbeatMessage;
use types::state::AppState;

/// Main entry point of the application
#[tokio::main]
pub async fn main() -> Result<()> {
    // Initialize the tracing subscriber with formatting options
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .with_file(false)
        .with_line_number(false)
        .with_span_events(FmtSpan::CLOSE)
        .init();

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

    // Generate random port between 10000-65535 if not specified
    let port = config.port.unwrap_or_else(|| {
        let mut rng = rand::thread_rng();
        rng.gen_range(10000..=65535)
    });

    // Get database name from config
    let database = config.database.unwrap_or_else(|| "p2p_network.db".to_string());

    info!("Using port: {}", port);
    info!("Using database: {}", database);

    // Create a channel for message passing
    let (tx, rx) = broadcast::channel(32);

    // Initialize database
    let db = Arc::new(DbContext::new(&database)?);

    // Get external IP and log the full endpoint address
    let external_ip = network::get_external_ip().await?;
    let own_address = format!("https://{}:{}", external_ip, port);
    info!("Server listening on internet endpoint: {}", own_address);

    // Initialize shared application state
    let app_state = Arc::new(AppState {
        peers: Arc::new(Mutex::new(HashMap::<String, health::PeerHealth>::new())),
        tx: tx.clone(),
        db: db.clone(),
        own_address: own_address.clone(),
    });
    info!("Starting up");

    // Set up UPnP port mapping
    let (actual_port, gateways) = upnp::setup_upnp(port).await?;
    info!("Using port {}", actual_port);

    // After UPnP setup
    if crate::upnp::is_wsl() {
        info!("Manual port forwarding required:");
        info!("1. On Windows host, run PowerShell as Admin");
        info!("2. Execute: netsh interface portproxy add v4tov4 listenport={} listenaddress=0.0.0.0 connectport={} connectaddress=127.0.0.1", actual_port, actual_port);
        info!("3. Allow the port in Windows Defender Firewall:");
        info!("   New-NetFirewallRule -DisplayName 'P2P Port' -Direction Inbound -Action Allow -Protocol TCP -LocalPort {}", actual_port);
    }

    // Connect to initial peer if specified
    if let Some(peer_addr) = config.peer {
        peer::connect_to_initial_peer(peer_addr, actual_port, app_state.peers.clone(), external_ip)
            .await?;
    }

    // Start message processor
    processor::start_message_processor(rx, db.clone()).await;

    // Start peer health checker
    health::start_health_checker(app_state.clone()).await;

    // Start the HTTP server
    if let Err(e) = server::run_http_server(actual_port, app_state.clone(), config.name.unwrap_or_else(|| "p2p_network".to_string())).await {
        error!("Failed to start HTTP server: {}", e);
        return Err(e);
    }

    Ok(())
}
