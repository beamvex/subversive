#[cfg(test)]
use std::sync::Arc;

use anyhow::Result;
use reqwest::Client;

use super::{NoIpProvider, OpenDnsProvider};

pub trait DdnsProviderConfig {
    fn try_from_config(config: &crate::types::config::Config) -> Option<DdnsProvider>;
}

#[derive(Debug, Clone)]
pub enum DdnsProvider {
    NoIp(NoIpProvider),
    OpenDns(OpenDnsProvider),
    #[cfg(test)]
    Mock(Arc<dyn UpdateDns>),
}

#[cfg(test)]
pub trait UpdateDns: Send + Sync + std::fmt::Debug {
    fn update_dns(&self, client: &Client) -> Result<String>;
    fn get_provider_name(&self) -> &'static str;
}

impl DdnsProvider {
    pub async fn update_dns(&self, client: &Client) -> Result<String> {
        match self {
            DdnsProvider::NoIp(provider) => provider.update_dns(client).await,
            DdnsProvider::OpenDns(provider) => provider.update_dns(client).await,
            #[cfg(test)]
            DdnsProvider::Mock(provider) => provider.update_dns(client),
        }
    }

    pub fn get_provider_name(&self) -> &'static str {
        match self {
            DdnsProvider::NoIp(_) => "No-IP",
            DdnsProvider::OpenDns(_) => "OpenDNS",
            #[cfg(test)]
            DdnsProvider::Mock(provider) => provider.get_provider_name(),
        }
    }
}
