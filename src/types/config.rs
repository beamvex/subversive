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
        }
    }

    /// Merge with command line arguments, preferring argument values over config file values
    pub fn merge_with_args(&self, args: &crate::types::args::Args) -> Self {
        Self {
            port: args.port.or(self.port),
            peer: args.peer.clone().or(self.peer.clone()),
            database: Some(args.database.clone()),
            name: Some(args.name.clone()),
        }
    }
}
