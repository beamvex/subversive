use crate::tui::color::{green, magenta};
use super::TraceId;

#[derive(Debug, Clone)]
pub struct StartupInit {
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct StartupPoc;

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
