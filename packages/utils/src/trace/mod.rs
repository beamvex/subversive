mod types;
mod format;
mod macros;

pub use types::TraceId;
pub use format::{format_msg_id, get_thread_id};

// Re-export macros
pub use crate::{trace_info, trace_debug, trace_error};
