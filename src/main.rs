use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tracing::info;

use subversive::{
    db::DbContext,
    server,
    shutdown::ShutdownState,
    types::{args::Args, config::Config, state::AppState},
};

#[tokio::main]
async fn main() -> Result<()> {
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
        shutdown: Arc::new(ShutdownState::new(
            port,
            Vec::new(), // No gateways for now
        )),
    });

    let server_handle = tokio::spawn(server::spawn_server(app_state.clone()));
    let _ = server_handle.await??;

    Ok(())
}
