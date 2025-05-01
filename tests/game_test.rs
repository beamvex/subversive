use anyhow::Result;
use base58::ToBase58;
use chrono::{DateTime, Timelike, Utc};
use subversive_crypto::Address;
use subversive_utils::test_utils::init_test_tracing;
use tracing::info;

// Result: "2025-05-01T06:37:00Z"

#[tokio::test]
async fn test_game() -> Result<()> {
    init_test_tracing();

    info!("test_game");

    let mut address = Address::new();
    info!(
        "address: {}",
        address.get_private_key().unwrap().to_bytes().to_base58()
    );

    let time = Utc::now();
    let rounded = time.with_nanosecond(0).unwrap().with_second(0).unwrap();
    let iso_string = rounded.to_rfc3339();
    info!("rounded time: {}", iso_string);

    let signature = address.sign(&iso_string).unwrap();
    info!("signature: {}", signature);

    Ok(())
}
