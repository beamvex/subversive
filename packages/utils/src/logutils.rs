use tracing::Level;
use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};

/// Update the tracing level
pub fn update_tracing(log_level: &str) {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(format!("{}", level))),
        )
        .with_span_events(FmtSpan::FULL)
        .try_init();
}
