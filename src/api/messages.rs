use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use chrono::Utc;
use std::sync::Arc;
use tracing::error;

use crate::{AppState, ChatMessage, Message};

/// Get recent messages
pub async fn get_recent_messages(State(state): State<Arc<AppState>>) -> Response {
    match state.db.get_recent_messages(100) {
        Ok(messages) => Json(messages).into_response(),
        Err(e) => {
            error!("Failed to get recent messages: {}", e);
            Json(Vec::<ChatMessage>::new()).into_response()
        }
    }
}

/// Send a message to all peers
pub async fn send_message(
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
pub async fn receive_message(
    State(state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> Response {
    if let Err(e) = state.tx.send((message, "remote".to_string())) {
        error!("Failed to process received message: {}", e);
        return Json("Failed to process message").into_response();
    }
    Json("Message received").into_response()
}
