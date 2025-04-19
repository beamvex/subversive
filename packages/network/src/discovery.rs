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
pub async fn set_ip_discovery_url(url: &str) -> tokio::sync::MutexGuard<'_, String> {
    let mut guard = init_ipify_url().lock().await;
    *guard = url.to_string();
    guard
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

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use crate::discovery::{get_external_ip, set_ip_discovery_url};
    use mockito::{Server, ServerGuard};
    use tokio::sync::{Mutex, MutexGuard};

    static MOCK_SERVER: OnceLock<Mutex<Option<ServerGuard>>> = OnceLock::new();

    async fn setup_test() -> MutexGuard<'static, Option<ServerGuard>> {
        let x = MOCK_SERVER.get_or_init(|| Mutex::new(None));
        let mut guard = x.lock().await;
        if guard.is_none() {
            *guard = Some(Server::new_async().await);
            let _ = set_ip_discovery_url(&guard.as_ref().unwrap().url()).await;
        }
        guard
    }

    #[tokio::test]
    async fn test_get_external_ip_success() {
        let mut server = setup_test().await;
        let server = server.as_mut().unwrap();
        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("1.2.3.4")
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1.2.3.4");
    }

    #[tokio::test]
    async fn test_get_external_ip_error() {
        let mut server = setup_test().await;
        let server = server.as_mut().unwrap();
        let _m = server
            .mock("GET", "/")
            .with_status(500)
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_external_ip_empty_response() {
        let mut server = setup_test().await;
        let server = server.as_mut().unwrap();
        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("")
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
}
