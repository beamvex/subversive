use anyhow::Result;
use async_trait::async_trait;
use igd::aio::Gateway;
use local_ip_address::local_ip;
use log::{error, info};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::Path;

#[async_trait]
pub trait GatewayInterface: Send + Sync + std::fmt::Debug {
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

#[async_trait]
impl GatewayInterface for Gateway {
    async fn add_port(
        &self,
        _protocol: igd::PortMappingProtocol,
        _external_port: u16,
        _local_addr: std::net::SocketAddrV4,
        _lease_duration: u32,
        _description: &str,
    ) -> Result<(), igd::AddPortError> {
        Ok(())
    }

    async fn remove_port(
        &self,
        _protocol: igd::PortMappingProtocol,
        _external_port: u16,
    ) -> Result<(), igd::RemovePortError> {
        Ok(())
    }

    fn local_addr(&self) -> SocketAddr {
        SocketAddrV4::new(Ipv4Addr::from([0, 0, 0, 0]), 0).into()
    }

    fn root_url(&self) -> String {
        "http://0.0.0.0".to_string()
    }
}

pub async fn try_setup_upnp(
    port: u16,
    #[cfg(test)] gateway: &Box<dyn GatewayInterface>,
) -> Result<&Box<dyn GatewayInterface>> {
    #[cfg(not(test))]
    let gateway =
        Box::new(igd::aio::search_gateway(Default::default()).await?) as Box<dyn GatewayInterface>;

    info!("found gateway: {:?}", gateway);

    let local_ip = local_ip().map_err(|e| anyhow::anyhow!("Failed to get local IP: {}", e))?;
    let local_ipv4 = match local_ip {
        std::net::IpAddr::V4(ip) => ip,
        _ => return Err(anyhow::anyhow!("Local IP is not IPv4")),
    };

    info!("Found local IP: {}", local_ipv4);

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
            Ok(&gateway)
        }
        Err(e) => {
            error!("Failed to add port mapping: {}", e);
            Err(anyhow::anyhow!("Failed to add port mapping: {}", e))
        }
    }
}

pub fn is_wsl() -> bool {
    Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
}

pub async fn setup_upnp(
    mut port: u16,
    #[cfg(test)] gateway: Box<dyn GatewayInterface>,
) -> Result<(u16, Vec<Box<dyn GatewayInterface>>)> {
    if is_wsl() {
        info!("WSL2 detected - skipping UPnP port mapping");
        return Ok((port, Vec::new()));
    }
    let mut gateways = Vec::new();

    info!("Searching for UPnP gateway");
    for attempt in 1..=10 {
        info!("Attempt {} of 10", attempt);
        match try_setup_upnp(
            port,
            #[cfg(test)]
            &gateway,
        )
        .await
        {
            Ok(new_gateway) => {
                gateways.push(new_gateway);
                break;
            }
            Err(e) => {
                error!("Failed to set up UPnP on port {}: {}", port, e);
                port += 1;
            }
        }
    }

    Ok((port, gateways))
}

pub async fn cleanup_upnp(port: u16, gateways: &[Box<dyn GatewayInterface>]) -> Result<()> {
    for gateway in gateways {
        if let Err(e) = gateway
            .remove_port(igd::PortMappingProtocol::TCP, port)
            .await
        {
            error!("Failed to remove port mapping: {}", e);
        }
    }
    Ok(())
}
