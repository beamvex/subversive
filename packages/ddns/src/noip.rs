use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;

use crate::{Config, DdnsProvider, DdnsProviderConfig};

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
            (Some(hostname), Some(username), Some(password)) => Some(DdnsProvider::NoIp(
                Self::new(hostname.clone(), username.clone(), password.clone()),
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, DdnsProvider, DdnsProviderConfig, NoIpProvider};

    use anyhow::Result;
    use mockito::Server;
    use reqwest::Client;

    #[tokio::test]
    async fn test_noip_update_dns() -> Result<()> {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("GET", "/nic/update")
            .match_header("Authorization", "Basic dXNlcjpwYXNz")
            .match_query(mockito::Matcher::UrlEncoded(
                "hostname".into(),
                "example.com".into(),
            ))
            .with_status(200)
            .with_body("good 203.0.113.1")
            .create_async()
            .await;

        let mut provider = NoIpProvider::new(
            "example.com".to_string(),
            "user".to_string(),
            "pass".to_string(),
        );

        provider.base_url = server.url();
        let client = Client::builder().build()?;

        let response = provider.update_dns(&client).await?;
        assert_eq!(response, "good 203.0.113.1");
        Ok(())
    }

    #[test]
    fn test_noip_provider_from_config() {
        let mut config = Config::default();
        assert!(NoIpProvider::try_from_config(&config).is_none());

        config.noip_hostname = Some("example.com".to_string());
        config.noip_username = Some("user".to_string());
        config.noip_password = Some("pass".to_string());

        let provider = NoIpProvider::try_from_config(&config).unwrap();
        match provider {
            DdnsProvider::NoIp(p) => {
                assert_eq!(p.hostname, "example.com");
                assert_eq!(p.username, "user");
                assert_eq!(p.password, "pass");
            }
            _ => panic!("Expected NoIp provider"),
        }
    }
}
