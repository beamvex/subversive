use anyhow::Result;
use igd::aio::Gateway;
use std::net::Ipv4Addr;
use tracing::info;

use crate::upnp;

/// Get the external IP address of the machine
///
/// # Returns
/// The external IP address as a string
pub async fn get_external_ip() -> Result<String> {
    let response = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(response)
}

/// Get the network interfaces of the machine
///
/// # Returns
/// A vector of IPv4 addresses of the network interfaces
pub fn get_network_interfaces() -> Result<Vec<Ipv4Addr>> {
    let output = std::process::Command::new("ip")
        .args(["addr", "show"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut addresses = Vec::new();

    for line in stdout.lines() {
        if line.contains("inet ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(addr_str) = parts.get(1) {
                if let Some(addr_str) = addr_str.split('/').next() {
                    if let Ok(addr) = addr_str.parse() {
                        addresses.push(addr);
                    }
                }
            }
        }
    }

    Ok(addresses)
}

/// Set up network connectivity including external IP discovery and UPnP port mapping
///
/// # Arguments
/// * `port` - The port to map
///
/// # Returns
/// A tuple containing:
/// * The actual port being used (may be different from requested if UPnP mapping fails)
/// * The list of discovered UPnP gateways
/// * The full address string in the format "https://<external_ip>:<actual_port>"
pub async fn setup_network(port: u16) -> Result<(u16, Vec<Gateway>, String)> {
    // Get external IP and log the full endpoint address
    let external_ip = get_external_ip().await?;
    let own_address = format!("https://{}:{}", external_ip, port);
    info!("Server listening on internet endpoint: {}", own_address);

    // Set up UPnP port mapping
    let (actual_port, gateways) = upnp::setup_upnp(port).await?;
    info!("Using port {}", actual_port);

    // After UPnP setup
    let own_address = format!("https://{}:{}", external_ip, actual_port);
    info!("Own address: {}", own_address);

    Ok((actual_port, gateways, own_address))
}
