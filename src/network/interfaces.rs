use anyhow::Result;
use std::net::Ipv4Addr;

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
