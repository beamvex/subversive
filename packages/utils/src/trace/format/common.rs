use crate::TraceId;

pub fn format_peer_status_message(status: &str, peer: &str) -> String {
    format!("{}: {}", status, peer)
}

pub fn format_msg_id(trace_id: &impl TraceId) -> String {
    format!("[{}]", trace_id.name())
}

pub fn get_thread_id() -> String {
    format!(
        "[Thread-{}]",
        std::thread::current().name().unwrap_or("main")
    )
}
