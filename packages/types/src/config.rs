use anyhow::Result;
#[cfg(not(test))]
use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use subversive_utils::logutils::update_tracing;
use tokio::fs;
use tracing::{debug, error, info};

use crate::args::Args;
use crate::network;

/// Configuration for the P2P network application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Port to listen on for P2P communication
    pub port: Option<u16>,
    /// Initial peer to connect to
    pub peer: Option<String>,
    /// Database file name
    pub database: Option<String>,
    /// Custom name for HTTP access logs
    pub name: Option<String>,
    /// Hostname to use for the server (defaults to external IP)
    pub hostname: Option<String>,
    /// No-IP hostname (e.g., example.ddns.net)
    pub noip_hostname: Option<String>,
    /// No-IP username
    pub noip_username: Option<String>,
    /// No-IP password
    pub noip_password: Option<String>,
    /// OpenDNS hostname
    pub opendns_hostname: Option<String>,
    /// OpenDNS username
    pub opendns_username: Option<String>,
    /// OpenDNS password
    pub opendns_password: Option<String>,
    /// OpenDNS network label
    pub opendns_network: Option<String>,
    /// Enable post-apocalyptic survival mode
    pub survival_mode: Option<bool>,
    /// Log level (trace, debug, info, warn, error)
    pub log_level: Option<String>,
}

impl Config {
    /// Load configuration from a YAML file
    pub async fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path).await?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Create default configuration
    pub fn default_config() -> Self {
        Self {
            port: Some({
                let mut rng = rand::rng();
                rng.random_range(10000..=65535)
            }),
            peer: None,
            database: Some("p2p_network.db".to_string()),
            name: Some("p2p_network".to_string()),
            hostname: None,
            noip_hostname: None,
            noip_username: None,
            noip_password: None,
            opendns_hostname: None,
            opendns_username: None,
            opendns_password: None,
            opendns_network: None,
            survival_mode: None,
            log_level: Some("info".to_string()),
        }
    }

    /// Set default hostname
    pub async fn default_hostname(config: &mut Self) {
        // Only set dynamic DNS hostnames if no static hostname is configured
        if config.hostname.is_none() {
            info!("host name not set: setting default hostname");
            debug!(
                "no ip hostname {}",
                config.noip_hostname.clone().unwrap_or("".to_string())
            );

            debug!(
                "opendns hostname {}",
                config.opendns_hostname.clone().unwrap_or("".to_string())
            );

            if config.noip_hostname.is_some() || config.opendns_hostname.is_some() {
                config.hostname =
                    Some(config.noip_hostname.clone().unwrap_or(
                        config.opendns_hostname.clone().unwrap_or(
                            config.noip_hostname.clone().unwrap_or(
                                config.opendns_hostname.clone().unwrap_or("".to_string()),
                            ),
                        ),
                    ));
            } else {
                config.hostname = Some(network::get_hostname().await.unwrap());
            }
        }
    }

    /// Merge with command line arguments, preferring argument values over config file values
    pub fn merge_with_args(&self, args: &Args) -> Self {
        Self {
            port: match args.port {
                Some(port) => {
                    info!("overriding port: {}", port);
                    Some(port)
                }
                None => {
                    info!(
                        "Port not set, using config or random: {}",
                        self.port.unwrap_or_default()
                    );
                    self.port
                }
            },
            peer: args.peer.clone().or(self.peer.clone()),
            database: args.database.clone().or(self.database.clone()),
            name: args.name.clone().or(self.name.clone()),
            hostname: args.hostname.clone().or(self.hostname.clone()),
            noip_hostname: args.noip_hostname.clone().or(self.noip_hostname.clone()),
            noip_username: args.noip_username.clone().or(self.noip_username.clone()),
            noip_password: args.noip_password.clone().or(self.noip_password.clone()),
            opendns_hostname: args
                .opendns_hostname
                .clone()
                .or(self.opendns_hostname.clone()),
            opendns_username: args
                .opendns_username
                .clone()
                .or(self.opendns_username.clone()),
            opendns_password: args
                .opendns_password
                .clone()
                .or(self.opendns_password.clone()),
            opendns_network: args
                .opendns_network
                .clone()
                .or(self.opendns_network.clone()),
            survival_mode: args.survival_mode,
            log_level: args.log_level.clone().or(self.log_level.clone()),
        }
    }

    /// Load configuration from command line arguments and optional config file for testing
    ///
    /// If a config file is specified in the arguments, it will be loaded and merged
    /// with the command line arguments. Command line arguments take precedence.
    #[cfg(test)]
    pub async fn load_test(args: Args) -> Self {
        Self::load_internal(args).await
    }

    /// Load configuration from command line arguments and optional config file
    ///
    /// If a config file is specified in the arguments, it will be loaded and merged
    /// with the command line arguments. Command line arguments take precedence.
    #[cfg(not(test))]
    pub async fn load() -> Self {
        let args = Args::parse();
        Self::load_internal(args).await
    }

    /// Internal implementation of configuration loading
    async fn load_internal(args: Args) -> Self {
        // Load configuration
        let mut config = if let Some(config_path) = &args.config {
            info!("Loading configuration from {}", config_path);
            Self::from_file(config_path).await.unwrap_or_else(|e| {
                error!("Failed to load config file: {}", e);
                Self::default_config()
            })
        } else {
            Self::default_config()
        };

        Self::default_hostname(&mut config).await;

        // Merge config with command line arguments
        config.merge_with_args(&args)
    }

    // Add getter method
    pub fn get_log_level(&self) -> String {
        self.log_level.clone().unwrap_or_else(|| "info".to_string())
    }

    /// Get the port number, generating a random one if not specified
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or_default()
    }

    /// Get the database name, using default if not specified
    pub fn get_database(&self) -> String {
        self.database.clone().unwrap_or_default()
    }

    /// Get the name, using default if not specified
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or_default()
    }

    /// Get the hostname, using external IP if not specified
    pub fn get_hostname(&self) -> Option<String> {
        self.hostname.clone()
    }

    /// Update the log level
    pub fn update_log_level(&mut self, log_level: String) {
        self.log_level = Some(log_level.clone());
        update_tracing(&log_level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use subversive_utils::test_utils::init_test_tracing;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_default_config() {
        let config = Config::default_config();

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

    #[tokio::test]
    async fn test_from_file() -> anyhow::Result<()> {
        // Create a temporary config file
        let config_file = NamedTempFile::new()?;
        let config_content = r#"
        {
            "port": 12345,
            "peer": "https://peer1:8080",
            "database": "test.db",
            "name": "test_node",
            "hostname": "test.example.com",
            "log_level": "debug"
        }
    "#;
        fs::write(config_file.path(), config_content)?;

        // Load the config
        let config = Config::from_file(config_file.path().to_str().unwrap()).await?;

        // Verify loaded values
        assert_eq!(config.port, Some(12345));
        assert_eq!(config.peer, Some("https://peer1:8080".to_string()));
        assert_eq!(config.database, Some("test.db".to_string()));
        assert_eq!(config.name, Some("test_node".to_string()));
        assert_eq!(config.hostname, Some("test.example.com".to_string()));
        assert_eq!(config.log_level, Some("debug".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_merge_with_args() {
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

    #[tokio::test]
    async fn test_getters() {
        let mut config = Config::default_config();
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
        let mut empty_config = Config::default_config();
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

    #[tokio::test]
    async fn test_update_log_level() {
        let mut config = Config::default_config();
        assert_eq!(config.get_log_level(), "info");

        config.update_log_level("debug".to_string());
        assert_eq!(config.get_log_level(), "debug");

        config.update_log_level("trace".to_string());
        assert_eq!(config.get_log_level(), "trace");
    }

    #[tokio::test]
    async fn test_config_merge() {
        let config = Config::default_config();
        let args = Args {
            port: Some(8081),
            peer: None,
            database: None,
            name: None,
            hostname: None,
            log_level: None,
            config: None,
            survival_mode: None,
            noip_hostname: None,
            noip_username: None,
            noip_password: None,
            opendns_hostname: None,
            opendns_username: None,
            opendns_password: None,
            opendns_network: None,
        };

        let merged = config.merge_with_args(&args);
        assert_eq!(merged.port, Some(8081));
    }

    #[tokio::test]
    async fn test_load_config() {
        init_test_tracing();

        // Create a temporary config file
        let config_file = NamedTempFile::new().unwrap();
        let config_content = r#"
        {
            "port": 12345,
            "database": "config.db",
            "name": "config_node",
            "log_level": "debug"
        }
        "#;
        let _ = fs::write(config_file.path(), config_content);

        let inject_args = Args {
            port: Some(54321),
            peer: None,
            database: None,
            name: Some("cli_node".to_string()),
            hostname: None,
            log_level: None,
            config: Some(config_file.path().to_str().unwrap().to_string()),
            survival_mode: None,
            noip_hostname: None,
            noip_username: None,
            noip_password: None,
            opendns_hostname: None,
            opendns_username: None,
            opendns_password: None,
            opendns_network: None,
        };

        // Load the configuration
        let config = Config::load(inject_args).await;

        // Verify that CLI args override config file values
        assert_eq!(config.port, Some(54321)); // From CLI
        assert_eq!(config.name, Some("cli_node".to_string())); // From CLI
        assert_eq!(config.database, Some("config.db".to_string())); // From config file
        assert_eq!(config.log_level, Some("debug".to_string())); // From config file
    }

    #[tokio::test]
    async fn test_load_config_no_file() {
        init_test_tracing();

        let inject_args = Args {
            port: Some(8080),
            peer: None,
            database: None,
            name: Some("test_node".to_string()),
            hostname: None,
            log_level: None,
            config: None,
            survival_mode: None,
            noip_hostname: None,
            noip_username: None,
            noip_password: None,
            opendns_hostname: None,
            opendns_username: None,
            opendns_password: None,
            opendns_network: None,
        };

        // Load the configuration
        let config = Config::load(inject_args).await;

        // Verify that values come from CLI and defaults
        assert_eq!(config.port, Some(8080)); // From CLI
        assert_eq!(config.name, Some("test_node".to_string())); // From CLI
        assert_eq!(config.database, Some("p2p_network.db".to_string())); // Default value
        assert_eq!(config.log_level, Some("info".to_string())); // Default value
    }

    #[tokio::test]
    async fn test_load_config_missing_file() {
        init_test_tracing();

        let inject_args = Args {
            port: Some(8080),
            peer: None,
            database: None,
            name: Some("test_node".to_string()),
            hostname: None,
            log_level: None,
            config: Some("/path/to/missing/config.json".to_string()),
            survival_mode: None,
            noip_hostname: None,
            noip_username: None,
            noip_password: None,
            opendns_hostname: None,
            opendns_username: None,
            opendns_password: None,
            opendns_network: None,
        };

        // Load the configuration
        let config = Config::load(inject_args).await;

        // Verify that values come from CLI and defaults
        assert_eq!(config.port, Some(8080)); // From CLI
        assert_eq!(config.name, Some("test_node".to_string())); // From CLI
        assert_eq!(config.database, Some("p2p_network.db".to_string())); // Default value
        assert_eq!(config.log_level, Some("info".to_string())); // Default value
    }
}
