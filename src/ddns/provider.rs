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
}

impl DdnsProvider {
    pub async fn update_dns(&self, client: &Client) -> Result<String> {
        match self {
            DdnsProvider::NoIp(provider) => provider.update_dns(client).await,
            DdnsProvider::OpenDns(provider) => provider.update_dns(client).await,
        }
    }

    pub fn get_provider_name(&self) -> &'static str {
        match self {
            DdnsProvider::NoIp(_) => "No-IP",
            DdnsProvider::OpenDns(_) => "OpenDNS",
        }
    }
}
