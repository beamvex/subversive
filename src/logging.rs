#[macro_export]
macro_rules! log {
    ($fmt:expr) => {
        println!("[{}:{}] {}", file!(), line!(), $fmt);
    };

    ($fmt:expr, $($arg:tt)*) => {
        println!("[{}:{}] {}", file!(), line!(), format_args!($fmt, $($arg)*));
    };
}
