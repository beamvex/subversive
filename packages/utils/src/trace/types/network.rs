use super::TraceId;
use crate::tui::color::{green, red};

#[derive(Debug, Clone)]
pub struct NetworkConnect {
    pub process: String,
    pub addr: String,
}

#[derive(Debug, Clone)]
pub struct NetworkScan {
    pub process: String,
    pub addr: String,
}

#[derive(Debug, Clone)]
pub struct NetworkError {
    pub process: String,
    pub error: String,
}

impl TraceId for NetworkConnect {
    fn id(&self) -> u64 {
        0x0101
    }
    fn name(&self) -> &'static str {
        "NetworkConnect"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Connected to network at"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for NetworkScan {
    fn id(&self) -> u64 {
        0x0102
    }
    fn name(&self) -> &'static str {
        "NetworkScan"
    }
    fn message(&self) -> String {
        format!("{} {}", green("Scanning network at"), &self.addr)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}

impl TraceId for NetworkError {
    fn id(&self) -> u64 {
        0x0102
    }
    fn name(&self) -> &'static str {
        "NetworkError"
    }
    fn message(&self) -> String {
        format!("{} {}", red("Network error:"), &self.error)
    }
    fn process(&self) -> String {
        self.process.clone()
    }
}
