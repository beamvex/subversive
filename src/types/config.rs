use anyhow::Result;
use clap::Parser;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::{debug, error, info};

use crate::{logutils::update_tracing, network, types::args::Args};

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
                let mut rng = thread_rng();
                rng.gen_range(10000..=65535)
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
            port: args.port.or(self.port),
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

    /// Load configuration from command line arguments and optional config file
    ///
    /// If a config file is specified in the arguments, it will be loaded and merged
    /// with the command line arguments. Command line arguments take precedence.
    pub async fn load() -> Self {
        // Parse command line arguments
        let args = Args::parse();

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
