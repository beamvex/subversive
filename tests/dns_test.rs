#[cfg(test)]
mod tests {
    use subversive::network::dns::reverse_lookup;
    use test_log::test;
    use tracing::info;

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
        info!("test_reverse_lookup_ipv6_unspecified");
        let result = reverse_lookup("::").await;
        assert!(result.is_ok());
        let hostname = result.unwrap();
        assert!(
            hostname == "::",
            "Expected :: to be returned, got: {}",
            hostname
        );
    }

    #[test(tokio::test)]
    async fn test_reverse_lookup_private_network() {
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
        info!("test_reverse_lookup_special_purpose");
        let cases = vec![
            ("0.0.0.0", None),                            // Unspecified IPv4
            ("255.255.255.255", None),                    // Broadcast IPv4
            ("224.0.0.1", Some("all-systems.mcast.net")), // IPv4 multicast all hosts
            ("ff02::1", Some("ip6-allnodes")),            // IPv6 multicast all nodes
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
