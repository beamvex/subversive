use crate::tui::color::{green, magenta};

pub trait TraceId: std::fmt::Debug {
    fn id(&self) -> u64;
    fn name(&self) -> &'static str;
    fn message(&self) -> String;
}

// Startup Events
#[derive(Debug, Clone)]
pub struct StartupInit {
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct StartupPoc;

// Network Events
#[derive(Debug, Clone)]
pub struct PeerConnect {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerInit {
    pub peer: String,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct NetworkScan;

#[derive(Debug, Clone)]
pub struct UserPrompt;

#[derive(Debug, Clone)]
pub struct BuildHttpClient;

// Peer Status Events
#[derive(Debug, Clone)]
pub struct PeerAlreadyConnected {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerRemoved {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerNotFound {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerLastSeen {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerAddOwn {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerAddRequest {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerResponse {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerConnected {
    pub peer: String,
}

#[derive(Debug, Clone)]
pub struct PeerKnownCount {
    pub peer: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct PeerConnectError {
    pub peer: String,
    pub error: String,
}

// Implementations
impl TraceId for StartupInit {
    fn id(&self) -> u64 { 0x0001 }
    fn name(&self) -> &'static str { "StartupInit" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Starting subversive on port"), green(&self.port.to_string()))
    }
}

impl TraceId for StartupPoc {
    fn id(&self) -> u64 { 0x0002 }
    fn name(&self) -> &'static str { "StartupPoc" }
    fn message(&self) -> String {
        green("Starting subversive poc going to run multiple peers at once to test the network").to_string()
    }
}

impl TraceId for PeerConnect {
    fn id(&self) -> u64 { 0x0003 }
    fn name(&self) -> &'static str { "PeerConnect" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Connecting to peer"), green(&self.peer))
    }
}

impl TraceId for PeerInit {
    fn id(&self) -> u64 { 0x0004 }
    fn name(&self) -> &'static str { "PeerInit" }
    fn message(&self) -> String {
        format!("{} {} from {}", magenta("Initializing peer"), green(&self.peer), green(&self.source))
    }
}

impl TraceId for NetworkScan {
    fn id(&self) -> u64 { 0x0005 }
    fn name(&self) -> &'static str { "NetworkScan" }
    fn message(&self) -> String {
        green("Scanning network for peers").to_string()
    }
}

impl TraceId for UserPrompt {
    fn id(&self) -> u64 { 0x0006 }
    fn name(&self) -> &'static str { "UserPrompt" }
    fn message(&self) -> String {
        green("Press Ctrl+C to exit").to_string()
    }
}

impl TraceId for BuildHttpClient {
    fn id(&self) -> u64 { 0x0007 }
    fn name(&self) -> &'static str { "BuildHttpClient" }
    fn message(&self) -> String {
        green("Building HTTP client for peer connection").to_string()
    }
}

impl TraceId for PeerAlreadyConnected {
    fn id(&self) -> u64 { 0x0011 }
    fn name(&self) -> &'static str { "PeerAlreadyConnected" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Already connected to peer"), green(&self.peer))
    }
}

impl TraceId for PeerRemoved {
    fn id(&self) -> u64 { 0x0012 }
    fn name(&self) -> &'static str { "PeerRemoved" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Removed peer"), green(&self.peer))
    }
}

impl TraceId for PeerNotFound {
    fn id(&self) -> u64 { 0x0013 }
    fn name(&self) -> &'static str { "PeerNotFound" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Peer not found"), green(&self.peer))
    }
}

impl TraceId for PeerLastSeen {
    fn id(&self) -> u64 { 0x0014 }
    fn name(&self) -> &'static str { "PeerLastSeen" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Last seen peer"), green(&self.peer))
    }
}

impl TraceId for PeerAddOwn {
    fn id(&self) -> u64 { 0x0015 }
    fn name(&self) -> &'static str { "PeerAddOwn" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Adding own peer"), green(&self.peer))
    }
}

impl TraceId for PeerAddRequest {
    fn id(&self) -> u64 { 0x0016 }
    fn name(&self) -> &'static str { "PeerAddRequest" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Requesting to add peer"), green(&self.peer))
    }
}

impl TraceId for PeerResponse {
    fn id(&self) -> u64 { 0x0017 }
    fn name(&self) -> &'static str { "PeerResponse" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Peer response"), green(&self.peer))
    }
}

impl TraceId for PeerConnected {
    fn id(&self) -> u64 { 0x0018 }
    fn name(&self) -> &'static str { "PeerConnected" }
    fn message(&self) -> String {
        format!("{} {}", magenta("Connected to peer"), green(&self.peer))
    }
}

impl TraceId for PeerKnownCount {
    fn id(&self) -> u64 { 0x0019 }
    fn name(&self) -> &'static str { "PeerKnownCount" }
    fn message(&self) -> String {
        format!("{} {} ({})", magenta("Known peers for"), green(&self.peer), green(&self.count.to_string()))
    }
}

impl TraceId for PeerConnectError {
    fn id(&self) -> u64 { 0x001A }
    fn name(&self) -> &'static str { "PeerConnectError" }
    fn message(&self) -> String {
        format!("{} {}: {}", magenta("Error connecting to peer"), green(&self.peer), green(&self.error))
    }
}
