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

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time;

    use mockall::automock;

    use crate::ddns::config_ddns;
    use crate::ddns::start_ddns_updater;
    use crate::types::config::Config;
    use subversive_utils::test_utils::init_test_tracing;

    use super::provider::{DdnsProvider, UpdateDns};
    use anyhow::Result;
    use reqwest::Client;

    // Mock provider for testing
    #[derive(Debug)]
    #[allow(dead_code)]
    struct TestProvider;

    #[automock]
    impl UpdateDns for TestProvider {
        fn update_dns(&self, _client: &Client) -> Result<String> {
            unimplemented!()
        }

        fn get_provider_name(&self) -> &'static str {
            unimplemented!()
        }
    }

    impl Clone for MockTestProvider {
        fn clone(&self) -> Self {
            Self::new()
        }
    }

    #[tokio::test]
    async fn test_start_ddns_updater_success() -> Result<()> {
        init_test_tracing();

        // Create a mock provider that succeeds
        let mut mock_provider = MockTestProvider::new();
        mock_provider
            .expect_get_provider_name()
            .returning(|| "MockProvider");
        mock_provider
            .expect_update_dns()
            .returning(|_| Ok("Update successful".to_string()));

        let provider = DdnsProvider::Mock(Arc::new(mock_provider));
        let client = Client::new();

        // Start the updater with a very short interval for testing
        const TEST_INTERVAL: Duration = Duration::from_millis(100);

        // Create a sleep set to control time
        let sleep = time::sleep(TEST_INTERVAL);
        tokio::pin!(sleep);

        // Start the updater
        start_ddns_updater(provider, client).await?;

        // Wait for the first update
        sleep.as_mut().await;

        Ok(())
    }

    #[tokio::test]
    async fn test_start_ddns_updater_failure() -> Result<()> {
        init_test_tracing();

        // Create a mock provider that fails
        let mut mock_provider = MockTestProvider::new();
        mock_provider
            .expect_get_provider_name()
            .returning(|| "MockProvider");
        mock_provider
            .expect_update_dns()
            .returning(|_| Err(anyhow::anyhow!("Update failed")));

        let provider = DdnsProvider::Mock(Arc::new(mock_provider));
        let client = Client::new();

        // Start the updater with a very short interval for testing
        const TEST_INTERVAL: Duration = Duration::from_millis(100);

        // Create a sleep set to control time
        let sleep = time::sleep(TEST_INTERVAL);
        tokio::pin!(sleep);

        // Start the updater
        start_ddns_updater(provider, client).await?;

        // Wait for the first update
        sleep.as_mut().await;

        Ok(())
    }

    #[tokio::test]
    async fn test_config_ddns_no_providers() {
        init_test_tracing();

        // Create a config with no DDNS providers
        let config = Config::default_config();

        // Configure DDNS
        config_ddns(&config).await;
    }

    #[tokio::test]
    async fn test_config_ddns_with_providers() {
        init_test_tracing();

        // Create a config with both providers
        let mut config = Config::default_config();

        // Add NoIP config
        config.noip_hostname = Some("test.example.com".to_string());
        config.noip_username = Some("test_user".to_string());
        config.noip_password = Some("test_pass".to_string());

        // Add OpenDNS config
        config.opendns_hostname = Some("test.example.com".to_string());
        config.opendns_username = Some("test_user".to_string());
        config.opendns_password = Some("test_pass".to_string());
        config.opendns_network = Some("test_network".to_string());

        // Configure DDNS
        config_ddns(&config).await;
    }
}
