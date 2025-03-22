use anyhow::Result;
use dns_lookup::lookup_addr;
use log::info;
use std::net::IpAddr;
use tracing::warn;

/// Attempt to do a reverse DNS lookup of an IP address
pub(crate) async fn reverse_lookup(ip: &str) -> Result<String> {
    // Parse the IP address
    let ip_addr: IpAddr = ip.parse()?;

    info!("Resolving hostname for IP: {}", ip);

    // Attempt reverse lookup using dns-lookup
    match lookup_addr(&ip_addr) {
        Ok(hostname) => Ok(hostname),
        Err(e) => {
            warn!(
                "No hostname found for IP {}: {} - will use IP instead",
                ip, e
            );
            Ok(ip.to_string())
        }
    }
}
