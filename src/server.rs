use crate::{api, AppState};
use axum::{
    extract::State,
    http::Request,
    response::Response,
    routing::{get, post},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::{info, Level, Span};

/// Start the HTTP server
///
/// # Arguments
/// * `port` - Port to listen on
/// * `app_state` - Shared application state
/// * `name` - Custom name for logging
pub async fn run_http_server(
    port: u16,
    app_state: Arc<AppState>,
    name: String,
) -> anyhow::Result<()> {
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Set up static file serving from public directory
    let public_dir = PathBuf::from("public");
    let static_files_service = ServeDir::new(public_dir);

    // Set up logging middleware
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(move |request: &Request<_>| {
            tracing::span!(
                Level::INFO,
                "http_request",
                name = %name,
                method = %request.method(),
                uri = %request.uri(),
                status = tracing::field::Empty,
                latency = tracing::field::Empty,
            )
        })
        .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
            span.record("status", &tracing::field::display(response.status()));
            span.record("latency", &tracing::field::display(latency.as_secs_f64()));
        });

    // Build router with all routes
    let app = Router::new()
        .route("/peers", get(api::peers::list_peers).post(api::peers::add_peer))
        .route("/messages", get(api::messages::get_recent_messages))
        .route("/message", post(api::messages::send_message))
        .route("/receive", post(api::messages::receive_message))
        .route("/heartbeat", post(api::health::heartbeat))
        .fallback_service(static_files_service)
        .layer(trace_layer)
        .layer(cors)
        .with_state(app_state);

    // Get the bind address
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting HTTP server on {}", addr);

    // Start the server
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
