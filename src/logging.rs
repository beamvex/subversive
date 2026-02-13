/// ANSI terminal color codes.
///
/// This module provides constants for ANSI escape sequences that can be used
/// to add color to terminal output.
pub mod colours {
    /// Green text
    pub const GREEN: &str = "\x1b[32m";
    /// Yellow text
    pub const YELLOW: &str = "\x1b[33m";
    /// Red text
    pub const RED: &str = "\x1b[31m";
    /// Blue text
    pub const BLUE: &str = "\x1b[34m";
    /// White text
    pub const WHITE: &str = "\x1b[37m";
    /// Magenta text
    pub const MAGENTA: &str = "\x1b[35m";
    /// Cyan text
    pub const CYAN: &str = "\x1b[36m";
    /// Bold text
    pub const BOLD: &str = "\x1b[1m";
    /// Reset all text formatting
    pub const RESET: &str = "\x1b[0m";
}

/// Formats text in green.
///
/// This macro wraps text in ANSI escape sequences to make it appear green
/// in terminal output.
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

/// Starts green text coloring.
///
/// This macro outputs the ANSI escape sequence to start green text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! green_start {
    () => {
        print!("{}", $crate::logging::colours::GREEN)
    };
}

/// Formats text in blue.
///
/// This macro wraps text in ANSI escape sequences to make it appear blue
/// in terminal output.
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

/// Starts blue text coloring.
///
/// This macro outputs the ANSI escape sequence to start blue text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! blue_start {
    () => {
        print!("{}", $crate::logging::colours::BLUE)
    };
}

/// Formats text in yellow.
///
/// This macro wraps text in ANSI escape sequences to make it appear yellow
/// in terminal output.
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

/// Starts yellow text coloring.
///
/// This macro outputs the ANSI escape sequence to start yellow text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! yellow_start {
    () => {
        print!("{}", $crate::logging::colours::YELLOW)
    };
}

/// Formats text in red.
///
/// This macro wraps text in ANSI escape sequences to make it appear red
/// in terminal output.
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

/// Starts red text coloring.
///
/// This macro outputs the ANSI escape sequence to start red text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! red_start {
    () => {
        print!("{}", $crate::logging::colours::RED)
    };
}

/// Starts magenta text coloring.
///
/// This macro outputs the ANSI escape sequence to start magenta text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! magenta_start {
    () => {
        print!("{}", $crate::logging::colours::MAGENTA)
    };
}

/// Starts cyan text coloring.
///
/// This macro outputs the ANSI escape sequence to start cyan text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! cyan_start {
    () => {
        print!("{}", $crate::logging::colours::CYAN)
    };
}

/// Starts bold text formatting.
///
/// This macro outputs the ANSI escape sequence to start bold text,
/// without resetting the formatting afterward.
#[macro_export]
macro_rules! bold_start {
    () => {
        print!("{}", $crate::logging::colours::BOLD)
    };
}

/// Starts white text coloring.
///
/// This macro outputs the ANSI escape sequence to start white text,
/// without resetting the color afterward.
#[macro_export]
macro_rules! white_start {
    () => {
        print!("{}", $crate::logging::colours::WHITE)
    };
}

/// Resets text formatting.
///
/// This macro outputs the ANSI escape sequence to reset all text
/// formatting (color, bold, etc.).
#[macro_export]
macro_rules! reset {
    () => {
        print!("{}", $crate::logging::colours::RESET)
    };
}

/// Resets text formatting and starts a new line.
///
/// This macro outputs the ANSI escape sequence to reset all text
/// formatting and then outputs a newline character.
#[macro_export]
macro_rules! colour_end {
    () => {
        println!("{}", $crate::logging::colours::RESET)
    };
}

/// Base logging macro that formats log messages.
///
/// This macro provides the common formatting for all log messages, including:
/// - Timestamp
/// - Log level
/// - File and line information
/// - Thread ID
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

/// Logs a debug message.
///
/// This macro logs a message at the DEBUG level with green coloring.
/// Debug messages are typically used for detailed information useful during
/// development and troubleshooting.
#[macro_export]
macro_rules! debug {
    ($fmt:expr) => {{
        $crate::log_base!($crate::green!("[DEBUG]"));
        $crate::green_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}

/// Logs an informational message.
///
/// This macro logs a message at the INFO level with blue coloring.
/// Info messages are typically used for general progress and status updates.
#[macro_export]
macro_rules! info {
    ($fmt:expr) => {{
        $crate::log_base!($crate::blue!("[INFO]"));
        $crate::blue_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}

/// Logs a warning message.
///
/// This macro logs a message at the WARN level with yellow coloring.
/// Warning messages indicate potential issues that don't prevent operation
/// but should be investigated.
#[macro_export]
macro_rules! warn {
    ($fmt:expr) => {{
        $crate::log_base!($crate::yellow!("[WARN]"));
        $crate::yellow_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}

/// Logs an error message.
///
/// This macro logs a message at the ERROR level with red coloring.
/// Error messages indicate serious problems that prevent normal operation.
#[macro_export]
macro_rules! error {
    ($fmt:expr) => {{
        $crate::log_base!($crate::red!("[ERROR]"));
        $crate::red_start!();
        print!($fmt);
        $crate::colour_end!();
    }};
}
