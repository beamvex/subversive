use axum::{
    body::to_bytes,
    extract::{Json, Query, State},
    response::IntoResponse,
};
use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
use subversive::{
    db::DbContext,
    server::api::messages::{GetMessagesQuery, Messages},
    shutdown::ShutdownState,
    types::{config::Config, message::Message, state::AppState},
};
use tokio::sync::Mutex;

async fn setup_test_state() -> Arc<AppState> {
    let config = Config::default_config();
    let port = 8080;
    let gateways = Vec::new();
    let shutdown = Arc::new(ShutdownState::new(port, gateways));

    Arc::new(AppState {
        config,
        own_address: "https://localhost:8080".to_string(),
        peers: Arc::new(Mutex::new(HashMap::new())),
        db: Arc::new(DbContext::new_memory().await.unwrap()),
        actual_port: port,
        shutdown,
    })
}

#[tokio::test(flavor = "multi_thread")]
async fn test_get_recent_messages_empty() {
    let state = setup_test_state().await;
    let query = GetMessagesQuery { since: None };
    let response = Messages::get_recent_messages(State(state.clone()), Query(query)).await.into_response();
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
    let response = Messages::get_recent_messages(State(state.clone()), Query(query)).await.into_response();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let messages: Vec<Message> = serde_json::from_slice(&body).unwrap();

    assert_eq!(messages.len(), 1);
    if let Message::Chat { content: msg_content } = &messages[0] {
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
    let response = Messages::get_recent_messages(State(state.clone()), Query(query)).await.into_response();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let messages: Vec<Message> = serde_json::from_slice(&body).unwrap();
    assert!(messages.is_empty());

    // Get messages since before the message was added
    let query = GetMessagesQuery {
        since: Some(timestamp - 1),
    };
    let response = Messages::get_recent_messages(State(state.clone()), Query(query)).await.into_response();
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

    let response = Messages::send_message(State(state.clone()), Json(message)).await.into_response();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let _: () = serde_json::from_slice(&body).unwrap();

    // Verify message was saved
    let timestamp = Utc::now().timestamp();
    let messages = state
        .db
        .messages
        .get_messages_since(timestamp - 3600)
        .await
        .unwrap();
    assert_eq!(messages.len(), 1);
}
