use crate::TraceId;

pub mod common;
pub mod network;
pub mod peer_connection;
pub mod peer_discovery;
pub mod peer_management;
pub mod startup;

pub use common::{format_msg_id, format_peer_status_message, get_thread_id};
pub use network::format_network_scan_category;
pub use peer_connection::format_peer_connection_category;
pub use peer_discovery::format_peer_discovery_category;
pub use peer_management::format_peer_management_category;
pub use startup::format_startup_category;

impl TraceId {
    pub fn message(&self) -> String {
        if let Some(msg) = format_startup_category(self) {
            return msg;
        }
        if let Some(msg) = format_network_scan_category(self) {
            return msg;
        }
        if let Some(msg) = format_peer_discovery_category(self) {
            return msg;
        }
        if let Some(msg) = format_peer_connection_category(self) {
            return msg;
        }
        if let Some(msg) = format_peer_management_category(self) {
            return msg;
        }
        format!("Unknown message type: {:?}", self)
    }
}
