use crate::tui::color::green;
use crate::TraceId;

pub fn format_peer_discovery_category(trace_id: &TraceId) -> Option<String> {
    match trace_id {
        TraceId::PeerAddOwn { peer } => Some(format_peer_status_message(
            "Adding own peer to initial peer",
            peer,
        )),
        TraceId::PeerAddRequest { peer } => {
            Some(format_peer_status_message("Requesting to add peer", peer))
        }
        TraceId::PeerResponse { peer } => Some(format_peer_status_message(
            "Response from initial peer",
            peer,
        )),
        TraceId::PeerKnownCount { peer, count } => Some(format_peer_count_message(peer, count)),
        _ => None,
    }
}

fn format_peer_count_message(peer: &str, count: &usize) -> String {
    format!("Peer {} knows {} peers", peer, count)
}

pub fn format_peer_status_message(status: &str, peer: &str) -> String {
    format!("{} {}", green(status), peer)
}
