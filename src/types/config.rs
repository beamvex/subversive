use anyhow::Result;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

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
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Create default configuration
    pub fn default() -> Self {
        Self {
            port: None,
            peer: None,
            database: Some("p2p_network.db".to_string()),
            name: Some("p2p_network".to_string()),
            noip_hostname: None,
            noip_username: None,
            noip_password: None,
            opendns_hostname: None,
            opendns_username: None,
            opendns_password: None,
            opendns_network: None,
            survival_mode: None,
        }
    }

    /// Merge with command line arguments, preferring argument values over config file values
    pub fn merge_with_args(&self, args: &crate::types::args::Args) -> Self {
        Self {
            port: args.port.or(self.port),
            peer: args.peer.clone().or(self.peer.clone()),
            database: args.database.clone().or(self.database.clone()),
            name: args.name.clone().or(self.name.clone()),
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
            survival_mode: args.survival_mode.clone(),
        }
    }

    /// Get the port number, generating a random one if not specified
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            rng.gen_range(10000..=65535)
        })
    }

    /// Get the database name, using default if not specified
    pub fn get_database(&self) -> String {
        self.database
            .clone()
            .unwrap_or_else(|| "p2p_network.db".to_string())
    }

    /// Get the name, using default if not specified
    pub fn get_name(&self) -> String {
        self.name
            .clone()
            .unwrap_or_else(|| "p2p_network".to_string())
    }
}
