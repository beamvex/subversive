use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};

mod noip;
mod opendns;
mod provider;

#[cfg(test)]
mod noip_test;
#[cfg(test)]
mod opendns_test;
#[cfg(test)]
mod provider_test;

pub use noip::NoIpProvider;
pub use opendns::OpenDnsProvider;
pub use provider::{DdnsProvider, DdnsProviderConfig};

const UPDATE_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes

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

/// Configure DDNS if settings are present
pub async fn config_ddns(config: &crate::types::config::Config) {
    info!("Configuring DDNS");
    let client = reqwest::Client::new();
    let providers = [
        NoIpProvider::try_from_config(config),
        OpenDnsProvider::try_from_config(config),
    ];

    for provider in providers.into_iter().flatten() {
        let _ = start_ddns_updater(provider, client.clone()).await;
    }
}
