use crate::tui::color::{green, magenta};
use crate::TraceId;

pub fn format_startup_category(trace_id: &TraceId) -> Option<String> {
    match trace_id {
        TraceId::StartupInit { port } => Some(format_startup_message(port)),
        TraceId::StartupPoc => Some(format_startup_message_simple()),
        TraceId::BuildHttpClient => Some(green("Building HTTP client for peer connection")),
        TraceId::UserPrompt => Some(green("Press Ctrl+C to exit")),
        _ => None,
    }
}

pub fn format_startup_message(port: &u16) -> String {
    format!(
        "{} {}",
        magenta("Starting subversive on port"),
        green(&port.to_string())
    )
}

pub fn format_startup_message_simple() -> String {
    green("Starting subversive poc going to run multiple peers at once to test the network")
}
