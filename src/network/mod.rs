pub mod discovery;
pub mod peer;
pub mod dns;
mod health;
mod interfaces;
mod upnp;

#[cfg(test)]
mod discovery_test;
#[cfg(test)]
mod peer_test;
#[cfg(test)]
mod dns_test;

pub use discovery::get_external_ip;
use dns::reverse_lookup;
pub use health::start_health_checker;
pub use interfaces::get_network_interfaces;
pub use peer::{broadcast_to_peers, connect_to_initial_peer};
pub use upnp::{cleanup_upnp, setup_upnp};

use anyhow::Result;
use igd::aio::Gateway;
use tracing::info;

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
/// * The full address string in the format "https://<hostname>:<actual_port>"
pub async fn setup_network(port: u16, config: &Config) -> Result<(u16, Vec<Gateway>, String)> {
    // Get external IP and resolve hostname
    let host = config.get_hostname().unwrap_or_default();
    let own_address = format!("https://{}:{}", host, port);
    info!("Server listening on internet endpoint: {}", own_address);

    // Set up UPnP port mapping
    let (actual_port, gateways) = upnp::setup_upnp(port).await?;
    info!("Using port {}", actual_port);

    // After UPnP setup
    let own_address = format!("https://{}:{}", host, actual_port);
    info!("Own address: {}", own_address);

    Ok((actual_port, gateways, own_address))
}

pub async fn get_hostname() -> Result<String> {
    reverse_lookup(&get_external_ip().await?).await
}
