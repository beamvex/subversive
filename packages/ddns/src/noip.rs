use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use tracing::info;

use crate::{DdnsProvider, DdnsProviderConfig, Config};

#[derive(Debug, Clone)]
pub struct NoIpProvider {
    pub hostname: String,
    pub username: String,
    pub password: String,
    #[cfg(test)]
    pub base_url: String,
}

impl NoIpProvider {
    pub async fn update_dns(&self, client: &Client) -> Result<String> {
        let auth = format!("{}:{}", self.username, self.password);
        let auth_header = format!("Basic {}", BASE64.encode(auth));

        #[cfg(not(test))]
        let url = "https://dynupdate.no-ip.com/nic/update";
        #[cfg(test)]
        let url = &format!("{}/nic/update", self.base_url);

        let response = client
            .get(url)
            .header("Authorization", auth_header)
            .query(&[("hostname", &self.hostname)])
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        Ok(response)
    }

    pub fn new(hostname: String, username: String, password: String) -> Self {
        Self {
            hostname,
            username,
            password,
            #[cfg(test)]
            base_url: String::new(),
        }
    }
}

impl DdnsProviderConfig for NoIpProvider {
    fn try_from_config(config: &Config) -> Option<DdnsProvider> {
        match (
            &config.noip_hostname,
            &config.noip_username,
            &config.noip_password,
        ) {
            (Some(hostname), Some(username), Some(password)) => Some(DdnsProvider::NoIp(Self::new(
                hostname.clone(),
                username.clone(),
                password.clone(),
            ))),
            _ => None,
        }
    }
}
