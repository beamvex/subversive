pub mod discovery;
pub mod dns;
pub mod health;
pub mod interfaces;
pub mod local_ip;
pub mod peer;
pub mod upnp;

#[cfg(test)]
pub mod discovery_test;
#[cfg(test)]
pub mod dns_test;
#[cfg(test)]
pub mod interfaces_test;
#[cfg(test)]
pub mod upnp_test;

use discovery::get_external_ip;
use dns::reverse_lookup;

use crate::network::upnp::Gateway2;

pub use peer::broadcast_to_peers;
pub use upnp::cleanup_upnp;

use anyhow::Result;

use crate::types::config::Config;

/// Set up network connectivity including external IP discovery and UPnP port mapping
///
/// # Arguments
/// * `port` - The port to map
/// * `config` - Application configuration
///
/// # Returns
/// A tuple containing:
/// * The actual port being used (may be different from requested if UPnP mapping fails)
/// * The list of discovered UPnP gateways
/// * The full address string in the format "<hostname>:<actual_port>"
pub async fn setup_network(port: u16, config: &Config) -> Result<(u16, Vec<Gateway2>, String)> {
    // Get external IP and resolve hostname
    let host = config.get_hostname().unwrap_or_default();
    let own_address = format!("{}:{}", host, port);

    let (actual_port, gateways) = upnp::setup_upnp(port).await?;

    Ok((actual_port, gateways, own_address))
}

pub async fn get_hostname() -> Result<String> {
    reverse_lookup(&get_external_ip().await?).await
}

pub async fn cleanup_network(port: u16, gateways: Vec<Gateway2>) -> Result<()> {
    upnp::cleanup_upnp(port, gateways).await
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::sync::Arc;

    use crate::network::upnp::{Gateway2, MockIGateway};
    use crate::test_utils::init_test_tracing;
    use crate::types::config::Config;

    use super::*;

    #[tokio::test]
    async fn test_setup_network_with_hostname() -> Result<()> {
        init_test_tracing();

        // Create a config with a hostname
        let mut config = Config::default_config();
        config.hostname = Some("test.example.com".to_string());

        // Mock the UPnP setup to return a specific port and gateways
        let port = 12345;
        let expected_port = 12345;
        let mock_gateway = MockIGateway::new();
        let expected_gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];

        // Call setup_network
        let (actual_port, gateways, address) = setup_network(port, &config).await?;

        // Verify results
        assert_eq!(actual_port, expected_port);
        assert_eq!(gateways.len(), expected_gateways.len());
        assert_eq!(address, "test.example.com:12345");

        Ok(())
    }

    #[tokio::test]
    async fn test_setup_network_without_hostname() -> Result<()> {
        init_test_tracing();

        // Create a config without a hostname
        let config = Config::default_config();

        // Call setup_network
        let port = 12345;
        let (actual_port, _gateways, address) = setup_network(port, &config).await?;

        // Verify results
        assert_eq!(actual_port, port);
        assert_eq!(address, format!(":{}", port));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_hostname() -> Result<()> {
        init_test_tracing();

        // Mock external IP to be a known value
        let hostname = get_hostname().await?;

        // Verify the hostname is not empty and is a valid domain or IP
        assert!(!hostname.is_empty());
        assert!(hostname.parse::<IpAddr>().is_ok() || hostname.contains('.'));

        Ok(())
    }

    #[tokio::test]
    async fn test_cleanup_network() -> Result<()> {
        init_test_tracing();

        // Create test gateways
        let mut mock_gateway = MockIGateway::new();
        mock_gateway.expect_remove_port().returning(|_, _| Ok(()));
        let expected_gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];

        // Call cleanup_network
        let result = cleanup_network(12345, expected_gateways).await;

        // Verify cleanup was successful
        assert!(result.is_ok());

        Ok(())
    }
}
