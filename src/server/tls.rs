use anyhow::Context;
use axum_server::tls_rustls::RustlsConfig;
use std::path::Path;

/// Configure TLS for the server
pub async fn configure_tls() -> anyhow::Result<RustlsConfig> {
    RustlsConfig::from_pem_file(Path::new("certs/cert.pem"), Path::new("certs/key.pem"))
        .await
        .context("Failed to load TLS certificates")
}
