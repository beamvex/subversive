use anyhow::Result;

/// Get the external IP address of the machine
///
/// # Returns
/// The external IP address as a string
pub async fn get_external_ip() -> Result<String> {
    let response = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(response)
}
