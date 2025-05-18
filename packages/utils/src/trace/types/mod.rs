pub mod startup;
pub mod network;
pub mod peer;

pub use startup::*;
pub use network::*;
pub use peer::*;

// Re-export all trace types
pub use peer::{BuildHttpClient, PeerConnect, PeerAlreadyConnected, PeerAddOwn, 
    PeerAddRequest, PeerResponse, PeerConnectError, PeerConnected, PeerKnownCount,
    PeerRemoved, PeerNotFound, PeerLastSeen};

pub trait TraceId: std::fmt::Debug {
    fn id(&self) -> u64;
    fn name(&self) -> &'static str;
    fn message(&self) -> String;
}
