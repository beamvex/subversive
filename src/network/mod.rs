pub mod discovery;
pub mod dns;
pub mod health;
pub mod interfaces;
pub mod peer;
pub mod upnp;

#[cfg(test)]
pub mod discovery_test;
#[cfg(test)]
pub mod dns_test;
#[cfg(test)]
pub mod health_test;
#[cfg(test)]
pub mod interfaces_test;
#[cfg(test)]
pub mod upnp_test;

use discovery::get_external_ip;
use dns::reverse_lookup;

pub use peer::broadcast_to_peers;
pub use upnp::cleanup_upnp;

#[cfg(test)]
use crate::network::upnp_test::MockTestGateway;
use anyhow::Result;

use crate::network::upnp::GatewayInterface;
use crate::types::config::Config;

use tracing::info;

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
pub async fn setup_network(
    port: u16,
    config: &Config,
) -> Result<(u16, Vec<Box<dyn GatewayInterface>>, String)> {
    // Get external IP and resolve hostname
    let host = config.get_hostname().unwrap_or_default();
    let own_address = format!("{}:{}", host, port);
    info!("Server listening on internet endpoint: {}", own_address);

    // Set up UPnP port mapping
    #[cfg(test)]
    let gateway = Box::new(MockTestGateway::new()) as Box<dyn GatewayInterface>;
    let (actual_port, gateways) = upnp::setup_upnp(
        port,
        #[cfg(test)]
        gateway,
    )
    .await?;
    info!("Using port {}", actual_port);

    // After UPnP setup
    Ok((actual_port, gateways, own_address))
}

pub async fn get_hostname() -> Result<String> {
    reverse_lookup(&get_external_ip().await?).await
}
