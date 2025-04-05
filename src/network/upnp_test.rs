#[cfg(test)]
mod tests {
    use crate::network::cleanup_upnp;

    #[cfg(test)]
    use mockall::predicate::*;
    use std::sync::Arc;
    use tracing::info;

    use crate::network::upnp::{
        set_wsl_path, setup_upnp_with_search, try_setup_upnp, DefaultGatewaySearch, Gateway2,
        IGateway, MockGatewaySearch, MockIGateway,
    };
    use crate::test_utils::init_test_tracing;
    use igd::PortMappingProtocol;

    pub fn init_test_upnp() {
        init_test_tracing();
    }

    #[tokio::test]
    async fn test_try_setup_upnp() -> anyhow::Result<()> {
        let port = 12345;
        let mut mock_search = MockGatewaySearch::new();

        mock_search
            .expect_search_gateway()
            .times(1)
            .returning(move || {
                let mut mock = MockIGateway::new();
                mock.expect_root_url()
                    .returning(|| "http://mock-gateway".to_string());
                mock.expect_add_port().returning(|_, _, _, _, _| Ok(()));
                Ok(Gateway2::Mock(Arc::new(mock)))
            });

        let gateway = try_setup_upnp(port, mock_search).await?;
        assert_eq!(gateway.root_url(), "http://mock-gateway");

        Ok(())
    }

    #[tokio::test]
    async fn test_setup_upnp_wsl() -> anyhow::Result<()> {
        init_test_upnp();
        // Create a temporary file to simulate WSL environment
        let temp_dir = tempfile::tempdir()?;
        let wsl_path = temp_dir.path().join("WSLInterop");
        std::fs::write(&wsl_path, "")?;

        info!("WSL2 temp file {}", wsl_path.display());

        let _ = set_wsl_path(wsl_path.display().to_string().as_str()).await;

        let result = setup_upnp_with_search(8080, DefaultGatewaySearch).await?;
        assert_eq!(result.1.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_setup_upnp_success() -> anyhow::Result<()> {
        init_test_upnp();
        let success_port = 8080;
        let local_ipv4 = crate::network::local_ip::get_local_ipv4()?;

        // Create a mock gateway that fails for the first port but succeeds for the second
        let mut mock_gateway = MockIGateway::new();
        mock_gateway
            .expect_add_port()
            .with(
                eq(PortMappingProtocol::TCP),
                eq(success_port),
                eq(std::net::SocketAddrV4::new(local_ipv4, success_port)),
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

        let mg = Gateway2::Mock(Arc::new(mock_gateway));

        // Create a mock gateway search
        let mut mock_search = MockGatewaySearch::new();
        mock_search
            .expect_search_gateway()
            .returning(move || Ok(mg.clone()));

        let result = setup_upnp_with_search(success_port, mock_search).await?;
        let (port, gateways) = result;

        assert_eq!(port, success_port);
        assert_eq!(gateways.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_cleanup_upnp_success() -> anyhow::Result<()> {
        init_test_upnp();
        let port = 8080;

        // Create a mock gateway
        let mut mock_gateway = MockIGateway::new();
        mock_gateway
            .expect_remove_port()
            .with(eq(PortMappingProtocol::TCP), eq(port))
            .returning(|_, _| Ok(()));
        mock_gateway
            .expect_local_addr()
            .returning(|| "127.0.0.1:0".parse().unwrap());
        mock_gateway
            .expect_root_url()
            .returning(|| "http://mock-gateway".to_string());

        // Cleanup should succeed even if removing port fails
        let gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];
        cleanup_upnp(port, gateways).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_cleanup_upnp_failure() -> anyhow::Result<()> {
        init_test_upnp();
        let port = 8080;

        // Create a mock gateway that fails to remove port
        let mut mock_gateway = MockIGateway::new();
        mock_gateway.expect_remove_port().returning(|_, _| {
            Err(igd::RemovePortError::RequestError(
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove port").into(),
            )
            .into())
        });
        mock_gateway
            .expect_local_addr()
            .returning(|| "127.0.0.1:0".parse().unwrap());
        mock_gateway
            .expect_root_url()
            .returning(|| "http://mock-gateway".to_string());

        // Cleanup should succeed even if removing port fails
        let gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];
        cleanup_upnp(port, gateways).await?;

        Ok(())
    }
}
