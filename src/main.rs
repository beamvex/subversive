// Import required dependencies and types
use anyhow::Result;
use clap::Parser;
use rand::Rng;
use std::{
    collections::HashMap,
    net::Ipv4Addr,
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
use types::message::{HeartbeatMessage, Message};
use types::peer::PeerInfo;
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

    // Generate random port between 10000-65535 if not specified
    let port = args.port.unwrap_or_else(|| {
        let mut rng = rand::thread_rng();
        rng.gen_range(10000..=65535)
    });

    // Ensure database name has .db extension
    let database = if !args.database.ends_with(".db") {
        format!("{}.db", args.database)
    } else {
        args.database
    };

    info!("Using port: {}", port);
    info!("Using database: {}", database);

    // Create a channel for message passing
    let (tx, rx) = broadcast::channel(32);

    // Initialize database
    let db = Arc::new(DbContext::new(&database)?);

    // Initialize shared application state
    let app_state = Arc::new(AppState {
        peers: Arc::new(Mutex::new(HashMap::new())),
        tx: tx.clone(),
        db: db.clone(),
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

    // Start message processor
    processor::start_message_processor(rx, db.clone()).await;

    // Set up cleanup on Ctrl+C
    shutdown::handle_shutdown(actual_port, gateways.clone()).await;

    // Connect to initial peer if specified
    if let Some(peer_addr) = args.peer {
        let external_ip = network::get_external_ip().await?;
        peer::connect_to_initial_peer(
            peer_addr,
            actual_port,
            app_state.peers.clone(),
            external_ip,
        ).await?;
    }

    // Start peer health checker
    health::start_health_checker(app_state.clone()).await;

    // Start the HTTP server
    if let Err(e) = server::run_http_server(actual_port, app_state.clone(), args.name).await {
        error!("Failed to start HTTP server: {}", e);
        return Err(e);
    }

    Ok(())
}
