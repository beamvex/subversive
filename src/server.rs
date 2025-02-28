use crate::{AppState, ChatMessage, HeartbeatMessage, Message, PeerInfo};
use axum::{
    extract::Json,
    extract::State,
    http::Request,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use axum_server::tls_rustls::RustlsConfig;
use chrono::Utc;
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
use tracing::{error, info, Level, Span};

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
            span.record("status", response.status().as_u16());
            span.record("latency", latency.as_secs_f64());
        });

    // Create router with routes
    let app = Router::new()
        .route("/peers", get(list_peers))
        .route("/send", post(send_message))
        .route("/receive", post(receive_message))
        .route("/peer", post(add_peer))
        .route("/heartbeat", post(heartbeat))
        .route("/recent_messages", get(get_recent_messages))
        .nest_service("/", static_files_service)
        .layer(cors)
        .layer(trace_layer)
        .with_state(app_state);

    // Set up TLS
    let cert_path = Path::new("cert.pem");
    let key_path = Path::new("key.pem");

    // Create self-signed certificate if it doesn't exist
    if !cert_path.exists() || !key_path.exists() {
        crate::tls::create_self_signed_cert(cert_path, key_path)?;
    }

    // Load TLS configuration
    let tls_config = RustlsConfig::from_pem_file(cert_path, key_path).await?;

    // Create TLS acceptor
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Listening on https://{}", addr);

    // Accept connections
    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// List all connected peers
async fn list_peers(State(state): State<Arc<AppState>>) -> Response {
    let one_hour_ago = Utc::now().timestamp() - 3600;
    match state.db.get_active_peers(one_hour_ago) {
        Ok(active_peers) => {
            let peer_list = active_peers
                .into_iter()
                .map(|peer| PeerInfo {
                    address: peer.address,
                })
                .collect::<Vec<_>>();
            Json(peer_list).into_response()
        }
        Err(e) => {
            error!("Failed to get active peers: {}", e);
            Json(Vec::<PeerInfo>::new()).into_response()
        }
    }
}

/// Get recent messages
async fn get_recent_messages(State(state): State<Arc<AppState>>) -> Response {
    match state.db.get_recent_messages(100) {
        Ok(messages) => Json(messages).into_response(),
        Err(e) => {
            error!("Failed to get recent messages: {}", e);
            Json(Vec::<ChatMessage>::new()).into_response()
        }
    }
}

/// Send a message to all peers
#[debug_handler]
async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<ChatMessage>,
) -> Response {
    // Save message to database
    let timestamp = Utc::now().timestamp();
    if let Err(e) = state.db.save_message(&message.content, "local", timestamp) {
        error!("Failed to save message to database: {}", e);
        return Json("Failed to save message").into_response();
    }

    // Broadcast message to all peers
    let msg = Message::Chat {
        content: message.content,
    };

    if let Err(e) = state.tx.send((msg.clone(), "local".to_string())) {
        error!("Failed to process message locally: {}", e);
        return Json("Failed to process message locally").into_response();
    }

    if let Err(e) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        error!("Failed to broadcast message to peers: {}", e);
        return Json("Failed to broadcast message").into_response();
    }

    Json("Message sent").into_response()
}

/// Receive a message from a peer
async fn receive_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Response {
    if let Err(e) = state.tx.send((message, "remote".to_string())) {
        error!("Failed to process received message: {}", e);
        return Json("Failed to process message").into_response();
    }
    Json("Message received").into_response()
}

/// Add a new peer to the network
async fn add_peer(State(state): State<Arc<AppState>>, Json(peer): Json<PeerInfo>) -> Response {
    if peer.address.is_empty() {
        error!("Attempted to add peer with empty address");
        return Json("Peer address cannot be empty").into_response();
    }

    // Ensure we're using HTTPS
    let peer_address = if !peer.address.starts_with("https://") {
        peer.address.replace("http://", "https://")
    } else {
        peer.address
    };

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to create HTTP client");
    state
        .peers
        .lock()
        .unwrap()
        .insert(peer_address.clone(), client);

    // Update peer's last seen timestamp in database
    if let Err(e) = state
        .db
        .update_peer_last_seen(&peer_address, Utc::now().timestamp())
    {
        error!("Failed to update peer in database: {}", e);
        return Json(format!("Failed to update peer in database: {}", e)).into_response();
    }

    let msg = Message::NewPeer {
        addr: peer_address.clone(),
    };

    if let Err(e) = state.tx.send((msg.clone(), "local".to_string())) {
        error!("Failed to process new peer locally: {}", e);
        return Json("Failed to process new peer locally").into_response();
    }

    if let Err(e) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        error!("Failed to broadcast new peer to network: {}", e);
        return Json("Failed to broadcast new peer to network").into_response();
    }

    Json("Peer added").into_response()
}

/// Handle heartbeat from a peer
async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Json(heartbeat): Json<HeartbeatMessage>,
) -> Response {
    let peer_addr = format!("http://localhost:{}", heartbeat.port);
    let peers = state.peers.lock().unwrap();

    if !peers.contains_key(&peer_addr) {
        error!("Heartbeat received from unknown peer: {}", peer_addr);
        return Json("Peer not found").into_response();
    }

    // Update peer's last seen timestamp
    if let Err(e) = state
        .db
        .update_peer_last_seen(&peer_addr, Utc::now().timestamp())
    {
        error!("Failed to update peer last seen timestamp: {}", e);
        return Json("Failed to update peer timestamp").into_response();
    }

    Json("Heartbeat acknowledged").into_response()
}
