use super::common::format_peer_status_message;
use crate::TraceId;

pub fn format_peer_management_category(trace_id: &TraceId) -> Option<String> {
    match trace_id {
        TraceId::PeerAlreadyConnected { peer } => Some(format_peer_status_message(
            "Already connected to peer",
            peer,
        )),
        TraceId::PeerRemoved { peer } => Some(format_peer_status_message("Removed peer", peer)),
        TraceId::PeerNotFound { peer } => Some(format_peer_not_found_message(peer)),
        TraceId::PeerLastSeen { peer } => Some(format_peer_status_message(
            "Updating last seen for peer",
            peer,
        )),
        _ => None,
    }
}

fn format_peer_not_found_message(peer: &str) -> String {
    format!("Could not find peer: {}", peer)
}
