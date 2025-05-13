use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::task::JoinError;
use tracing::info;

use subversive::types::{args::Args, config::Config, state::AppState};
use subversive_database::context::DbContext;
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

    let server_handle = tokio::spawn(subversive_server::server::spawn_server(app_state));
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
        handles.push(run_poc(&format!("peer_{}", i), i).await?);
    }

    info!("Press Ctrl+C to exit");

    tokio::signal::ctrl_c().await?;

    Ok(())
}
