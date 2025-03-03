use std::fs;
use crate::{api, tls, types::state::AppState};
use axum::{
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
            span.record("status", tracing::field::display(response.status()));
            span.record("latency", tracing::field::display(latency.as_secs_f64()));
        });

    // Build router with all routes
    let app = Router::new()
        .route("/peers", get(api::peers::list_peers))
        .route("/peer", post(api::peers::add_peer))
        .route("/messages", get(api::messages::get_recent_messages))
        .route("/message", post(api::messages::send_message))
        .route("/receive", post(api::messages::receive_message))
        .route("/heartbeat", post(api::health::heartbeat))
        .fallback_service(static_files_service)
        .layer(trace_layer)
        .layer(cors)
        .with_state(app_state);

    // Set up TLS
    let certs_dir = Path::new("certs");
    if !certs_dir.exists() {
        fs::create_dir_all(certs_dir)?;
    }

    let cert_path = certs_dir.join("cert.pem");
    let key_path = certs_dir.join("key.pem");

    // Create self-signed certificate if it doesn't exist
    if !cert_path.exists() || !key_path.exists() {
        tls::create_self_signed_cert(&cert_path, &key_path)?;
    }

    // Load TLS configuration
    let tls_config = RustlsConfig::from_pem_file(cert_path, key_path).await?;

    // Get the bind address and start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Starting HTTPS server on {}", addr);

    // Start the server with TLS
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
