use std::net::IpAddr;
use tracing::{debug, error};

use crate::peer::PeerInfo;

pub async fn reverse_lookup(addr: &str) -> Result<String, String> {
    // Parse the IP address
    let ip = match addr.parse::<IpAddr>() {
        Ok(ip) => ip,
        Err(e) => {
            error!("Failed to parse IP address {}: {}", addr, e);
            return Err(format!("Invalid IP address: {}", e));
        }
    };

    // Special cases
    match ip {
        IpAddr::V4(ipv4) => {
            if ipv4.is_loopback() {
                return Ok("localhost".to_string());
            }
            if ipv4.is_unspecified() {
                return Ok("unspecified".to_string());
            }
            if ipv4.is_broadcast() {
                return Ok("broadcast".to_string());
            }
            if ipv4.is_multicast() {
                return Ok("multicast".to_string());
            }
        }
        IpAddr::V6(ipv6) => {
            if ipv6.is_loopback() {
                return Ok("localhost".to_string());
            }
            if ipv6.is_unspecified() {
                return Ok("unspecified".to_string());
            }
            if ipv6.is_multicast() {
                return Ok("multicast".to_string());
            }
        }
    }

    // For all other cases, return the original IP
    debug!("No reverse DNS entry found for {}, using IP", addr);
    Ok(addr.to_string())
}

pub async fn get_peer_info(addr: &str) -> Result<PeerInfo, String> {
    let parts: Vec<&str> = addr.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid address format".to_string());
    }

    let port = parts[1].parse::<u16>().map_err(|e| e.to_string())?;
    Ok(PeerInfo {
        address: parts[0].to_string(),
        port,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_reverse_lookup() {
        // Test localhost
        let result = reverse_lookup("127.0.0.1").await.unwrap();
        assert_eq!(result, "localhost");

        // Test IPv6 localhost
        let result = reverse_lookup("::1").await.unwrap();
        assert_eq!(result, "localhost");

        // Test unspecified
        let result = reverse_lookup("0.0.0.0").await.unwrap();
        assert_eq!(result, "unspecified");

        // Test IPv6 unspecified
        let result = reverse_lookup("::").await.unwrap();
        assert_eq!(result, "unspecified");

        // Test broadcast
        let result = reverse_lookup("255.255.255.255").await.unwrap();
        assert_eq!(result, "broadcast");

        // Test multicast
        let result = reverse_lookup("224.0.0.1").await.unwrap();
        assert_eq!(result, "multicast");

        // Test invalid IP
        let result = reverse_lookup("invalid").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_peer_info() {
        // Test valid address
        let result = get_peer_info("127.0.0.1:8080").await.unwrap();
        assert_eq!(result.address, "127.0.0.1");
        assert_eq!(result.port, 8080);

        // Test invalid address
        let result = get_peer_info("invalid").await;
        assert!(result.is_err());
    }
}
