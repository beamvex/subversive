pub mod network;
pub mod peer;
pub mod startup;

pub use network::*;
pub use peer::*;
pub use startup::*;

// Re-export all trace types
pub use peer::{
    BuildHttpClient, PeerAddOwn, PeerAddRequest, PeerAlreadyConnected, PeerConnect,
    PeerConnectError, PeerConnected, PeerKnownCount, PeerLastSeen, PeerNotFound, PeerRemoved,
    PeerResponse,
};

pub trait TraceId: std::fmt::Debug {
    fn id(&self) -> u64;
    fn name(&self) -> &'static str;
    fn message(&self) -> String;
    fn process(&self) -> String;
}
