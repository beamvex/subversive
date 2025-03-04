use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};

const UPDATE_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

#[derive(Debug, Clone)]
pub enum DdnsProvider {
    NoIp {
        hostname: String,
        username: String,
        password: String,
    },
    OpenDns {
        hostname: String,
        username: String,
        password: String,
        network: String,
    },
}

impl DdnsProvider {
    async fn update_dns(&self, client: &Client) -> Result<String> {
        match self {
            DdnsProvider::NoIp {
                hostname,
                username,
                password,
            } => {
                let auth = format!("{}:{}", username, password);
                let auth_header = format!("Basic {}", BASE64.encode(auth));

                let response = client
                    .get("https://dynupdate.no-ip.com/nic/update")
                    .header("Authorization", auth_header)
                    .query(&[("hostname", hostname)])
                    .send()
                    .await?
                    .error_for_status()?
                    .text()
                    .await?;

                Ok(response)
            }
            DdnsProvider::OpenDns {
                hostname,
                username,
                password,
                network,
            } => {
                let auth = format!("{}:{}", username, password);
                let auth_header = format!("Basic {}", BASE64.encode(auth));

                let response = client
                    .get(&format!(
                        "https://updates.opendns.com/nic/update?hostname={}&network={}",
                        hostname, network
                    ))
                    .header("Authorization", auth_header)
                    .send()
                    .await?
                    .error_for_status()?
                    .text()
                    .await?;

                Ok(response)
            }
        }
    }

    fn get_provider_name(&self) -> &'static str {
        match self {
            DdnsProvider::NoIp { .. } => "No-IP",
            DdnsProvider::OpenDns { .. } => "OpenDNS",
        }
    }
}

/// Start a background task that periodically updates Dynamic DNS records
pub async fn start_ddns_updater(provider: DdnsProvider, client: Client) -> Result<()> {
    let provider_name = provider.get_provider_name();
    info!("Starting {} DNS updater", provider_name);

    tokio::spawn(async move {
        loop {
            match provider.update_dns(&client).await {
                Ok(response) => {
                    info!("{} DNS update successful: {}", provider_name, response);
                }
                Err(e) => {
                    warn!("Failed to update {} DNS: {}", provider_name, e);
                }
            }
            time::sleep(UPDATE_INTERVAL).await;
        }
    });

    Ok(())
}
