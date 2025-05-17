use std::thread;

use crate::tui::color::*;
use super::types::TraceId;

impl TraceId {
    pub fn message(&self) -> String {
        match self {
            // Startup messages
            TraceId::StartupInit { port } => self.format_startup_message(port),
            TraceId::StartupPoc => self.format_startup_message_simple(),
            TraceId::BuildHttpClient => green("Building HTTP client for peer connection"),
            TraceId::UserPrompt => green("Press Ctrl+C to exit"),
            
            // Network scan messages
            TraceId::NetworkScan => green("Attempting to connect to all peers"),
            
            // Peer connection messages
            TraceId::PeerConnect { peer } => self.format_peer_connect_message(peer),
            TraceId::PeerInit { peer, source } => self.format_peer_init_message(peer, source),
            TraceId::PeerConnected { peer } => self.format_peer_connected_message(peer),
            TraceId::PeerConnectError { peer, error } => self.format_peer_error_message(peer, error),
            
            // Peer management messages
            TraceId::PeerAlreadyConnected { peer } => self.format_peer_status_message("Already connected to peer", peer),
            TraceId::PeerRemoved { peer } => self.format_peer_status_message("Removed peer", peer),
            TraceId::PeerNotFound { peer } => self.format_peer_not_found_message(peer),
            TraceId::PeerLastSeen { peer } => self.format_peer_status_message("Updating last seen for peer", peer),
            
            // Peer discovery messages
            TraceId::PeerAddOwn { peer } => self.format_peer_status_message("Adding own peer to initial peer", peer),
            TraceId::PeerAddRequest { peer } => self.format_peer_status_message("Requesting to add peer", peer),
            TraceId::PeerResponse { peer } => self.format_peer_status_message("Response from initial peer", peer),
            TraceId::PeerKnownCount { peer, count } => self.format_peer_count_message(peer, count)
        }
    }

    fn format_startup_message(&self, port: &u16) -> String {
        format!("{} {}", 
            green("Starting subversive node on port"), 
            magenta(&port.to_string())
        )
    }

    fn format_startup_message_simple(&self) -> String {
        green("Starting subversive poc going to run multiple peers at once to test the network")
    }

    fn format_peer_connect_message(&self, peer: &str) -> String {
        format!("{} {}", 
            green("Connecting to initial peer:"), 
            magenta(peer)
        )
    }

    fn format_peer_init_message(&self, peer: &str, source: &str) -> String {
        format!("{} {} {} {}", 
            green("Adding initial peer to peer list:"), 
            magenta(peer),
            green("from"), 
            magenta(source)
        )
    }

    fn format_peer_connected_message(&self, peer: &str) -> String {
        format!("{} {}", 
            green("Successfully connected to peer:"), 
            magenta(peer)
        )
    }

    fn format_peer_error_message(&self, peer: &str, error: &str) -> String {
        format!("{} {} {} {}", 
            green("Failed to connect to peer:"), 
            magenta(peer),
            green("-"), 
            magenta(error)
        )
    }

    fn format_peer_status_message(&self, status: &str, peer: &str) -> String {
        format!("{} {}", 
            green(status), 
            magenta(peer)
        )
    }

    fn format_peer_not_found_message(&self, peer: &str) -> String {
        format!("{} {} {}", 
            green("Peer"), 
            magenta(peer), 
            green("not found")
        )
    }

    fn format_peer_count_message(&self, peer: &str, count: &usize) -> String {
        format!("{} {} {} {}", 
            green("Received"), 
            magenta(&count.to_string()),
            green("known peers from"), 
            magenta(peer)
        )
    }
    }

/// Format a message ID for logging with yellow color
pub fn format_msg_id(id: &TraceId) -> String {
    let name = match id {
        TraceId::StartupInit { .. } => "StartupInit",
        TraceId::StartupPoc => "StartupPoc",
        TraceId::PeerConnect { .. } => "PeerConnect",
        TraceId::PeerInit { .. } => "PeerInit",
        TraceId::NetworkScan => "NetworkScan",
        TraceId::UserPrompt => "UserPrompt",
        TraceId::BuildHttpClient => "BuildHttpClient",
        TraceId::PeerAlreadyConnected { .. } => "PeerAlreadyConnected",
        TraceId::PeerRemoved { .. } => "PeerRemoved",
        TraceId::PeerNotFound { .. } => "PeerNotFound",
        TraceId::PeerLastSeen { .. } => "PeerLastSeen",
        TraceId::PeerAddOwn { .. } => "PeerAddOwn",
        TraceId::PeerAddRequest { .. } => "PeerAddRequest",
        TraceId::PeerResponse { .. } => "PeerResponse",
        TraceId::PeerConnected { .. } => "PeerConnected",
        TraceId::PeerKnownCount { .. } => "PeerKnownCount",
        TraceId::PeerConnectError { .. } => "PeerConnectError",
    };
    format!("[{}]", yellow(name))
}

/// Get the current thread ID as a colored string (cyan)
pub fn get_thread_id() -> String {
    format!("[{}]", cyan(&format!("{:?}", thread::current().id())))
}
