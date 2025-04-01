use super::*;
use crate::types::config::Config;

#[test]
fn test_provider_name() {
    let noip = DdnsProvider::NoIp(NoIpProvider::new(
        "example.com".to_string(),
        "user".to_string(),
        "pass".to_string(),
    ));
    assert_eq!(noip.get_provider_name(), "No-IP");

    let opendns = DdnsProvider::OpenDns(OpenDnsProvider::new(
        "example.com".to_string(),
        "user".to_string(),
        "pass".to_string(),
        "network".to_string(),
    ));
    assert_eq!(opendns.get_provider_name(), "OpenDNS");
}

#[test]
fn test_try_from_empty_config() {
    let config = Config::default_config();
    assert!(NoIpProvider::try_from_config(&config).is_none());
    assert!(OpenDnsProvider::try_from_config(&config).is_none());
}
