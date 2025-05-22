use super::TraceId;
use crate::tui::color::{green, magenta, red, yellow};

#[derive(Debug, Clone)]
pub struct PeerInit {
    pub peer: String,
    pub source: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerList {
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct BuildHttpClient {
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerConnect {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerAlreadyConnected {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerAddOwn {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerAddRequest {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerResponse {
    pub addr: String,
    pub status: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerConnectError {
    pub addr: String,
    pub error: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerConnected {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerKnownCount {
    pub addr: String,
    pub count: usize,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerRemoved {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerNotFound {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerLastSeen {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct ConnectToPeer {
    pub addr: String,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct PeerLastSeenCheck {
    pub addr: String,
    pub last_seen: i64,
    pub process: String,
}

impl TraceId for PeerInit {
    fn id(&self) -> u64 {
        0x0200
    }
    fn name(&self) -> &'static str {
        "PeerInit"
    }
    fn message(&self) -> String {
        format!(
            "{} {} from {}",
            green("Initializing peer"),
            &self.peer,
            &self.source
        )
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for BuildHttpClient {
    fn id(&self) -> u64 {
        0x0200
    }
    fn name(&self) -> &'static str {
        "BuildHttpClient"
    }
    fn message(&self) -> String {
        "Building HTTP client".to_string()
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerConnect {
    fn id(&self) -> u64 {
        0x0201
    }
    fn name(&self) -> &'static str {
        "PeerConnect"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Connected to peer at"), magenta(&self.addr))
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerAlreadyConnected {
    fn id(&self) -> u64 {
        0x0202
    }
    fn name(&self) -> &'static str {
        "PeerAlreadyConnected"
    }
    fn message(&self) -> String {
        format!("{} {}", yellow("Already connected to peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerAddOwn {
    fn id(&self) -> u64 {
        0x0203
    }
    fn name(&self) -> &'static str {
        "PeerAddOwn"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Adding own address to peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerAddRequest {
    fn id(&self) -> u64 {
        0x0204
    }
    fn name(&self) -> &'static str {
        "PeerAddRequest"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Sending add request to peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerResponse {
    fn id(&self) -> u64 {
        0x0205
    }
    fn name(&self) -> &'static str {
        "PeerResponse"
    }
    fn message(&self) -> String {
        format!(
            "{} {} - {}",
            yellow("Peer response from"),
            &self.addr,
            &self.status
        )
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerConnectError {
    fn id(&self) -> u64 {
        0x0206
    }
    fn name(&self) -> &'static str {
        "PeerConnectError"
    }
    fn message(&self) -> String {
        format!(
            "{} {} - {}",
            red("Peer error from"),
            &self.addr,
            &self.error
        )
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerConnected {
    fn id(&self) -> u64 {
        0x0207
    }
    fn name(&self) -> &'static str {
        "PeerConnected"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Successfully connected to peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerKnownCount {
    fn id(&self) -> u64 {
        0x0208
    }
    fn name(&self) -> &'static str {
        "PeerKnownCount"
    }
    fn message(&self) -> String {
        format!(
            "{} {} has {} known peers",
            green("Peer"),
            &self.addr,
            self.count
        )
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerRemoved {
    fn id(&self) -> u64 {
        0x0209
    }
    fn name(&self) -> &'static str {
        "PeerRemoved"
    }
    fn message(&self) -> String {
        format!("{} {}", yellow("Removed peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerNotFound {
    fn id(&self) -> u64 {
        0x020A
    }
    fn name(&self) -> &'static str {
        "PeerNotFound"
    }
    fn message(&self) -> String {
        format!("{} {}", yellow("Peer not found:"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerLastSeen {
    fn id(&self) -> u64 {
        0x020B
    }
    fn name(&self) -> &'static str {
        "PeerLastSeen"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Updated last seen for peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerList {
    fn id(&self) -> u64 {
        0x020C
    }
    fn name(&self) -> &'static str {
        "PeerList"
    }
    fn message(&self) -> String {
        green("Listing peers").to_string()
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for ConnectToPeer {
    fn id(&self) -> u64 {
        0x020D
    }
    fn name(&self) -> &'static str {
        "ConnectToPeer"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Connecting to peer"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for PeerLastSeenCheck {
    fn id(&self) -> u64 {
        0x020E
    }
    fn name(&self) -> &'static str {
        "PeerLastSeenCheck"
    }
    fn message(&self) -> String {
        format!(
            "{} {} {}",
            green("Checking last seen for peer"),
            &self.addr,
            &self.last_seen
        )
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}
