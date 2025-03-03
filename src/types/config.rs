use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

/// Configuration for the P2P network application
#[derive(Debug, Serialize, Deserialize)]
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
        }
    }

    /// Merge with command line arguments, preferring argument values over config file values
    pub fn merge_with_args(&self, args: &crate::types::args::Args) -> Self {
        Self {
            port: args.port.or(self.port),
            peer: args.peer.clone().or(self.peer.clone()),
            database: Some(args.database.clone()),
            name: Some(args.name.clone()),
            noip_hostname: args.noip_hostname.clone().or(self.noip_hostname.clone()),
            noip_username: args.noip_username.clone().or(self.noip_username.clone()),
            noip_password: args.noip_password.clone().or(self.noip_password.clone()),
            opendns_hostname: args.opendns_hostname.clone().or(self.opendns_hostname.clone()),
            opendns_username: args.opendns_username.clone().or(self.opendns_username.clone()),
            opendns_password: args.opendns_password.clone().or(self.opendns_password.clone()),
            opendns_network: args.opendns_network.clone().or(self.opendns_network.clone()),
        }
    }
}
