use anyhow::Result;
use mockall::predicate::*;

use crate::network::upnp::{try_setup_upnp, Gateway2, IGateway, MockGatewaySearch, MockIGateway};
use crate::test_utils::init_test_tracing;

// Helper function to initialize tracing for tests
pub fn init_test_upnp() {
    init_test_tracing();
}

#[tokio::test]
async fn test_try_setup_upnp() -> anyhow::Result<()> {
    let port = 12345;
    let mut mock_search = MockGatewaySearch::new();
    let mut mock_gateway = MockIGateway::new();

    mock_gateway
        .expect_add_port()
        .returning(|_, _, _, _, _| Ok(()));

    mock_gateway
        .expect_root_url()
        .returning(|| "http://mock-gateway".to_string());

    mock_search
        .expect_search_gateway()
        .times(1)
        .returning(move || {
            let mut mock = MockIGateway::new();
            mock.expect_root_url()
                .returning(|| "http://mock-gateway".to_string());
            Ok(Gateway2::Mock(mock))
        });

    let gateway = try_setup_upnp(port, mock_search).await?;
    assert_eq!(gateway.root_url(), "http://mock-gateway");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use igd::PortMappingProtocol;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_try_setup_upnp_with_port() -> anyhow::Result<()> {
        let mut mock_search = MockGatewaySearch::new();
        let mut mock_gateway = MockIGateway::new();

        mock_gateway
            .expect_add_port()
            .with(
                eq(PortMappingProtocol::TCP),
                eq(12345),
                always(),
                eq(0),
                eq("Subversive"),
            )
            .times(1)
            .returning(|_, _, _, _, _| Ok(()));

        mock_gateway
            .expect_root_url()
            .times(1)
            .returning(|| "http://192.168.1.1:1900".to_string());

        mock_search
            .expect_search_gateway()
            .times(1)
            .returning(move || {
                let mut mock = MockIGateway::new();
                mock.expect_root_url()
                    .returning(|| "http://192.168.1.1:1900".to_string());
                Ok(Gateway2::Mock(mock))
            });

        let gateway = try_setup_upnp(12345, mock_search).await?;
        assert_eq!(gateway.root_url(), "http://192.168.1.1:1900");

        Ok(())
    }
}

/*
#[tokio::test]
async fn test_try_setup_upnp_failure() -> Result<()> {
    init_tracing();
    let port = 8080;
    let _local_ip = Ipv4Addr::new(192, 168, 1, 100);

    // Create a mock gateway search
    let mut mock_search = MockGatewaySearch::new();
    mock_search
        .expect_search_gateway()
        .times(1)
        .returning(|| Ok(Gateway::new()));

    // Create a mock gateway that fails to add port
    let mut mock_gateway = MockTestGateway::new();
    mock_gateway.expect_add_port().returning(|_, _, _, _, _| {
        Err(igd::AddPortError::RequestError(
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to add port").into(),
        ))
    });
    mock_gateway
        .expect_local_addr()
        .returning(|| "127.0.0.1:0".parse().unwrap());
    mock_gateway
        .expect_root_url()
        .returning(|| "http://mock-gateway".to_string());

    let result = try_setup_upnp(port, mock_search).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_setup_upnp_wsl() -> Result<()> {
    init_tracing();
    // Create a temporary file to simulate WSL environment
    let temp_dir = tempfile::tempdir()?;
    let wsl_path = temp_dir.path().join("WSLInterop");
    std::fs::write(&wsl_path, "")?;

    let mock_search = MockGatewaySearch::new();
    let result = setup_upnp(8080, mock_search).await?;
    assert_eq!(result.1.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_setup_upnp_retry_success() -> Result<()> {
    init_tracing();
    let initial_port = 8080;
    let success_port = 8081;

    // Create a mock gateway search
    let mut mock_search = MockGatewaySearch::new();
    mock_search
        .expect_search_gateway()
        .times(2)
        .returning(|| Ok(Gateway::new()));

    // Create a mock gateway that fails for the first port but succeeds for the second
    let mut mock_gateway = MockTestGateway::new();
    mock_gateway
        .expect_add_port()
        .with(
            eq(igd::PortMappingProtocol::TCP),
            eq(initial_port),
            eq(std::net::SocketAddrV4::new(
                std::net::Ipv4Addr::new(192, 168, 1, 100),
                initial_port,
            )),
            eq(0),
            eq("P2P Network"),
        )
        .returning(|_, _, _, _, _| {
            Err(igd::AddPortError::RequestError(
                std::io::Error::new(std::io::ErrorKind::Other, "Port in use").into(),
            ))
        });
    mock_gateway
        .expect_add_port()
        .with(
            eq(igd::PortMappingProtocol::TCP),
            eq(success_port),
            eq(std::net::SocketAddrV4::new(
                std::net::Ipv4Addr::new(192, 168, 1, 100),
                success_port,
            )),
            eq(0),
            eq("P2P Network"),
        )
        .returning(|_, _, _, _, _| Ok(()));
    mock_gateway
        .expect_local_addr()
        .returning(|| "127.0.0.1:0".parse().unwrap());
    mock_gateway
        .expect_root_url()
        .returning(|| "http://mock-gateway".to_string());

    let result = setup_upnp(initial_port, mock_search).await?;
    let (port, gateways) = result;

    assert_eq!(port, success_port);
    assert_eq!(gateways.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_setup_upnp_max_attempts() -> Result<()> {
    init_tracing();
    let initial_port = 8080;

    // Create a mock gateway search
    let mut mock_search = MockGatewaySearch::new();
    mock_search
        .expect_search_gateway()
        .times(5)
        .returning(|| Ok(Gateway::new()));

    // Create a mock gateway that always fails
    let mut mock_gateway = MockTestGateway::new();
    mock_gateway.expect_add_port().returning(|_, _, _, _, _| {
        Err(igd::AddPortError::RequestError(
            std::io::Error::new(std::io::ErrorKind::Other, "Port in use").into(),
        ))
    });
    mock_gateway
        .expect_local_addr()
        .returning(|| "127.0.0.1:0".parse().unwrap());
    mock_gateway
        .expect_root_url()
        .returning(|| "http://mock-gateway".to_string());

    let result = setup_upnp(initial_port, mock_search).await;

    let is_error = result.is_err();
    let (port, _gateways) = result.unwrap();

    info!("Result: {:?}", port);
    assert!(is_error);

    Ok(())
}

#[tokio::test]
async fn test_cleanup_upnp_success() -> Result<()> {
    init_tracing();
    let port = 8080;

    // Create a mock gateway search
    let mut mock_search = MockGatewaySearch::new();
    mock_search
        .expect_search_gateway()
        .times(1)
        .returning(|| Ok(Gateway::new()));

    // Create a mock gateway
    let mut mock_gateway = MockTestGateway::new();
    mock_gateway
        .expect_remove_port()
        .with(eq(igd::PortMappingProtocol::TCP), eq(port))
        .returning(|_, _| Ok(()));
    mock_gateway
        .expect_local_addr()
        .returning(|| "127.0.0.1:0".parse().unwrap());
    mock_gateway
        .expect_root_url()
        .returning(|| "http://mock-gateway".to_string());

    let gateways: Vec<dyn GatewayInterface> = vec![mock_gateway];
    cleanup_upnp(port, &gateways).await?;

    Ok(())
}

#[tokio::test]
async fn test_cleanup_upnp_failure() -> Result<()> {
    init_tracing();
    let port = 8080;

    // Create a mock gateway search
    let mut mock_search = MockGatewaySearch::new();
    mock_search
        .expect_search_gateway()
        .times(1)
        .returning(|| Ok(Gateway::new()));

    // Create a mock gateway that fails to remove port
    let mut mock_gateway = MockTestGateway::new();
    mock_gateway.expect_remove_port().returning(|_, _| {
        Err(igd::RemovePortError::RequestError(
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove port").into(),
        ))
    });
    mock_gateway
        .expect_local_addr()
        .returning(|| "127.0.0.1:0".parse().unwrap());
    mock_gateway
        .expect_root_url()
        .returning(|| "http://mock-gateway".to_string());

    // Cleanup should succeed even if removing port fails
    let gateways: Vec<dyn GatewayInterface> = vec![mock_gateway];
    cleanup_upnp(port, &gateways).await?;

    Ok(())
}
*/
