use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use tracing::info;

use crate::{DdnsProvider, DdnsProviderConfig, Config};

#[derive(Debug, Clone)]
pub struct OpenDnsProvider {
    pub hostname: String,
    pub username: String,
    pub password: String,
    pub network: String,
    #[cfg(test)]
    pub base_url: String,
}

impl OpenDnsProvider {
    pub async fn update_dns(&self, client: &Client) -> Result<String> {
        let auth = format!("{}:{}", self.username, self.password);
        let auth_header = format!("Basic {}", BASE64.encode(auth));

        #[cfg(not(test))]
        let url = format!(
            "https://updates.opendns.com/nic/update?hostname={}&network={}",
            self.hostname, self.network
        );
        #[cfg(test)]
        let url = format!(
            "{}/nic/update?hostname={}&network={}",
            self.base_url, self.hostname, self.network
        );

        let response = client
            .get(url)
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
            #[cfg(test)]
            base_url: String::new(),
        }
    }
}

impl DdnsProviderConfig for OpenDnsProvider {
    fn try_from_config(config: &Config) -> Option<DdnsProvider> {
        match (
            &config.opendns_hostname,
            &config.opendns_username,
            &config.opendns_password,
            &config.opendns_network,
        ) {
            (Some(hostname), Some(username), Some(password), Some(network)) => {
                Some(DdnsProvider::OpenDns(OpenDnsProvider::new(
                    hostname.clone(),
                    username.clone(),
                    password.clone(),
                    network.clone(),
                )))
            }
            _ => None,
        }
    }
}
