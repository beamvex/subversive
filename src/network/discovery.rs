use anyhow::Result;
use std::sync::OnceLock;

static IPIFY_URL: OnceLock<String> = OnceLock::new();

/// Set a custom URL for IP discovery (used for testing)
#[cfg(test)]
pub fn set_ip_discovery_url(url: &str) {
    let _ = IPIFY_URL.set(url.to_string());
}

fn get_ipify_url() -> &'static str {
    IPIFY_URL.get().map(|s| s.as_str()).unwrap_or("https://api.ipify.org")
}

/// Get the external IP address of the machine
///
/// # Returns
/// The external IP address as a string
pub async fn get_external_ip() -> Result<String> {
    let response = reqwest::get(get_ipify_url()).await?.text().await?;
    Ok(response)
}
