use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use reqwest::Client;
use std::time::Duration;
use tokio::time;
use tracing::{info, warn};

const NOIP_UPDATE_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
const NOIP_UPDATE_URL: &str = "https://dynupdate.no-ip.com/nic/update";

/// Start a background task that periodically updates No-IP DNS records
pub async fn start_noip_updater(
    hostname: String,
    username: String,
    password: String,
    client: Client,
) -> Result<()> {
    let auth = format!("{}:{}", username, password);
    let auth_header = format!("Basic {}", BASE64.encode(auth));

    tokio::spawn(async move {
        loop {
            match update_noip_dns(&hostname, &auth_header, &client).await {
                Ok(response) => {
                    info!("No-IP DNS update successful: {}", response);
                }
                Err(e) => {
                    warn!("Failed to update No-IP DNS: {}", e);
                }
            }
            time::sleep(NOIP_UPDATE_INTERVAL).await;
        }
    });

    Ok(())
}

async fn update_noip_dns(hostname: &str, auth_header: &str, client: &Client) -> Result<String> {
    let response = client
        .get(NOIP_UPDATE_URL)
        .header("Authorization", auth_header)
        .query(&[("hostname", hostname)])
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    Ok(response)
}
