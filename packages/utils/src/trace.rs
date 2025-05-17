use std::thread;

/// Format a message ID for logging with yellow color
pub fn format_msg_id(id: u64) -> String {
    format!("\x1b[33m[{:06x}]\x1b[0m", id)
}

/// Get the current thread ID as a colored string (cyan)
pub fn get_thread_id() -> String {
    format!("\x1b[36m[{:?}]\x1b[0m", thread::current().id())
}

/// Macro to log with message ID and thread ID
#[macro_export]
macro_rules! trace_info {
    ($msg_id:expr, $($arg:tt)*) => {
        tracing::info!("{} {} {}", $crate::trace::format_msg_id($msg_id), $crate::trace::get_thread_id(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! trace_debug {
    ($msg_id:expr, $($arg:tt)*) => {
        tracing::debug!("{} {} {}", $crate::trace::format_msg_id($msg_id), $crate::trace::get_thread_id(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! trace_error {
    ($msg_id:expr, $($arg:tt)*) => {
        tracing::error!("{} {} {}", $crate::trace::format_msg_id($msg_id), $crate::trace::get_thread_id(), format!($($arg)*))
    }
}
