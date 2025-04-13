use anyhow::Result;
use std::net::Ipv4Addr;
use tracing::info;

/// Get the local IPv4 address of this machine
pub fn get_local_ipv4() -> Result<Ipv4Addr> {
    let local_ip = local_ip().map_err(|e| anyhow::anyhow!("Failed to get local IP: {}", e))?;
    let local_ipv4 = match local_ip {
        std::net::IpAddr::V4(ip) => ip,
        _ => return Err(anyhow::anyhow!("Local IP is not IPv4")),
    };

    info!("Found local IP: {}", local_ipv4);
    Ok(local_ipv4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ipv4() -> Result<()> {
        let ip = get_local_ipv4()?;
        assert!(
            ip.is_private() || ip.is_loopback(),
            "Local IP should be private or loopback"
        );
        Ok(())
    }
}
