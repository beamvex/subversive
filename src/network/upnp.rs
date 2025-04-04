use anyhow::Result;
use igd::aio::Gateway;
use local_ip_address::local_ip;
use log::{error, info};
use std::net::SocketAddrV4;
use std::path::Path;

pub async fn try_setup_upnp(port: u16) -> Result<Gateway> {
    let gateway = igd::aio::search_gateway(Default::default()).await?;

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
            Ok(gateway.clone())
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

pub async fn setup_upnp(mut port: u16) -> Result<(u16, Vec<Gateway>)> {
    if is_wsl() {
        info!("WSL2 detected - skipping UPnP port mapping");
        return Ok((port, Vec::new()));
    }
    let mut gateways = Vec::new();

    info!("Searching for UPnP gateway");
    for attempt in 1..=10 {
        info!("Attempt {} of 10", attempt);
        match try_setup_upnp(port).await {
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

pub async fn cleanup_upnp(port: u16, gateways: Vec<Gateway>) -> Result<()> {
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
