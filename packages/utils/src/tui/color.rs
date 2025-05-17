//! Color formatting utilities for terminal output
//! 
//! This module provides functions for coloring and formatting text output in the terminal.
//! It includes:
//! - Regular foreground colors (e.g., `red`, `green`, `blue`)
//! - Bright foreground colors (e.g., `bright_red`, `bright_green`)
//! - Background colors (e.g., `bg_red`, `bg_green`)
//! - Text formatting (e.g., `bold`, `italic`, `underline`)
//!
//! # Example
//! ```
//! use subversive_utils::trace::color;
//! 
//! println!("{} {} {}",
//!     color::red("Error:"),
//!     color::bold("Failed to connect"),
//!     color::dim("(try again later)")
//! );
//! ```

// Regular foreground colors
/// Colors text black (may be invisible on dark terminals)
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for black foreground color
pub fn black(text: &str) -> String {
    format!("\x1b[30m{}\x1b[0m", text)
}

/// Colors text red - commonly used for errors and warnings
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for red foreground color
pub fn red(text: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", text)
}

/// Colors text green - commonly used for success messages
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for green foreground color
pub fn green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

/// Colors text yellow - commonly used for warnings or important notices
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for yellow foreground color
pub fn yellow(text: &str) -> String {
    format!("\x1b[33m{}\x1b[0m", text)
}

/// Colors text blue - commonly used for informational messages
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for blue foreground color
pub fn blue(text: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", text)
}

/// Colors text magenta - commonly used for special values or parameters
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for magenta foreground color
pub fn magenta(text: &str) -> String {
    format!("\x1b[35m{}\x1b[0m", text)
}

/// Colors text cyan - commonly used for system or process information
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for cyan foreground color
pub fn cyan(text: &str) -> String {
    format!("\x1b[36m{}\x1b[0m", text)
}

/// Colors text white (may be default on light terminals)
/// 
/// # Arguments
/// * `text` - The text to color
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for white foreground color
pub fn white(text: &str) -> String {
    format!("\x1b[37m{}\x1b[0m", text)
}

// Bright foreground colors - higher intensity versions of the regular colors
pub fn bright_black(text: &str) -> String {
    format!("\x1b[90m{}\x1b[0m", text)
}

pub fn bright_red(text: &str) -> String {
    format!("\x1b[91m{}\x1b[0m", text)
}

pub fn bright_green(text: &str) -> String {
    format!("\x1b[92m{}\x1b[0m", text)
}

pub fn bright_yellow(text: &str) -> String {
    format!("\x1b[93m{}\x1b[0m", text)
}

pub fn bright_blue(text: &str) -> String {
    format!("\x1b[94m{}\x1b[0m", text)
}

pub fn bright_magenta(text: &str) -> String {
    format!("\x1b[95m{}\x1b[0m", text)
}

pub fn bright_cyan(text: &str) -> String {
    format!("\x1b[96m{}\x1b[0m", text)
}

pub fn bright_white(text: &str) -> String {
    format!("\x1b[97m{}\x1b[0m", text)
}

// Background colors - sets the background color while keeping the default text color
pub fn bg_black(text: &str) -> String {
    format!("\x1b[40m{}\x1b[0m", text)
}

pub fn bg_red(text: &str) -> String {
    format!("\x1b[41m{}\x1b[0m", text)
}

pub fn bg_green(text: &str) -> String {
    format!("\x1b[42m{}\x1b[0m", text)
}

pub fn bg_yellow(text: &str) -> String {
    format!("\x1b[43m{}\x1b[0m", text)
}

pub fn bg_blue(text: &str) -> String {
    format!("\x1b[44m{}\x1b[0m", text)
}

pub fn bg_magenta(text: &str) -> String {
    format!("\x1b[45m{}\x1b[0m", text)
}

pub fn bg_cyan(text: &str) -> String {
    format!("\x1b[46m{}\x1b[0m", text)
}

pub fn bg_white(text: &str) -> String {
    format!("\x1b[47m{}\x1b[0m", text)
}

// Text formatting - applies various text effects and styles
/// Makes text bold/bright
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for bold formatting
pub fn bold(text: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", text)
}

/// Makes text dim/faint
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for dim formatting
pub fn dim(text: &str) -> String {
    format!("\x1b[2m{}\x1b[0m", text)
}

/// Makes text italic (not widely supported)
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for italic formatting
pub fn italic(text: &str) -> String {
    format!("\x1b[3m{}\x1b[0m", text)
}

/// Underlines text
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for underline formatting
pub fn underline(text: &str) -> String {
    format!("\x1b[4m{}\x1b[0m", text)
}

/// Makes text blink (not widely supported)
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for blink effect
pub fn blink(text: &str) -> String {
    format!("\x1b[5m{}\x1b[0m", text)
}

/// Swaps foreground and background colors
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for reverse video
pub fn reverse(text: &str) -> String {
    format!("\x1b[7m{}\x1b[0m", text)
}

/// Makes text invisible (same color as background)
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for hidden text
pub fn hidden(text: &str) -> String {
    format!("\x1b[8m{}\x1b[0m", text)
}

/// Adds a line through the text
/// 
/// # Arguments
/// * `text` - The text to format
/// 
/// # Returns
/// The text wrapped in ANSI escape codes for strikethrough formatting
pub fn strikethrough(text: &str) -> String {
    format!("\x1b[9m{}\x1b[0m", text)
}
