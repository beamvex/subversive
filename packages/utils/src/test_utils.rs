use tracing::Level;
use tracing_subscriber::fmt::format::FmtSpan;

/// Initialize tracing for tests with debug level output
pub fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_max_level(Level::DEBUG)
        .with_span_events(FmtSpan::FULL)
        .try_init();
}
