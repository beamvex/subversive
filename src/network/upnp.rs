use anyhow::Result;
use async_trait::async_trait;
use igd::aio::Gateway;
use igd::PortMappingProtocol;
use log::{error, info};
use std::net::SocketAddrV4;
use std::path::Path;
#[cfg(test)]
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::Mutex;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IGateway: Send + Sync {
    async fn add_port(
        &self,
        protocol: PortMappingProtocol,
        external_port: u16,
        internal_addr: SocketAddrV4,
        lease_duration: u32,
        description: &str,
    ) -> Result<()>;

    async fn remove_port(&self, protocol: PortMappingProtocol, external_port: u16) -> Result<()>;

    fn root_url(&self) -> String;
}

#[derive(Clone)]
pub struct GatewayWrapper(Gateway);

impl GatewayWrapper {
    pub fn new(gateway: Gateway) -> Self {
        Self(gateway)
    }

    pub fn root_url(&self) -> String {
        self.0.root_url.to_string()
    }
}

#[async_trait]
impl IGateway for GatewayWrapper {
    async fn add_port(
        &self,
        protocol: PortMappingProtocol,
        external_port: u16,
        internal_addr: SocketAddrV4,
        lease_duration: u32,
        description: &str,
    ) -> Result<()> {
        Ok(self
            .0
            .add_port(
                protocol,
                external_port,
                internal_addr,
                lease_duration,
                description,
            )
            .await?)
    }

    async fn remove_port(&self, protocol: PortMappingProtocol, external_port: u16) -> Result<()> {
        Ok(self.0.remove_port(protocol, external_port).await?)
    }

    fn root_url(&self) -> String {
        self.0.root_url.to_string()
    }
}

#[cfg(test)]
#[derive(Clone)]
pub enum Gateway2 {
    Real(GatewayWrapper),
    Mock(Arc<MockIGateway>),
}

#[cfg(not(test))]
#[derive(Clone)]
pub enum Gateway2 {
    Real(GatewayWrapper),
}

#[async_trait]
impl IGateway for Gateway2 {
    async fn add_port(
        &self,
        protocol: PortMappingProtocol,
        external_port: u16,
        internal_addr: SocketAddrV4,
        lease_duration: u32,
        description: &str,
    ) -> Result<()> {
        match self {
            Gateway2::Real(g) => {
                g.add_port(
                    protocol,
                    external_port,
                    internal_addr,
                    lease_duration,
                    description,
                )
                .await
            }
            #[cfg(test)]
            Gateway2::Mock(m) => {
                m.add_port(
                    protocol,
                    external_port,
                    internal_addr,
                    lease_duration,
                    description,
                )
                .await
            }
        }
    }

    async fn remove_port(&self, protocol: PortMappingProtocol, external_port: u16) -> Result<()> {
        match self {
            Gateway2::Real(g) => g.remove_port(protocol, external_port).await,
            #[cfg(test)]
            Gateway2::Mock(m) => m.remove_port(protocol, external_port).await,
        }
    }

    fn root_url(&self) -> String {
        match self {
            Gateway2::Real(g) => g.root_url(),
            #[cfg(test)]
            Gateway2::Mock(m) => m.root_url(),
        }
    }
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait GatewaySearch {
    async fn search_gateway(&self) -> Result<Gateway2>;
}

pub struct DefaultGatewaySearch;

#[async_trait]
impl GatewaySearch for DefaultGatewaySearch {
    async fn search_gateway(&self) -> Result<Gateway2> {
        Ok(Gateway2::Real(GatewayWrapper::new(
            igd::aio::search_gateway(Default::default()).await?,
        )))
    }
}

pub async fn try_setup_upnp(port: u16, gateway_search: impl GatewaySearch) -> Result<Gateway2> {
    let gateway = gateway_search.search_gateway().await?;
    let local_ipv4 = crate::network::local_ip::get_local_ipv4()?;

    info!("found gateway: {:?}", gateway.root_url());

    match gateway
        .add_port(
            igd::PortMappingProtocol::TCP,
            port,
            SocketAddrV4::new(local_ipv4, port),
            0,
            "P2P Network",
        )
        .await
    {
        Ok(()) => {
            info!(
                "Successfully added port mapping for port {} using IP {}",
                port, local_ipv4
            );
            Ok(gateway)
        }
        Err(e) => {
            error!("Failed to add port mapping: {}", e);
            Err(anyhow::anyhow!("Failed to add port mapping: {}", e))
        }
    }
}

pub async fn setup_upnp(port: u16) -> Result<(u16, Vec<Gateway2>)> {
    setup_upnp_with_search(port, DefaultGatewaySearch).await
}

pub async fn setup_upnp_with_search(
    port: u16,
    gateway_search: impl GatewaySearch,
) -> Result<(u16, Vec<Gateway2>)> {
    if is_wsl().await {
        info!("WSL2 detected - skipping UPnP port mapping");
        return Ok((port, Vec::new()));
    }

    let mut gateways = Vec::new();

    if let Ok(gateway) = try_setup_upnp(port, gateway_search).await {
        gateways.push(gateway);
    }

    Ok((port, gateways))
}

pub async fn cleanup_upnp(port: u16, gateways: Vec<Gateway2>) -> Result<()> {
    for gateway in gateways {
        if let Err(e) = gateway
            .remove_port(igd::PortMappingProtocol::TCP, port)
            .await
        {
            error!("failed to remove port mapping: {}", e);
        }
    }
    Ok(())
}

static WSL_PATH: OnceLock<Mutex<String>> = OnceLock::new();

/// Initialize the WSL_PATH Mutex with the default path
async fn init_wsl_path() -> &'static Mutex<String> {
    let path =
        WSL_PATH.get_or_init(|| Mutex::new("/proc/sys/fs/binfmt_misc/WSLInterop".to_string()));
    info!("init WSL path: {}", path.lock().await.as_str());
    path
}

/// Set a custom path for WSL detection (used for testing)
#[cfg(test)]
pub async fn set_wsl_path(path: &str) -> tokio::sync::MutexGuard<'_, String> {
    info!("set WSL path: {}", path);
    let mut guard = init_wsl_path().await.lock().await;
    *guard = path.to_string();
    guard
}

async fn is_wsl() -> bool {
    let path = init_wsl_path().await.lock().await;
    info!("WSL path: {}", path.as_str());
    Path::new(path.as_str()).exists()
}

#[cfg(test)]
pub use mockall::automock;

#[cfg(test)]
mod tests {
    use crate::network::cleanup_upnp;

    #[cfg(test)]
    use mockall::predicate::*;

    use std::sync::Arc;

    use tracing::info;

    use crate::network::upnp::{
        set_wsl_path, setup_upnp_with_search, try_setup_upnp, DefaultGatewaySearch, Gateway2,
        GatewayWrapper, IGateway, MockGatewaySearch, MockIGateway,
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
            .expect_root_url()
            .returning(|| "http://mock-gateway".to_string());

        // Cleanup should succeed even if removing port fails
        let gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];
        cleanup_upnp(port, gateways).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_try_setup_upnp_with_mockito() -> anyhow::Result<()> {
        use mockito::Server;
        use std::net::Ipv4Addr;

        let mut server = Server::new();
        let port = 12345;

        // Mock the device description response
        let device_desc = r#"<?xml version="1.0"?>
        <root xmlns="urn:schemas-upnp-org:device-1-0">
            <specVersion>
                <major>1</major>
                <minor>0</minor>
            </specVersion>
            <device>
                <deviceType>urn:schemas-upnp-org:device:InternetGatewayDevice:1</deviceType>
                <friendlyName>Mock Gateway</friendlyName>
                <manufacturer>Mock Manufacturer</manufacturer>
                <manufacturerURL>http://www.example.com</manufacturerURL>
                <modelDescription>Mock Gateway Device</modelDescription>
                <modelName>Mock Gateway</modelName>
                <modelNumber>1.0</modelNumber>
                <modelURL>http://www.example.com</modelURL>
                <serialNumber>12345678</serialNumber>
                <UDN>uuid:Mock-Gateway-1_0-12345678</UDN>
                <serviceList>
                    <service>
                        <serviceType>urn:schemas-upnp-org:service:WANIPConnection:1</serviceType>
                        <serviceId>urn:upnp-org:serviceId:WANIPConnection:1</serviceId>
                        <controlURL>/upnp/control/WANIPConnection</controlURL>
                        <eventSubURL>/upnp/event/WANIPConnection</eventSubURL>
                        <SCPDURL>/WANIPConnection.xml</SCPDURL>
                    </service>
                </serviceList>
            </device>
        </root>"#;

        // Mock the device description endpoint
        let _m1 = server
            .mock("GET", "/rootDesc.xml")
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(device_desc);

        // Mock the add port mapping endpoint
        let _m2 = server.mock("POST", "/upnp/control/WANIPConnection")
            .with_status(200)
            .with_header("content-type", "text/xml")
            .with_body(r#"<?xml version="1.0"?>
                <s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/">
                    <s:Body>
                        <u:AddPortMappingResponse xmlns:u="urn:schemas-upnp-org:service:WANIPConnection:1"/>
                    </s:Body>
                </s:Envelope>"#);

        // Create a mock search that returns our mockito server URL
        let mut mock_search = MockGatewaySearch::new();
        let server_url = server.url();

        mock_search
            .expect_search_gateway()
            .times(1)
            .returning(move || {
                let gateway = Gateway2::Real(GatewayWrapper::new(igd::aio::Gateway {
                    addr: std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 0),
                    root_url: "http://mock-gateway".to_string(),
                    control_url: "http://mock-gateway/upnp/control/WANIPConnection".to_string(),
                    control_schema_url: "http://mock-gateway/upnp/event/WANIPConnection"
                        .to_string(),
                    control_schema: "http://schemas.upnp.org/wanipconnection/1-0".to_string(),
                }));
                Ok(gateway)
            });

        let gateway = try_setup_upnp(port, mock_search).await?;
        assert!(gateway.root_url().contains(&server.url()));

        Ok(())
    }
}
