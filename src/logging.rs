use tracing::{info, Level};
use tracing_subscriber::fmt::format::FmtSpan;

/// Update the tracing level dynamically
pub fn update_tracing(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    // Reset the tracing subscriber with the new level
    let _ = tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .with_span_events(FmtSpan::CLOSE)
            .with_max_level(level)
            .finish(),
    );

    info!("Log level updated to: {}", log_level);
}
