use anyhow::Result;
use std::sync::OnceLock;
use tokio::sync::Mutex;
use tracing::info;

static IPIFY_URL: OnceLock<Mutex<String>> = OnceLock::new();

/// Initialize the IPIFY_URL Mutex with the default URL
fn init_ipify_url() -> &'static Mutex<String> {
    IPIFY_URL.get_or_init(|| Mutex::new("https://api.ipify.org".to_string()))
}

/// Set a custom URL for IP discovery (used for testing)
#[cfg(test)]
pub async fn set_ip_discovery_url(url: &str) {
    let mut guard = init_ipify_url().lock().await;
    *guard = url.to_string();
}

/// Get the external IP address of the machine
///
/// # Returns
/// The external IP address as a string
pub async fn get_external_ip() -> Result<String> {
    let url = init_ipify_url().lock().await.clone();
    info!("Getting external IP from {}", url);
    let response = reqwest::get(&url).await?.error_for_status()?.text().await?;
    Ok(response)
}
