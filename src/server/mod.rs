pub mod api;
pub mod tls;

use crate::types::state::AppState;
use axum::{
    http::Request,
    Router,
};
use axum_server::bind_rustls;
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
};
use tokio::task::JoinHandle;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::{info, Level};

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
    // Set up CORS
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    // Set up static file serving from public directory
    let public_dir = PathBuf::from("public");
    let static_files_service = ServeDir::new(public_dir);
    let name = app_state.config.get_name();
    let port = app_state.actual_port;

    // Set up logging middleware
    let trace_layer = TraceLayer::new_for_http().make_span_with(move |request: &Request<_>| {
        let method = request.method();
        let uri = request.uri();
        tracing::span!(
            Level::INFO,
            "http_request",
            method = %method,
            uri = %uri,
            name = %name,
        )
    });

    // Build router with routes and middleware
    let app = Router::new()
        .merge(api::register_routes(Router::new()))
        .layer(cors)
        .layer(trace_layer)
        .fallback_service(static_files_service)
        .with_state(app_state);

    // Set up TLS config
    let config = tls::configure_tls().await?;

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening on {}", addr);

    // Start the server
    bind_rustls(addr, config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}
