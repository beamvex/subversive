use axum::{
    extract::{Json, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::{sync::Arc, time::SystemTime};

use subversive_types::{message::Message, state::AppState};

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
            .get_messages(since)
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

#[cfg(test)]
mod tests {
    use axum::{
        body::to_bytes,
        extract::{Json, Query, State},
        response::IntoResponse,
    };
    use chrono::Utc;
    use std::{collections::HashMap, sync::Arc};
    use subversive_database::context::DbContext;
    use subversive_types::{config::Config, message::Message, state::AppState};
    use tokio::sync::Mutex;

    use crate::api::messages::{GetMessagesQuery, Messages};

    async fn setup_test_state() -> Arc<AppState> {
        let config = Config::default_config();
        let port = 8080;

        Arc::new(AppState {
            config,
            own_address: "https://localhost:8080".to_string(),
            peers: SafeMap::new(),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            actual_port: port,
        })
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_recent_messages_empty() {
        let state = setup_test_state().await;
        let query = GetMessagesQuery { since: None };
        let response = Messages::get_recent_messages(State(state.clone()), Query(query))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let messages: Vec<Message> = serde_json::from_slice(&body).unwrap();
        assert!(messages.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_recent_messages_with_message() {
        let state = setup_test_state().await;
        let content = "Test message";
        let source = "test";
        let timestamp = Utc::now().timestamp();

        // Add a test message
        state
            .db
            .messages
            .save_message(content, source, timestamp)
            .await
            .unwrap();

        let query = GetMessagesQuery { since: None };
        let response = Messages::get_recent_messages(State(state.clone()), Query(query))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let messages: Vec<Message> = serde_json::from_slice(&body).unwrap();

        assert_eq!(messages.len(), 1);
        if let Message::Chat {
            content: msg_content,
        } = &messages[0]
        {
            assert_eq!(msg_content, content);
        } else {
            panic!("Expected Chat message");
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_get_recent_messages_with_timestamp() {
        let state = setup_test_state().await;
        let content = "Test message";
        let source = "test";
        let timestamp = Utc::now().timestamp();

        // Add a test message
        state
            .db
            .messages
            .save_message(content, source, timestamp)
            .await
            .unwrap();

        // Get messages since after the message was added
        let query = GetMessagesQuery {
            since: Some(timestamp + 1),
        };
        let response = Messages::get_recent_messages(State(state.clone()), Query(query))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let messages: Vec<Message> = serde_json::from_slice(&body).unwrap();
        assert!(messages.is_empty());

        // Get messages since before the message was added
        let query = GetMessagesQuery {
            since: Some(timestamp - 1),
        };
        let response = Messages::get_recent_messages(State(state.clone()), Query(query))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let messages: Vec<Message> = serde_json::from_slice(&body).unwrap();
        assert_eq!(messages.len(), 1);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_send_message() {
        let state = setup_test_state().await;
        let message = Message::Chat {
            content: "Test message".to_string(),
        };

        let response = Messages::send_message(State(state.clone()), Json(message))
            .await
            .into_response();
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let _: () = serde_json::from_slice(&body).unwrap();

        // Verify message was saved
        let timestamp = Utc::now().timestamp();
        let messages = state
            .db
            .messages
            .get_messages(timestamp - 3600)
            .await
            .unwrap();
        assert_eq!(messages.len(), 1);
    }
}
