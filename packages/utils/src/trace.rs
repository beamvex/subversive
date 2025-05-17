use std::sync::atomic::{AtomicU64, Ordering};

static TRACE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generate a unique trace ID for logging
pub fn generate_trace_id() -> String {
    let id = TRACE_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("[{:016x}]", id)
}

/// Macro to log with trace ID
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        tracing::info!("{} {}", $crate::trace::generate_trace_id(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        tracing::debug!("{} {}", $crate::trace::generate_trace_id(), format!($($arg)*))
    }
}

#[macro_export]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        tracing::error!("{} {}", $crate::trace::generate_trace_id(), format!($($arg)*))
    }
}
