use anyhow::Result;
use std::net::IpAddr;
use tracing::debug;

/// Get the hostname for this node
pub async fn get_hostname() -> Result<String> {
    // For now, just return localhost for testing
    debug!("Using localhost as hostname");
    Ok("localhost".to_string())
}

/// Parse an IP address and port from a string
pub fn parse_address(addr: &str) -> Result<(IpAddr, u16)> {
    let parts: Vec<&str> = addr.split(':').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid address format"));
    }

    let ip = parts[0].parse()?;
    let port = parts[1].parse()?;
    Ok((ip, port))
}
