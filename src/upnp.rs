use anyhow::Result;
use igd::aio::Gateway;
use log::{error, info};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};

pub async fn try_setup_upnp(port: u16) -> Result<Vec<Gateway>> {
    let gateway = igd::aio::search_gateway(Default::default()).await?;
    match gateway
        .add_port(
            igd::PortMappingProtocol::TCP,
            port,
            SocketAddrV4::new(Ipv4Addr::LOCALHOST, port),
            0,
            "P2P Network",
        )
        .await
    {
        Ok(()) => {
            info!("Successfully added port mapping for port {}", port);
            Ok(vec![gateway])
        }
        Err(e) => {
            error!("Failed to add port mapping: {}", e);
            Err(anyhow::anyhow!("Failed to add port mapping: {}", e))
        }
    }
}

pub async fn try_add_port_mapping(
    gateways: &mut Vec<Gateway>,
    gateway: Gateway,
    port: u16,
    interface: Ipv4Addr,
) {
    match gateway
        .add_port(
            igd::PortMappingProtocol::TCP,
            port,
            SocketAddrV4::new(interface, port),
            0,
            "P2P Network",
        )
        .await
    {
        Ok(()) => {
            info!("Successfully added port mapping for port {}", port);
            gateways.push(gateway);
        }
        Err(e) => {
            error!("Failed to add port mapping: {}", e);
        }
    }
}

pub async fn setup_upnp(mut port: u16) -> Result<(u16, Vec<Gateway>)> {
    let interfaces = crate::get_network_interfaces()?;
    let mut gateways = Vec::new();
    let mut attempts = 0;
    let max_attempts = 10;

    while attempts < max_attempts {
        match try_setup_upnp(port).await {
            Ok(found_gateways) => {
                for gateway in found_gateways {
                    for interface in &interfaces {
                        try_add_port_mapping(&mut gateways, gateway.clone(), port, *interface).await;
                    }
                }
                if !gateways.is_empty() {
                    return Ok((port, gateways));
                }
            }
            Err(_) => {
                port += 1;
                attempts += 1;
            }
        }
    }

    Err(anyhow::anyhow!("Failed to set up UPnP after multiple attempts"))
}

pub async fn cleanup_upnp(port: u16, gateways: Vec<Gateway>) -> Result<()> {
    for gateway in gateways {
        if let Err(e) = gateway.remove_port(igd::PortMappingProtocol::TCP, port).await {
            error!("Error removing port mapping: {}", e);
        }
    }
    Ok(())
}
