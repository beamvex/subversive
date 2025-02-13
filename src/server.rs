use crate::{AppState, ChatMessage, Message, PeerInfo, HeartbeatMessage};
use anyhow::Result as AnyhowResult;
use axum::{
    extract::{State, Json},
    routing::{get, post},
    Router,
    response::{IntoResponse, Response, Result},
    http::StatusCode,
};
use log::{error, info};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

type ApiResult<T> = Result<T, Response>;

pub async fn run_http_server(
    port: u16,
    app_state: Arc<AppState>,
) -> AnyhowResult<()> {
    // Set up CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/peers", get(list_peers))
        .route("/send", post(send_message))
        .route("/receive", post(receive_message))
        .route("/peer", post(add_peer))
        .route("/heartbeat", post(heartbeat))
        .layer(cors)
        .with_state(app_state);

    // Run it
    let addr = format!("0.0.0.0:{}", port).parse()?;
    info!("Starting HTTP server on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await?,
        app.into_make_service(),
    )
    .await?;

    Ok(())
}

pub async fn list_peers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>> {
    let peers = state.peers.lock().unwrap();
    let peers_list = peers.keys().cloned().collect::<Vec<String>>();
    Ok(Json(peers_list))
}

pub async fn send_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<ChatMessage>,
) -> Result<Json<&'static str>> {
    let msg = Message::Chat {
        content: message.content,
    };

    if let Err(e) = state
        .tx
        .send((msg.clone(), "local".to_string()))
    {
        error!("Error broadcasting message: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    if let Err(e) = crate::broadcast_to_peers(msg, "local", &state.peers).await {
        error!("Error broadcasting to peers: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    Ok(Json("Message sent"))
}

pub async fn receive_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Result<Json<&'static str>> {
    if let Err(e) = state.tx.send((message, "remote".to_string())) {
        error!("Error broadcasting received message: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }
    Ok(Json("Message received"))
}

pub async fn add_peer(
    State(state): State<Arc<AppState>>,
    Json(peer_info): Json<PeerInfo>,
) -> Result<Json<&'static str>> {
    let mut peers = state.peers.lock().unwrap();
    
    // Check if we already have this peer
    if peers.contains_key(&peer_info.address) {
        return Err(StatusCode::BAD_REQUEST.into_response());
    }

    // Create HTTP client for the peer
    let client = reqwest::Client::new();

    // Add peer to our list
    peers.insert(peer_info.address.clone(), client.clone());
    info!("Added new peer: {}", peer_info.address);

    // Notify existing peers about the new peer
    let new_peer_msg = Message::NewPeer {
        addr: peer_info.address.clone(),
    };

    drop(peers); // Release the lock before async operation
    if let Err(e) = crate::broadcast_to_peers(new_peer_msg, &peer_info.address, &state.peers).await {
        error!("Error broadcasting new peer: {}", e);
    }

    // Send our peer list to the new peer
    let peers = state.peers.lock().unwrap();
    let our_peers: Vec<String> = peers.keys().cloned().collect();
    drop(peers); // Release the lock before async operation

    for peer_addr in our_peers {
        if peer_addr == peer_info.address {
            continue;
        }

        let msg = PeerInfo {
            address: peer_addr,
        };

        if let Err(e) = client
            .post(format!("{}/peer", peer_info.address))
            .json(&msg)
            .send()
            .await
        {
            error!("Error sending peer info to new peer: {}", e);
        }
    }

    Ok(Json("Peer added"))
}

pub async fn heartbeat(
    State(state): State<Arc<AppState>>,
    Json(heartbeat): Json<HeartbeatMessage>,
) -> Result<Json<&'static str>> {
    let peers = state.peers.lock().unwrap();
    
    // Check if we have this peer
    if !peers.contains_key(&heartbeat.port.to_string()) {
        return Err(StatusCode::NOT_FOUND.into_response());
    }

    Ok(Json("Heartbeat received"))
}
