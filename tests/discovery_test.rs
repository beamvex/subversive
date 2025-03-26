#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use crate::network::discovery::{get_external_ip, set_ip_discovery_url};
    use mockito::{Server, ServerGuard};
    use test_log::test;
    use tokio::sync::Mutex;
    use tracing::info;

    static SERVER: OnceLock<Mutex<ServerGuard>> = OnceLock::new();

    #[test(tokio::test)]
    async fn test_get_external_ip_success() {
        info!("test_get_external_ip_success");
        let server = Server::new_async().await;
        let mut server = SERVER.get_or_init(|| Mutex::new(server)).lock().await;

        info!("test_get_external_ip_success {}", server.url());
        set_ip_discovery_url(&server.url()).await;

        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("203.0.113.1")
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "203.0.113.1");
    }

    #[test(tokio::test)]
    async fn test_get_external_ip_server_error() {
        info!("test_get_external_ip_server_error");
        let server = Server::new_async().await;
        let mut server = SERVER.get_or_init(|| Mutex::new(server)).lock().await;
        info!("test_get_external_ip_server_error {}", server.url());
        set_ip_discovery_url(&server.url()).await;
        let _m = server
            .mock("GET", "/")
            .with_status(500)
            .with_header("content-length", "0")
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("500") || err.contains("server error"),
            "Expected error to mention 500 or server error, got: {}",
            err
        );
    }

    #[test(tokio::test)]
    async fn test_get_external_ip_malformed_response() {
        info!("test_get_external_ip_malformed_response");
        let server = Server::new_async().await;
        let mut server = SERVER.get_or_init(|| Mutex::new(server)).lock().await;
        info!("test_get_external_ip_malformed_response {}", server.url());
        set_ip_discovery_url(&server.url()).await;
        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("") // Empty response
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "",
            "Empty response should be returned as-is"
        );
    }
}
