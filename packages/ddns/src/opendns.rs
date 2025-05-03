use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;

use crate::{Config, DdnsProvider, DdnsProviderConfig};

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use mockito::Server;
    use reqwest::Client;

    #[tokio::test]
    async fn test_opendns_update_dns() -> Result<()> {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/nic/update")
            .match_header("Authorization", "Basic dXNlcjpwYXNz")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("hostname".into(), "example.com".into()),
                mockito::Matcher::UrlEncoded("network".into(), "home".into()),
            ]))
            .with_status(200)
            .with_body("good 203.0.113.1")
            .create_async()
            .await;

        let mut provider = OpenDnsProvider::new(
            "example.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
            "home".to_string(),
        );

        provider.base_url = server.url();
        let client = Client::builder().build()?;

        let response = provider.update_dns(&client).await?;
        assert_eq!(response, "good 203.0.113.1");
        Ok(())
    }

    #[test]
    fn test_opendns_provider_from_config() {
        let mut config = Config::default();
        assert!(OpenDnsProvider::try_from_config(&config).is_none());

        config.opendns_hostname = Some("example.com".to_string());
        config.opendns_username = Some("user".to_string());
        config.opendns_password = Some("pass".to_string());
        config.opendns_network = Some("home".to_string());

        let provider = OpenDnsProvider::try_from_config(&config).unwrap();
        match provider {
            DdnsProvider::OpenDns(p) => {
                assert_eq!(p.hostname, "example.com");
                assert_eq!(p.username, "user");
                assert_eq!(p.password, "pass");
                assert_eq!(p.network, "home");
            }
            DdnsProvider::NoIp(_no_ip_provider) => todo!(),
        }
    }
}
