pub mod colours {
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const RED: &str = "\x1b[31m";
    pub const BLUE: &str = "\x1b[34m";
    pub const WHITE: &str = "\x1b[37m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RESET: &str = "\x1b[0m";
}

#[macro_export]
macro_rules! green {
    ($fmt:expr) => {
        format!(
            "{}{}{}",
            $crate::logging::colours::GREEN,
            $fmt,
            $crate::logging::colours::RESET
        )
    };
}

#[macro_export]
macro_rules! green_start {
    () => {
        print!("{}", $crate::logging::colours::GREEN)
    };
}

#[macro_export]
macro_rules! blue {
    ($fmt:expr) => {
        format!(
            "{}{}{}",
            $crate::logging::colours::BLUE,
            $fmt,
            $crate::logging::colours::RESET
        )
    };
}

#[macro_export]
macro_rules! blue_start {
    () => {
        print!("{}", $crate::logging::colours::BLUE)
    };
}

#[macro_export]
macro_rules! yellow {
    ($fmt:expr) => {
        format!(
            "{}{}{}",
            $crate::logging::colours::YELLOW,
            $fmt,
            $crate::logging::colours::RESET
        )
    };
}

#[macro_export]
macro_rules! yellow_start {
    () => {
        print!("{}", $crate::logging::colours::YELLOW)
    };
}

#[macro_export]
macro_rules! red {
    ($fmt:expr) => {
        format!(
            "{}{}{}",
            $crate::logging::colours::RED,
            $fmt,
            $crate::logging::colours::RESET
        )
    };
}

#[macro_export]
macro_rules! red_start {
    () => {
        print!("{}", $crate::logging::colours::RED)
    };
}

#[macro_export]
macro_rules! magenta_start {
    () => {
        print!("{}", $crate::logging::colours::MAGENTA)
    };
}

#[macro_export]
macro_rules! cyan_start {
    () => {
        print!("{}", $crate::logging::colours::CYAN)
    };
}

#[macro_export]
macro_rules! bold_start {
    () => {
        print!("{}", $crate::logging::colours::BOLD)
    };
}

#[macro_export]
macro_rules! white_start {
    () => {
        print!("{}", $crate::logging::colours::WHITE)
    };
}

#[macro_export]
macro_rules! reset {
    () => {
        print!("{}", $crate::logging::colours::RESET)
    };
}

#[macro_export]
macro_rules! colour_end {
    () => {
        println!("{}", $crate::logging::colours::RESET)
    };
}

#[macro_export]
macro_rules! log_base {
    ($level:expr) => {
        let ts = ::chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let level = $level;
        let file = file!();
        let line = line!();
        let thread = std::thread::current().id();
        $crate::bold_start!();
        $crate::white_start!();
        print!("[{ts}] ");
        $crate::reset!();
        print!("{level} ");
        $crate::reset!();
        $crate::cyan_start!();
        print!("[{file}:{line}] ");
        $crate::reset!();
        $crate::magenta_start!();
        print!("[{thread:?}] ");
        $crate::reset!();
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => {{
        $crate::log_base!($crate::green!("[DEBUG]"));
        $crate::green_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}

#[macro_export]
macro_rules! info {
    ($fmt:expr) => {{
        $crate::log_base!($crate::blue!("[INFO]"));
        $crate::blue_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => {{
        $crate::log_base!($crate::yellow!("[WARN]"));
        $crate::yellow_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => {{
        $crate::log_base!($crate::red!("[ERROR]"));
        $crate::red_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}
