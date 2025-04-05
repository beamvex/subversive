use anyhow::Result;
use async_trait::async_trait;
use igd::aio::Gateway;
use igd::PortMappingProtocol;
use log::{error, info};
use std::net::SocketAddrV4;
use std::path::Path;
#[cfg(test)]
use std::sync::Arc;

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

    fn local_addr(&self) -> SocketAddrV4;
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

    pub fn local_addr(&self) -> SocketAddrV4 {
        self.0.addr
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

    fn local_addr(&self) -> SocketAddrV4 {
        self.0.addr
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

    fn local_addr(&self) -> SocketAddrV4 {
        match self {
            Gateway2::Real(g) => g.local_addr(),
            #[cfg(test)]
            Gateway2::Mock(m) => m.local_addr(),
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
        let gateway = igd::aio::search_gateway(Default::default()).await?;
        Ok(Gateway2::Real(GatewayWrapper::new(gateway)))
    }
}

pub async fn try_setup_upnp(_port: u16, gateway_search: impl GatewaySearch) -> Result<Gateway2> {
    let gateway = gateway_search.search_gateway().await?;

    info!("found gateway: {:?}", gateway.root_url());

    Ok(gateway)
}

pub async fn setup_upnp(port: u16) -> Result<(u16, Vec<Gateway2>)> {
    if is_wsl() {
        info!("WSL2 detected - skipping UPnP port mapping");
        return Ok((port, Vec::new()));
    }

    let mut gateways = Vec::new();

    let gateway_search = DefaultGatewaySearch;
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

fn is_wsl() -> bool {
    Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
}

#[cfg(test)]
pub use mockall::automock;
