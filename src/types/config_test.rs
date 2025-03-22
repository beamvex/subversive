use super::config::Config;
use crate::types::args::Args;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_default_config() {
    let config = Config::default();

    // Check default values
    assert!(config.port.is_some());
    assert!(config.port.unwrap() >= 10000, "Port should be >= 10000");
    assert_eq!(config.peer, None);
    assert_eq!(config.database, Some("p2p_network.db".to_string()));
    assert_eq!(config.name, Some("p2p_network".to_string()));
    assert_eq!(config.hostname, None);
    assert_eq!(config.noip_hostname, None);
    assert_eq!(config.noip_username, None);
    assert_eq!(config.noip_password, None);
    assert_eq!(config.opendns_hostname, None);
    assert_eq!(config.opendns_username, None);
    assert_eq!(config.opendns_password, None);
    assert_eq!(config.opendns_network, None);
    assert_eq!(config.survival_mode, None);
    assert_eq!(config.log_level, Some("info".to_string()));
}

#[test]
fn test_from_file() -> anyhow::Result<()> {
    // Create a temporary config file
    let config_file = NamedTempFile::new()?;
    let config_content = r#"
        port: 12345
        peer: "https://peer1:8080"
        database: "test.db"
        name: "test_node"
        hostname: "test.example.com"
        log_level: "debug"
    "#;
    fs::write(config_file.path(), config_content)?;

    // Load the config
    let config = Config::from_file(config_file.path().to_str().unwrap())?;

    // Verify loaded values
    assert_eq!(config.port, Some(12345));
    assert_eq!(config.peer, Some("https://peer1:8080".to_string()));
    assert_eq!(config.database, Some("test.db".to_string()));
    assert_eq!(config.name, Some("test_node".to_string()));
    assert_eq!(config.hostname, Some("test.example.com".to_string()));
    assert_eq!(config.log_level, Some("debug".to_string()));

    Ok(())
}

#[test]
fn test_merge_with_args() {
    let config = Config {
        port: Some(8080),
        peer: Some("https://peer1:8080".to_string()),
        database: Some("config.db".to_string()),
        name: Some("config_node".to_string()),
        hostname: Some("config.example.com".to_string()),
        noip_hostname: None,
        noip_username: None,
        noip_password: None,
        opendns_hostname: None,
        opendns_username: None,
        opendns_password: None,
        opendns_network: None,
        survival_mode: None,
        log_level: Some("info".to_string()),
    };

    // Create args that override some values
    let args = Args {
        port: Some(9090),
        peer: Some("https://peer2:8080".to_string()),
        database: None, // Keep config value
        name: Some("arg_node".to_string()),
        hostname: None, // Keep config value
        config: None,
        noip_hostname: None,
        noip_username: None,
        noip_password: None,
        opendns_hostname: None,
        opendns_username: None,
        opendns_password: None,
        opendns_network: None,
        survival_mode: Some(true),
        log_level: Some("debug".to_string()),
    };

    let merged = config.merge_with_args(&args);

    // Check that args override config values when present
    assert_eq!(merged.port, Some(9090));
    assert_eq!(merged.peer, Some("https://peer2:8080".to_string()));
    assert_eq!(merged.database, Some("config.db".to_string())); // Kept from config
    assert_eq!(merged.name, Some("arg_node".to_string()));
    assert_eq!(merged.hostname, Some("config.example.com".to_string())); // Kept from config
    assert_eq!(merged.survival_mode, Some(true));
    assert_eq!(merged.log_level, Some("debug".to_string()));
}

#[test]
fn test_getters() {
    let mut config = Config::default();
    config.port = Some(8080);
    config.database = Some("test.db".to_string());
    config.name = Some("test_node".to_string());
    config.hostname = Some("test.example.com".to_string());
    config.log_level = Some("debug".to_string());

    assert_eq!(config.get_port(), 8080);
    assert_eq!(config.get_database(), "test.db");
    assert_eq!(config.get_name(), "test_node");
    assert_eq!(config.get_hostname(), Some("test.example.com".to_string()));
    assert_eq!(config.get_log_level(), "debug");

    // Test defaults when values are None
    let mut empty_config = Config::default();
    empty_config.port = None;
    empty_config.database = None;
    empty_config.name = None;
    empty_config.hostname = None;
    empty_config.log_level = None;

    assert_eq!(empty_config.get_port(), 0); // Default from u16
    assert_eq!(empty_config.get_database(), "");
    assert_eq!(empty_config.get_name(), "");
    assert_eq!(empty_config.get_hostname(), None);
    assert_eq!(empty_config.get_log_level(), "info");
}

#[test]
fn test_update_log_level() {
    let mut config = Config::default();
    assert_eq!(config.get_log_level(), "info");

    config.update_log_level("debug".to_string());
    assert_eq!(config.get_log_level(), "debug");

    config.update_log_level("trace".to_string());
    assert_eq!(config.get_log_level(), "trace");
}
