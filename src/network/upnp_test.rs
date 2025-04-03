use anyhow::Result;
use async_trait::async_trait;
use mockall::automock;
use mockall::predicate::*;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use crate::network::upnp::{cleanup_upnp, setup_upnp, try_setup_upnp, GatewayInterface};

// Define a new trait for testing that mirrors GatewayInterface
#[automock]
#[async_trait]
pub trait TestGateway {
    async fn add_port(
        &self,
        protocol: igd::PortMappingProtocol,
        external_port: u16,
        local_addr: std::net::SocketAddrV4,
        lease_duration: u32,
        description: &str,
    ) -> Result<(), igd::AddPortError>;

    async fn remove_port(
        &self,
        protocol: igd::PortMappingProtocol,
        external_port: u16,
    ) -> Result<(), igd::RemovePortError>;

    fn local_addr(&self) -> SocketAddr;
    fn root_url(&self) -> String;
}

// Make MockTestGateway implement GatewayInterface
#[async_trait]
impl GatewayInterface for MockTestGateway {
    async fn add_port(
        &self,
        protocol: igd::PortMappingProtocol,
        external_port: u16,
        local_addr: std::net::SocketAddrV4,
        lease_duration: u32,
        description: &str,
    ) -> Result<(), igd::AddPortError> {
        TestGateway::add_port(
            self,
            protocol,
            external_port,
            local_addr,
            lease_duration,
            description,
        )
        .await
    }

    async fn remove_port(
        &self,
        protocol: igd::PortMappingProtocol,
        external_port: u16,
    ) -> Result<(), igd::RemovePortError> {
        TestGateway::remove_port(self, protocol, external_port).await
    }

    fn local_addr(&self) -> SocketAddr {
        TestGateway::local_addr(self)
    }

    fn root_url(&self) -> String {
        TestGateway::root_url(self)
    }
}

#[tokio::test]
async fn test_try_setup_upnp_success() -> Result<()> {
    let port = 8080;
    let local_ip = Ipv4Addr::new(192, 168, 1, 100);

    // Create a mock gateway
    let mut mock_gateway = MockTestGateway::new();
    mock_gateway
        .expect_add_port()
        .with(
            eq(igd::PortMappingProtocol::TCP),
            eq(port),
            eq(std::net::SocketAddrV4::new(local_ip, port)),
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

    let _gateways: Vec<Box<dyn GatewayInterface>> = vec![Box::new(mock_gateway)];
    let gateways = try_setup_upnp(port).await?;
    assert_eq!(gateways.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_try_setup_upnp_failure() -> Result<()> {
    let port = 8080;
    let _local_ip = Ipv4Addr::new(192, 168, 1, 100);

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

    let _gateways: Vec<Box<dyn GatewayInterface>> = vec![Box::new(mock_gateway)];
    let result = try_setup_upnp(port).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_setup_upnp_wsl() -> Result<()> {
    // Create a temporary file to simulate WSL environment
    let temp_dir = tempfile::tempdir()?;
    let wsl_path = temp_dir.path().join("WSLInterop");
    std::fs::write(&wsl_path, "")?;

    // Mock is_wsl to return true
    let (port, gateways) = setup_upnp(8080).await?;

    assert_eq!(port, 8080);
    assert!(gateways.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_setup_upnp_retry_success() -> Result<()> {
    let initial_port = 8080;
    let success_port = 8081;

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

    let _gateways: Vec<Box<dyn GatewayInterface>> = vec![Box::new(mock_gateway)];
    let result = setup_upnp(initial_port).await?;
    let (port, gateways) = result;

    assert_eq!(port, success_port);
    assert_eq!(gateways.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_setup_upnp_max_attempts() -> Result<()> {
    let initial_port = 8080;

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

    let _gateways: Vec<Box<dyn GatewayInterface>> = vec![Box::new(mock_gateway)];
    let result = setup_upnp(initial_port).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_cleanup_upnp_success() -> Result<()> {
    let port = 8080;

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

    let gateways: Vec<Box<dyn GatewayInterface>> = vec![Box::new(mock_gateway)];
    cleanup_upnp(port, &gateways).await?;

    Ok(())
}

#[tokio::test]
async fn test_cleanup_upnp_failure() -> Result<()> {
    let port = 8080;

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
    let gateways: Vec<Box<dyn GatewayInterface>> = vec![Box::new(mock_gateway)];
    cleanup_upnp(port, &gateways).await?;

    Ok(())
}
