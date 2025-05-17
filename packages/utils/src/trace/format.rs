use std::thread;
use super::color;
use super::types::TraceId;

impl TraceId {
    pub fn message(&self) -> String {
        use color::*;
        match self {
            TraceId::StartupInit { port } => 
                format!("{} {}", green("Starting subversive node on port"), magenta(&port.to_string())),
            TraceId::StartupPoc => 
                green("Starting subversive poc going to run multiple peers at once to test the network"),
            TraceId::PeerConnect { peer } => 
                format!("{} {}", green("Connecting to initial peer:"), magenta(peer)),
            TraceId::PeerInit { peer, source } => 
                format!("{} {} {} {}", 
                    green("Adding initial peer to peer list:"), 
                    magenta(peer), 
                    green("from"), 
                    magenta(source)
                ),
            TraceId::NetworkScan => 
                green("Attempting to connect to all peers"),
            TraceId::UserPrompt => 
                green("Press Ctrl+C to exit"),
            TraceId::BuildHttpClient => 
                green("Building HTTP client for peer connection"),
            TraceId::PeerAlreadyConnected { peer } => 
                format!("{} {}", green("Already connected to peer:"), magenta(peer)),
            TraceId::PeerRemoved { peer } => 
                format!("{} {}", green("Removed peer:"), magenta(peer)),
            TraceId::PeerNotFound { peer } => 
                format!("{} {} {}", green("Peer"), magenta(peer), green("not found")),
            TraceId::PeerLastSeen { peer } => 
                format!("{} {}", green("Updating last seen for peer:"), magenta(peer)),
            TraceId::PeerAddOwn { peer } => 
                format!("{} {}", green("Adding own peer to initial peer:"), magenta(peer)),
            TraceId::PeerAddRequest { peer } => 
                format!("{} {}", green("Requesting to add peer:"), magenta(peer)),
            TraceId::PeerResponse { peer } => 
                format!("{} {}", green("Response from initial peer:"), magenta(peer)),
            TraceId::PeerConnected { peer } => 
                format!("{} {}", green("Successfully connected to peer:"), magenta(peer)),
            TraceId::PeerKnownCount { peer, count } => 
                format!("{} {} {} {}", 
                    green("Received"), 
                    magenta(&count.to_string()), 
                    green("known peers from"), 
                    magenta(peer)
                ),
            TraceId::PeerConnectError { peer, error } => 
                format!("{} {} {} {}", 
                    green("Failed to connect to peer:"), 
                    magenta(peer), 
                    green("-"), 
                    magenta(error)
                )
        }
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
    format!("[{}]", color::yellow(name))
}

/// Get the current thread ID as a colored string (cyan)
pub fn get_thread_id() -> String {
    format!("[{}]", color::cyan(&format!("{:?}", thread::current().id())))
}
