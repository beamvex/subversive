use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use subversive_types::args::Args;
use subversive_utils::{trace::types::StartupInit, trace_info, TraceId};

#[cfg(feature = "poc")]
use subversive_utils::trace::types::{NetworkScan, PeerInit, StartupPoc, UserPrompt};
#[cfg(feature = "poc")]
use tokio::task::JoinError;

use subversive::types::{config::Config, state::AppState};
use subversive_database::context::DbContext;
#[cfg(feature = "poc")]
use subversive_network::peer::connect_to_peer;
use subversive_utils::logutils::update_tracing;

#[cfg(not(feature = "poc"))]
#[tokio::main]
async fn main() -> Result<()> {
    update_tracing("info");

    let args = Args::parse();
    let port = args.port.unwrap_or(8080);

    trace_info!(StartupInit { port: port });

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
        trace_info!(PeerInit {
            peer: initial_peer.clone(),
            source: app_state.own_address.clone()
        });
        let _ =
            subversive_network::peer::add_peer(app_state.peers.clone(), initial_peer.clone()).await;

        connect_to_peers(app_state);
    }

    Ok(server_handle)
}

#[cfg(feature = "poc")]
fn connect_to_peers(app_state: Arc<AppState>) {
    // Spawn a background task to periodically connect to all peers

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            trace_info!(NetworkScan {
                addr: app_state_clone.own_address.clone()
            });

            // Get list of all peers
            let peers = app_state_clone.peers.lock().await;
            let peer_addresses: Vec<String> = peers.keys().cloned().collect();
            drop(peers); // Release the lock

            // Try to connect to each peer
            for peer_addr in peer_addresses {
                // Skip connecting to self
                if peer_addr == app_state_clone.own_address {
                    continue;
                }
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

#[cfg(feature = "poc")]
#[tokio::main]
async fn main() -> Result<()> {
    update_tracing("info");
    subversive_utils::tui_utils::banner();

    trace_info!(StartupPoc);

    let mut handles = vec![];
    for i in 8080..8086 {
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

    trace_info!(UserPrompt);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
