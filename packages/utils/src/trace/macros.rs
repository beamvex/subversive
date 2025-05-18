/// Macro to log with message ID and thread ID
#[macro_export]
macro_rules! trace_info {
    // For structs with fields
    ($type:ident { $($field:ident : $value:expr),* $(,)? }) => {
        {
            let event = $type { $($field: $value.into()),* };
            ::tracing::info!("{} {} {}",
                $crate::trace::format::get_thread_id(),
                $crate::trace::format::format_msg_id(&event),
                event.message()
            )
        }
    };
    // For unit-like structs
    ($type:ident) => {
        {
            let event = $type;
            ::tracing::info!("{} {} {}",
                $crate::trace::format::get_thread_id(),
                $crate::trace::format::format_msg_id(&event),
                event.message()
            )
        }
    };
}

#[macro_export]
macro_rules! trace_debug {
    // For structs with fields
    ($type:ident { $($field:ident : $value:expr),* $(,)? }) => {
        {
            let event = $type { $($field: $value.into()),* };
            ::tracing::debug!("{} {} {}",
                $crate::trace::format::get_thread_id(),
                $crate::trace::format::format_msg_id(&event),
                event.message()
            )
        }
    };
    // For unit-like structs
    ($type:ident) => {
        {
            let event = $type;
            ::tracing::debug!("{} {} {}",
                $crate::trace::format::get_thread_id(),
                $crate::trace::format::format_msg_id(&event),
                event.message()
            )
        }
    };
}

#[macro_export]
macro_rules! trace_error {
    // For structs with fields
    ($type:ident { $($field:ident : $value:expr),* $(,)? }) => {
        {
            let event = $type { $($field: $value.into()),* };
            ::tracing::error!("{} {} {}",
                $crate::trace::format::get_thread_id(),
                $crate::trace::format::format_msg_id(&event),
                event.message()
            )
        }
    };
    // For unit-like structs
    ($type:ident) => {
        {
            let event = $type;
            ::tracing::error!("{} {} {}",
                $crate::trace::format::get_thread_id(),
                $crate::trace::format::format_msg_id(&event),
                event.message()
            )
        }
    };
}
