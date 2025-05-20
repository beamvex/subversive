use crate::tui::color::{blue, bright_red, yellow};
use crate::TraceId;

pub fn format_peer_status_message(status: &str, peer: &str) -> String {
    format!("{}: {}", status, peer)
}

pub fn format_msg_id(trace_id: &impl TraceId) -> String {
    format!("[{}]", yellow(trace_id.name()))
}

pub fn format_process_id(trace_id: &impl TraceId) -> String {
    format!("[{}]", blue(&trace_id.process()))
}

pub fn get_thread_id() -> String {
    format!(
        "[{}]",
        bright_red(std::thread::current().name().unwrap_or("main"))
    )
}
