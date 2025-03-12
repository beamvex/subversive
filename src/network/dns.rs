use anyhow::Result;
use log::info;
use std::net::IpAddr;
use tokio::net::lookup_host;
use tracing::warn;

/// Attempt to do a reverse DNS lookup of an IP address
pub(crate) async fn reverse_lookup(ip: &str) -> Result<String> {
    // Parse the IP address
    let ip_addr: IpAddr = ip.parse()?;

    info!("Resolving hostname for IP: {}", ip);

    // Attempt reverse lookup
    let addrs = lookup_host(format!("{}:0", ip_addr)).await?;

    // Look for the first hostname in the results
    for addr in addrs {
        if let Some(hostname) = addr.ip().to_string().strip_suffix(".in-addr.arpa") {
            return Ok(hostname.to_string());
        }
    }

    warn!("No hostname found for IP {}: will use IP instead", ip);
    Ok(ip.to_string())
}
