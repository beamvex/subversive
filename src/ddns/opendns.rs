use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use tracing::info;

use super::{DdnsProvider, DdnsProviderConfig};

#[derive(Debug, Clone)]
pub struct OpenDnsProvider {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub network: String,
}

impl OpenDnsProvider {
    pub async fn update_dns(&self, client: &Client) -> Result<String> {
        let auth = format!("{}:{}", self.username, self.password);
        let auth_header = format!("Basic {}", BASE64.encode(auth));

        let response = client
            .get(&format!(
                "https://updates.opendns.com/nic/update?hostname={}&network={}",
                self.hostname, self.network
            ))
            .header("Authorization", auth_header)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(response)
    }

    pub fn new(hostname: String, username: String, password: String, network: String) -> Self {
        Self {
            hostname,
            username,
            password,
            network,
        }
    }
}

impl DdnsProviderConfig for OpenDnsProvider {
    fn try_from_config(config: &crate::Config) -> Option<DdnsProvider> {
        match (
            config.opendns_hostname.clone(),
            config.opendns_username.clone(),
            config.opendns_password.clone(),
            config.opendns_network.clone(),
        ) {
            (Some(hostname), Some(username), Some(password), Some(network)) => {
                info!("Starting OpenDNS updater for hostname: {}", hostname);
                Some(DdnsProvider::OpenDns(Self::new(
                    hostname, username, password, network,
                )))
            }
            _ => None,
        }
    }
}
