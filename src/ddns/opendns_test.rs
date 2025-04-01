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
    let mut config = crate::types::config::Config::default_config();
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
        _ => panic!("Wrong provider type"),
    }
}
