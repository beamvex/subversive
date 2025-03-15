#[cfg(test)]
mod tests {
    use crate::network::discovery::{get_external_ip, set_ip_discovery_url};
    use mockito::Server;

    #[tokio::test]
    async fn test_get_external_ip_success() {
        let mut server = Server::new_async().await;
        set_ip_discovery_url(&server.url());
        
        let _m = server.mock("GET", "/")
            .with_status(200)
            .with_body("203.0.113.1")
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "203.0.113.1");
    }

    #[tokio::test]
    async fn test_get_external_ip_server_error() {
        let mut server = Server::new_async().await;
        set_ip_discovery_url(&server.url());
        
        let _m = server.mock("GET", "/")
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_external_ip_invalid_response() {
        let mut server = Server::new_async().await;
        set_ip_discovery_url(&server.url());
        
        let _m = server.mock("GET", "/")
            .with_status(200)
            .with_body("")  // Empty response should cause an error
            .create_async()
            .await;

        let result = get_external_ip().await;
        assert!(result.is_err());
    }
}
