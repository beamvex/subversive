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
