use crate::tui::color::green;
use crate::TraceId;

pub fn format_network_scan_category(trace_id: &TraceId) -> Option<String> {
    match trace_id {
        TraceId::NetworkScan => Some(green("Starting network scan")),
        _ => None,
    }
}
