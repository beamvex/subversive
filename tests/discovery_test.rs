#[cfg(test)]
mod tests {
    use std::sync::OnceLock;
    use subversive::network::discovery::get_external_ip;
    use mockito::{Server, ServerGuard};
    use test_log::test;
    use tokio::sync::Mutex;

    static TEST_SERVER: OnceLock<Mutex<Option<ServerGuard>>> = OnceLock::new();

    async fn setup_test() -> ServerGuard {
        let server = Server::new();
        let mut guard = TEST_SERVER
            .get_or_init(|| Mutex::new(None))
            .lock()
            .await;
        *guard = Some(server);
        guard.take().unwrap()
    }

    #[tokio::test]
    async fn test_get_external_ip_success() {
        let mut server = setup_test().await;
        let _m = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("1.2.3.4")
            .create();

        let result = get_external_ip().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1.2.3.4");
    }

    #[tokio::test]
    async fn test_get_external_ip_error() {
        let mut server = setup_test().await;
        let _m = server.mock("GET", "/").with_status(500).create();

        let result = get_external_ip().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_external_ip_empty_response() {
        let mut server = setup_test().await;
        let _m = server.mock("GET", "/").with_body("").create();

        let result = get_external_ip().await;
        assert!(
            result.unwrap().is_empty(),
            "Empty response should be returned as-is"
        );
    }
}
