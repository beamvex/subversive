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
    use crate::dns::reverse_lookup;
    use subversive_utils::test_utils::init_test_tracing;
    use test_log::test;
    use tracing::info;

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

    #[test(tokio::test)]
    async fn test_reverse_lookup_invalid_ip() {
        info!("test_reverse_lookup_invalid_ip");
        let result = reverse_lookup("invalid_ip").await;
        assert!(result.is_err());
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_localhost() {
        info!("test_reverse_lookup_localhost");
        let result = reverse_lookup("127.0.0.1").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("localhost"));
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_ipv6_localhost() {
        info!("test_reverse_lookup_ipv6_localhost");
        let result = reverse_lookup("::1").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("localhost"));
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_unresolvable() {
        info!("test_reverse_lookup_unresolvable");
        let result = reverse_lookup("192.0.2.1").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "192.0.2.1");
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_empty_string() {
        info!("test_reverse_lookup_empty_string");
        let result = reverse_lookup("").await;
        assert!(result.is_err());
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_ipv6_unspecified() {
        init_test_tracing();
        info!("test_reverse_lookup_ipv6_unspecified");
        let result = reverse_lookup("::").await;
        assert!(result.is_ok());
        let hostname = result.unwrap();
        assert!(
            hostname == "unspecified",
            "Expected :: to be resolved to 'unspecified', got: {}",
            hostname
        );
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_private_network() {
        init_test_tracing();
        info!("test_reverse_lookup_private_network");
        let test_ip = "192.168.1.1";
        let result = reverse_lookup(test_ip).await;
        assert!(result.is_ok());
        let hostname = result.unwrap();
        assert!(
            hostname == test_ip || !hostname.contains("."),
            "Expected either IP or local hostname, got: {}",
            hostname
        );
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_link_local() {
        init_test_tracing();
        info!("test_reverse_lookup_link_local");
        let ipv4_ll = "169.254.1.1";
        let ipv6_ll = "fe80::1";

        let ipv4_result = reverse_lookup(ipv4_ll).await;
        assert!(ipv4_result.is_ok());
        let ipv4_hostname = ipv4_result.unwrap();
        assert!(
            ipv4_hostname == ipv4_ll,
            "Expected link-local IPv4 to return IP, got: {}",
            ipv4_hostname
        );

        let ipv6_result = reverse_lookup(ipv6_ll).await;
        assert!(ipv6_result.is_ok());
        let ipv6_hostname = ipv6_result.unwrap();
        assert!(
            ipv6_hostname == ipv6_ll,
            "Expected link-local IPv6 to return IP, got: {}",
            ipv6_hostname
        );
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_special_purpose() {
        init_test_tracing();
        info!("test_reverse_lookup_special_purpose");
        let cases = vec![
            ("0.0.0.0", Some("unspecified")),       // Unspecified IPv4
            ("255.255.255.255", Some("broadcast")), // Broadcast IPv4
            ("224.0.0.1", Some("multicast")),       // IPv4 multicast all hosts
            ("ff02::1", Some("multicast")),         // IPv6 multicast all nodes
        ];

        for (ip, expected_hostname) in cases {
            let result = reverse_lookup(ip).await;
            assert!(result.is_ok(), "Failed to handle special IP: {}", ip);
            let hostname = result.unwrap();
            match expected_hostname {
                Some(expected) => assert_eq!(
                    hostname, expected,
                    "Expected special IP {} to resolve to {}, got: {}",
                    ip, expected, hostname
                ),
                None => assert_eq!(
                    hostname, ip,
                    "Expected special IP {} to return itself, got: {}",
                    ip, hostname
                ),
            }
        }
    }
}
