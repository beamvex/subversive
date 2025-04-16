use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::{sync::Arc, time::SystemTime};

use crate::types::{message::Message, state::AppState};

/// Messages API module
pub struct Messages;

#[derive(Debug, Deserialize)]
pub struct GetMessagesQuery {
    pub since: Option<i64>,
}

impl Messages {
    /// Get recent messages
    pub async fn get_recent_messages(
        State(state): State<Arc<AppState>>,
        Query(query): Query<GetMessagesQuery>,
    ) -> impl IntoResponse {
        let since = query.since.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64
                - 3600 // Default to last hour
        });

        let messages = state
            .db
            .messages
            .get_messages_since(since)
            .await
            .unwrap_or_default();

        let chat_messages: Vec<Message> = messages
            .into_iter()
            .map(|m| Message::Chat { content: m.content })
            .collect();

        Json(chat_messages)
    }

    /// Send a new message
    pub async fn send_message(
        State(state): State<Arc<AppState>>,
        Json(message): Json<Message>,
    ) -> impl IntoResponse {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // Save message to database
        let content = match &message {
            Message::Chat { content } => content,
            _ => return Json(()),
        };

        if let Err(e) = state
            .db
            .messages
            .save_message(content, "local", timestamp)
            .await
        {
            tracing::error!("Failed to save message to database: {}", e);
            return Json(());
        }

        Json(())
    }
}

impl super::ApiModule for Messages {
    fn register_routes() -> Router<Arc<AppState>> {
        Router::new()
            .route("/messages", get(Self::get_recent_messages))
            .route("/message", post(Self::send_message))
    }
}
