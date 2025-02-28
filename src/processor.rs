use std::sync::Arc;
use tokio::sync::broadcast::Receiver;
use tracing::{error, info};

use crate::{db::DbContext, Message};

/// Start the message processing thread
///
/// # Arguments
/// * `rx` - Broadcast receiver for messages
/// * `db` - Database context for saving messages and peers
pub async fn start_message_processor(mut rx: Receiver<(Message, String)>, db: Arc<DbContext>) {
    tokio::spawn(async move {
        while let Ok((message, source)) = rx.recv().await {
            match message {
                Message::Chat { content } => {
                    info!("Received chat message from {}: {}", source, content);
                    if let Err(e) =
                        db.save_message(&content, &source, chrono::Utc::now().timestamp())
                    {
                        error!("Failed to save message: {}", e);
                    }
                }
                Message::NewPeer { addr } => {
                    info!("Received new peer from {}: {}", source, addr);
                    if let Err(e) = db.save_peer(&addr, chrono::Utc::now().timestamp()) {
                        error!("Failed to save peer: {}", e);
                    }
                }
            }
        }
    });
}
