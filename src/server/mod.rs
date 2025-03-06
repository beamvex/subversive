pub mod api;
pub mod config;
pub mod tls;

use crate::types::state::AppState;
use axum::Router;
use std::{net::SocketAddr, sync::Arc};
use tokio::task::JoinHandle;
use tracing::info;

/// Start the HTTP server in a new task
///
/// # Arguments
/// * `app_state` - Shared application state
///
/// # Returns
/// A JoinHandle for the server task
pub fn spawn_server(app_state: Arc<AppState>) -> JoinHandle<anyhow::Result<()>> {
    info!("Starting HTTP server");
    tokio::spawn(run_http_server(app_state))
}

/// Run the HTTP server
///
/// # Arguments
/// * `app_state` - Shared application state
pub async fn run_http_server(app_state: Arc<AppState>) -> anyhow::Result<()> {
    let components = config::ServerComponents::initialize();
    let port = app_state.actual_port;

    // Build router with routes and middleware
    let router = Router::new().merge(api::register_routes(Router::new()));
    let app = components.configure_router(router).with_state(app_state);

    // Set up TLS config
    let config = tls::configure_tls().await?;

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on {}", addr);

    // Start the server with TLS
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
