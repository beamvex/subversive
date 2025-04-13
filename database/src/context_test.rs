#[cfg(test)]
mod tests {
    use crate::context::DbContext;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    #[tokio::test]
    async fn test_new_memory_db() {
        let db = DbContext::new_memory().await.unwrap();
        assert!(db
            .save_message("test", "source", get_current_timestamp())
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_message_operations() {
        let db = DbContext::new_memory().await.unwrap();
        let timestamp = get_current_timestamp();

        // Test saving message
        db.save_message("test message", "test source", timestamp)
            .await
            .unwrap();

        // Test retrieving messages
        let messages = db.get_messages_since(0).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "test message");
        assert_eq!(messages[0].source, "test source");
        assert_eq!(messages[0].timestamp, timestamp);
    }

    #[tokio::test]
    async fn test_peer_operations() {
        let db = DbContext::new_memory().await.unwrap();
        let timestamp = get_current_timestamp();

        // Test saving peer
        db.save_peer("127.0.0.1:8080", timestamp).await.unwrap();

        // Test retrieving active peers
        let peers = db.get_active_peers(0).await.unwrap();
        assert_eq!(peers.len(), 1);
        assert_eq!(peers[0].address, "127.0.0.1:8080");
        assert_eq!(peers[0].last_seen, timestamp);

        // Test updating peer last seen
        let new_timestamp = timestamp + 100;
        db.update_peer_last_seen("127.0.0.1:8080", new_timestamp)
            .await
            .unwrap();

        let updated_peers = db.get_active_peers(0).await.unwrap();
        assert_eq!(updated_peers[0].last_seen, new_timestamp);
    }

    #[tokio::test]
    async fn test_peer_filtering() {
        let db = DbContext::new_memory().await.unwrap();
        let timestamp = get_current_timestamp();

        // Add two peers with different timestamps
        db.save_peer("127.0.0.1:8080", timestamp - 100)
            .await
            .unwrap();
        db.save_peer("127.0.0.1:8081", timestamp).await.unwrap();

        // Test filtering by timestamp
        let active_peers = db.get_active_peers(timestamp - 50).await.unwrap();
        assert_eq!(active_peers.len(), 1);
        assert_eq!(active_peers[0].address, "127.0.0.1:8081");
    }

    #[tokio::test]
    async fn test_message_filtering() {
        let db = DbContext::new_memory().await.unwrap();
        let timestamp = get_current_timestamp();

        // Add two messages with different timestamps
        db.save_message("old message", "source", timestamp - 100)
            .await
            .unwrap();
        db.save_message("new message", "source", timestamp)
            .await
            .unwrap();

        // Test filtering by timestamp
        let recent_messages = db.get_messages_since(timestamp - 50).await.unwrap();
        assert_eq!(recent_messages.len(), 1);
        assert_eq!(recent_messages[0].content, "new message");
    }
}
