use anyhow::Result;
use log::{error, info};
use std::net::{Ipv4Addr, SocketAddrV4};

pub async fn try_setup_upnp(port: u16) -> Result<Vec<igd::aio::Gateway>> {
    let gateway = igd::aio::search_gateway(Default::default()).await?;
    Ok(vec![gateway])
}

pub async fn try_add_port_mapping(
    gateways: &mut Vec<igd::aio::Gateway>,
    gateway: igd::aio::Gateway,
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

pub async fn setup_upnp(mut port: u16) -> Result<(u16, Vec<igd::aio::Gateway>)> {
    let interfaces = crate::get_network_interfaces()?;
    let mut gateways = Vec::new();

    for _ in 0..10 {
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
            Err(e) => {
                error!("Failed to set up UPnP: {}", e);
            }
        }
        port += 1;
    }

    Err(anyhow::anyhow!("Failed to set up UPnP after multiple attempts"))
}

pub async fn cleanup_upnp(port: u16, gateways: Vec<igd::aio::Gateway>) {
    for gateway in gateways {
        match gateway
            .remove_port(igd::PortMappingProtocol::TCP, port)
            .await
        {
            Ok(()) => {
                info!("Successfully removed port mapping for port {}", port);
            }
            Err(e) => {
                error!("Failed to remove port mapping: {}", e);
            }
        }
    }
}
