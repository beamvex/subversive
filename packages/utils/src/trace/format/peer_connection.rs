use crate::TraceId;

pub fn format_peer_connection_category(trace_id: &TraceId) -> Option<String> {
    match trace_id {
        TraceId::PeerConnect { peer } => Some(format_peer_connect_message(peer)),
        TraceId::PeerInit { peer, source } => Some(format_peer_init_message(peer, source)),
        TraceId::PeerConnected { peer } => Some(format_peer_connected_message(peer)),
        TraceId::PeerConnectError { peer, error } => Some(format_peer_error_message(peer, error)),
        _ => None,
    }
}

fn format_peer_connect_message(peer: &str) -> String {
    format!("Connecting to initial peer: {}", peer)
}

fn format_peer_init_message(peer: &str, source: &str) -> String {
    format!("Adding initial peer to peer list: {} from {}", peer, source)
}

fn format_peer_connected_message(peer: &str) -> String {
    format!("Connected to peer: {}", peer)
}

fn format_peer_error_message(peer: &str, error: &str) -> String {
    format!("Error connecting to peer {}: {}", peer, error)
}
