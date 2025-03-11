use anyhow::Result;
use std::net::IpAddr;
use tokio::net::lookup_host;
use tracing::{info, warn};

use crate::types::config::Config;

/// Get the hostname to use for the server, following priority:
/// 1. Configured hostname
/// 2. DDNS hostname if configured
/// 3. Reverse DNS lookup of external IP
/// 4. External IP address as string
pub async fn resolve_hostname(config: &Config, external_ip: &str) -> String {
    // Check for explicitly configured hostname
    if let Some(hostname) = config.get_hostname() {
        info!("Using configured hostname: {}", hostname);
        return hostname;
    }

    // Check for DDNS hostname
    if let Some(hostname) = get_ddns_hostname(config) {
        info!("Using DDNS hostname: {}", hostname);
        return hostname;
    }

    // Try reverse DNS lookup
    if let Ok(hostname) = reverse_lookup(external_ip).await {
        info!("Using reverse DNS hostname: {}", hostname);
        return hostname;
    }

    // Fallback to IP address
    info!("Using IP address as hostname: {}", external_ip);
    external_ip.to_string()
}

/// Get the DDNS hostname if configured (NoIP or OpenDNS)
fn get_ddns_hostname(config: &Config) -> Option<String> {
    // Try NoIP first
    if let Some(hostname) = &config.noip_hostname {
        if config.noip_username.is_some() && config.noip_password.is_some() {
            return Some(hostname.clone());
        }
    }

    // Then try OpenDNS
    if let Some(hostname) = &config.opendns_hostname {
        if config.opendns_username.is_some() && config.opendns_password.is_some() {
            return Some(hostname.clone());
        }
    }

    None
}

/// Attempt to do a reverse DNS lookup of an IP address
async fn reverse_lookup(ip: &str) -> Result<String> {
    // Parse the IP address
    let ip_addr: IpAddr = ip.parse()?;
    
    // Attempt reverse lookup
    let addrs = lookup_host(format!("{}:0", ip_addr)).await?;
    
    // Look for the first hostname in the results
    for addr in addrs {
        if let Some(hostname) = addr.ip().to_string().strip_suffix(".in-addr.arpa") {
            return Ok(hostname.to_string());
        }
    }

    warn!("No hostname found for IP {}", ip);
    anyhow::bail!("No hostname found")
}
