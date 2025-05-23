pub mod logutils;
pub mod test_utils;
pub mod trace;
pub mod tui;
pub mod tui_utils;

// Re-export macros and types
pub use trace::macros::*;
pub use trace::types::TraceId;
pub use trace::types::{
    BuildHttpClient, PeerAddOwn, PeerAddRequest, PeerAlreadyConnected, PeerConnect,
    PeerConnectError, PeerConnected, PeerKnownCount, PeerLastSeen, PeerNotFound, PeerRemoved,
    PeerResponse,
};
