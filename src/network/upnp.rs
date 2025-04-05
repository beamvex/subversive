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
