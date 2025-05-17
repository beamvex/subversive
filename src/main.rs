use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
#[cfg(feature = "poc")]
use tokio::task::JoinError;
use tracing::info;

use subversive::types::{args::Args, config::Config, state::AppState};
use subversive_database::context::DbContext;
#[cfg(feature = "poc")]
use subversive_network::peer::connect_to_peer;
use subversive_utils::logutils::update_tracing;

#[cfg(all(feature = "default", not(feature = "poc")))]
#[tokio::main]
async fn main() -> Result<()> {
    update_tracing("info");

    let args = Args::parse();
    let port = args.port.unwrap_or(8080);

    info!("Starting subversive node on port {}", port);

    let db = Arc::new(DbContext::new("subversive.db").await?);
    let config = Config::default_config();

    let app_state = Arc::new(AppState {
        peers: Default::default(),
        db,
        actual_port: port,
        config: config.clone(),
        own_address: format!("https://localhost:{}", port),
    });

    let server_handle = tokio::spawn(subversive_server::server::spawn_server(app_state.clone()));
    let _ = server_handle.await??;

    Ok(())
}

#[cfg(feature = "poc")]
async fn run_poc(
    name: &str,
    port: u16,
    initial_peer: Option<String>,
) -> Result<tokio::task::JoinHandle<Result<Result<(), anyhow::Error>, JoinError>>> {
    let mut config = Config::load().await;
    config.database = Some(format!("{}.db", name));
    config.port = Some(port);
    let db = Arc::new(
        DbContext::new(
            config
                .database
                .clone()
                .unwrap_or("subversive.db".to_string()),
        )
        .await?,
    );

    let app_state = Arc::new(AppState {
        peers: Default::default(),
        db,
        actual_port: port,
        config: config.clone(),
        own_address: format!("https://localhost:{}", port),
    });

    let server_handle = tokio::spawn(subversive_server::server::spawn_server(app_state.clone()));

    // Add initial peer to peer list if provided
    if let Some(initial_peer) = initial_peer {
        info!(
            "Adding initial peer to peer list: {} from {}",
            initial_peer, app_state.own_address
        );
        let _ =
            subversive_network::peer::add_peer(app_state.peers.clone(), initial_peer.clone()).await;

        // Spawn a background task to periodically connect to all peers
        let app_state_clone = app_state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                info!("Attempting to connect to all peers");

                // Get list of all peers
                let peers = app_state_clone.peers.lock().await;
                let peer_addresses: Vec<String> = peers.keys().cloned().collect();
                drop(peers); // Release the lock

                // Try to connect to each peer
                for peer_addr in peer_addresses {
                    let _ = connect_to_peer(
                        app_state_clone.peers.clone(),
                        Some(peer_addr),
                        app_state_clone.own_address.clone(),
                        app_state_clone.actual_port,
                    )
                    .await;
                }
            }
        });
    }

    Ok(server_handle)
}

#[cfg(feature = "poc")]
#[tokio::main]
async fn main() -> Result<()> {
    update_tracing("info");
    subversive_utils::tui_utils::banner();

    info!("Starting subversive poc going to run multiple peers at once to test the network");

    let mut handles = vec![];
    for i in 8080..8090 {
        handles.push(
            run_poc(
                &format!("peer_{}", i),
                i,
                if i == 8080 {
                    None
                } else {
                    Some(format!("https://localhost:{}", 8080))
                },
            )
            .await,
        );
    }

    /* */

    info!("Press Ctrl+C to exit");

    tokio::signal::ctrl_c().await?;

    Ok(())
}
