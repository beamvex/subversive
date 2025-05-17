#[derive(Debug, Clone)]
pub enum TraceId {
    StartupInit { port: u16 },
    StartupPoc,
    PeerConnect { peer: String },
    PeerInit { peer: String, source: String },
    NetworkScan,
    UserPrompt,
    BuildHttpClient,
    PeerAlreadyConnected { peer: String },
    PeerRemoved { peer: String },
    PeerNotFound { peer: String },
    PeerLastSeen { peer: String },
    PeerAddOwn { peer: String },
    PeerAddRequest { peer: String },
    PeerResponse { peer: String },
    PeerConnected { peer: String },
    PeerKnownCount { peer: String, count: usize },
    PeerConnectError { peer: String, error: String },
}

impl TraceId {
    pub fn id(&self) -> u64 {
        match self {
            TraceId::StartupInit { .. } => 0x0001,
            TraceId::StartupPoc => 0x0002,
            TraceId::PeerConnect { .. } => 0x0003,
            TraceId::PeerInit { .. } => 0x0004,
            TraceId::NetworkScan => 0x0005,
            TraceId::UserPrompt => 0x0006,
            TraceId::BuildHttpClient => 0x0007,
            TraceId::PeerAlreadyConnected { .. } => 0x0011,
            TraceId::PeerRemoved { .. } => 0x0012,
            TraceId::PeerNotFound { .. } => 0x0013,
            TraceId::PeerLastSeen { .. } => 0x0014,
            TraceId::PeerAddOwn { .. } => 0x0015,
            TraceId::PeerAddRequest { .. } => 0x0016,
            TraceId::PeerResponse { .. } => 0x0017,
            TraceId::PeerConnected { .. } => 0x0018,
            TraceId::PeerKnownCount { .. } => 0x0019,
            TraceId::PeerConnectError { .. } => 0x0015,
        }
    }
}
