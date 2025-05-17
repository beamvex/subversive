/// Macro to log with message ID and thread ID
#[macro_export]
macro_rules! trace_info {
    ($($arg:tt)*) => {
        println!("{} {} {}",
            $crate::trace::format::get_thread_id(),
            $crate::trace::format::format_msg_id(&$crate::TraceId::$($arg)*),
            $crate::TraceId::$($arg)*.message()
        )
    };
}

#[macro_export]
macro_rules! trace_debug {
    ($($arg:tt)*) => {
        println!("{} {} {}",
            $crate::trace::format::get_thread_id(),
            $crate::trace::format::format_msg_id(&$crate::TraceId::$($arg)*),
            $crate::TraceId::$($arg)*.message()
        )
    };
}

#[macro_export]
macro_rules! trace_error {
    ($($arg:tt)*) => {
        eprintln!("{} {} {}",
            $crate::trace::format::get_thread_id(),
            $crate::trace::format::format_msg_id(&$crate::TraceId::$($arg)*),
            $crate::TraceId::$($arg)*.message()
        )
    };
}
