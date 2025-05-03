use anyhow::Result;
use reqwest::Client;

mod opendns;
mod noip;
#[cfg(test)]
mod opendns_test;
#[cfg(test)]
mod noip_test;

pub use opendns::OpenDnsProvider;
pub use noip::NoIpProvider;

#[derive(Debug, Clone)]
pub enum DdnsProvider {
    OpenDns(OpenDnsProvider),
    NoIp(NoIpProvider),
}

pub trait DdnsProviderConfig {
    fn try_from_config(config: &Config) -> Option<DdnsProvider>;
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub opendns_hostname: Option<String>,
    pub opendns_username: Option<String>,
    pub opendns_password: Option<String>,
    pub opendns_network: Option<String>,
    pub noip_hostname: Option<String>,
    pub noip_username: Option<String>,
    pub noip_password: Option<String>,
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
