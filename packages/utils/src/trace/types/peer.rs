use crate::tui::color::{green, red, yellow};
use super::TraceId;

#[derive(Debug, Clone)]
pub struct PeerConnect {
    pub addr: String,
}

#[derive(Debug, Clone)]
pub struct PeerResponse {
    pub addr: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct PeerError {
    pub addr: String,
    pub error: String,
}

impl TraceId for PeerConnect {
    fn id(&self) -> u64 { 0x0201 }
    fn name(&self) -> &'static str { "PeerConnect" }
    fn message(&self) -> String {
        format!("{} {}", green("Connected to peer at"), &self.addr)
    }
}

impl TraceId for PeerResponse {
    fn id(&self) -> u64 { 0x0202 }
    fn name(&self) -> &'static str { "PeerResponse" }
    fn message(&self) -> String {
        format!("{} {} - {}", yellow("Peer response from"), &self.addr, &self.status)
    }
}

impl TraceId for PeerError {
    fn id(&self) -> u64 { 0x0203 }
    fn name(&self) -> &'static str { "PeerError" }
    fn message(&self) -> String {
        format!("{} {} - {}", red("Peer error from"), &self.addr, &self.error)
    }
}
