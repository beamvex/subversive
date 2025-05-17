/// Macro to log with message ID and thread ID
#[macro_export]
macro_rules! trace_info {
    ($msg_id:expr) => {
        tracing::info!(
            "{} {} {}",
            $crate::trace::format_msg_id(&$msg_id),
            $crate::trace::get_thread_id(),
            $msg_id.message()
        )
    };
}

#[macro_export]
macro_rules! trace_debug {
    ($msg_id:expr) => {
        tracing::debug!(
            "{} {} {}",
            $crate::trace::format_msg_id(&$msg_id),
            $crate::trace::get_thread_id(),
            $msg_id.message()
        )
    };
}

#[macro_export]
macro_rules! trace_error {
    ($msg_id:expr) => {
        tracing::error!(
            "{} {} {}",
            $crate::trace::format_msg_id(&$msg_id),
            $crate::trace::get_thread_id(),
            $msg_id.message()
        )
    };
}
