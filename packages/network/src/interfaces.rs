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

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use std::net::Ipv4Addr;
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;
    use std::sync::Once;
    use std::{env, fs};

    use anyhow::Result;
    use once_cell::sync::Lazy;
    use subversive_utils::test_utils::init_test_tracing;
    use tokio::sync::Mutex;
    use tracing::info;

    use crate::interfaces::get_network_interfaces;

    static MOCK_SETUP: Once = Once::new();
    const MOCK_IP_COMMAND: &str = "ip";
    static PATH_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn setup_mock_ip_command() -> Result<()> {
        MOCK_SETUP.call_once(|| {
            // Create a temporary mock ip command that outputs test data
            let mock_script = r#"#!/bin/sh
if [ "$1" = "addr" ] && [ "$2" = "show" ]; then
cat << 'EOF'
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
    inet 127.0.0.1/8 scope host lo
       valid_lft forever preferred_lft forever
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP group default qlen 1000
    link/ether 00:15:5d:01:ca:05 brd ff:ff:ff:ff:ff:ff
    inet 192.168.1.100/24 brd 192.168.1.255 scope global eth0
       valid_lft forever preferred_lft forever
3: wlan0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000
    link/ether 00:15:5d:01:ca:06 brd ff:ff:ff:ff:ff:ff
    inet 192.168.2.50/24 brd 192.168.2.255 scope global dynamic wlan0
       valid_lft 86389sec preferred_lft 86389sec
EOF
fi"#;

            let temp_dir = env::temp_dir();
            let mock_path = temp_dir.join(MOCK_IP_COMMAND);
            fs::write(&mock_path, mock_script).expect("Failed to write mock script");
            fs::set_permissions(&mock_path, fs::Permissions::from_mode(0o755))
                .expect("Failed to set mock script permissions");

            // Add temp directory to PATH for this test
            let old_path = env::var("PATH").unwrap_or_default();
            env::set_var(
                "PATH",
                format!("{}:{}", temp_dir.to_string_lossy(), old_path),
            );
        });
        Ok(())
    }

    #[test]
    fn test_get_network_interfaces_success() -> Result<()> {
        setup_mock_ip_command()?;

        let interfaces = get_network_interfaces()?;

        assert_eq!(interfaces.len(), 3);
        assert!(interfaces.contains(&"127.0.0.1".parse()?));
        assert!(interfaces.contains(&"192.168.1.100".parse()?));
        assert!(interfaces.contains(&"192.168.2.50".parse()?));

        Ok(())
    }

    #[tokio::test]
    async fn test_get_network_interfaces_no_interfaces() -> Result<()> {
        let _lock = PATH_MUTEX.lock().await;
        // Use a unique temp dir for the mock
        let temp_dir = tempfile::TempDir::new()?;
        let mock_path = temp_dir.path().join("ip");
        let mock_script = r#"#!/bin/sh
if [ "$1" = "addr" ] && [ "$2" = "show" ]; then
    echo ""
fi"#;
        fs::write(&mock_path, mock_script)?;
        fs::set_permissions(&mock_path, fs::Permissions::from_mode(0o755))?;

        // Temporarily override PATH
        let old_path = env::var("PATH").unwrap_or_default();
        env::set_var(
            "PATH",
            format!("{}:{}", temp_dir.path().to_string_lossy(), old_path),
        );

        let interfaces = get_network_interfaces()?;

        // Restore PATH
        env::set_var("PATH", old_path);
        // temp_dir is dropped here, cleaning up the mock

        assert!(interfaces.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_network_interfaces_invalid_output() -> Result<()> {
        let _lock = PATH_MUTEX.lock().await;
        // Use a unique temp dir for the mock
        let temp_dir = tempfile::TempDir::new()?;
        let mock_path = temp_dir.path().join("ip");
        let mock_script = r#"#!/bin/sh
if [ "$1" = "addr" ] && [ "$2" = "show" ]; then
    echo "invalid data format"
fi"#;
        fs::write(&mock_path, mock_script)?;
        fs::set_permissions(&mock_path, fs::Permissions::from_mode(0o755))?;

        // Temporarily override PATH
        let old_path = env::var("PATH").unwrap_or_default();
        env::set_var(
            "PATH",
            format!("{}:{}", temp_dir.path().to_string_lossy(), old_path),
        );

        let interfaces = get_network_interfaces()?;

        // Restore PATH
        env::set_var("PATH", old_path);
        // temp_dir is dropped here, cleaning up the mock

        assert!(interfaces.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_network_interfaces_command_not_found() {
        init_test_tracing();
        let _lock = PATH_MUTEX.lock().await;
        // Use a unique temp dir for the mock (but don't create an ip mock)
        let temp_dir = tempfile::TempDir::new().unwrap();
        let old_path = env::var("PATH").unwrap_or_default();
        info!("Old PATH: {}", old_path);
        // Set PATH to ONLY the empty temp dir
        env::set_var("PATH", temp_dir.path());
        info!("New PATH: {}", env::var("PATH").unwrap_or_default());

        let result = get_network_interfaces();

        // Restore PATH
        env::set_var("PATH", old_path);
        // temp_dir is dropped here

        info!("Result: {:?}", result);

        assert!(result.is_err());
    }
}
