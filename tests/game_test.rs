use anyhow::Result;
use base58::ToBase58;
use chrono::{Timelike, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use subversive_crypto::Address;
use subversive_utils::test_utils::init_test_tracing;
use tokio::sync::Mutex;
use tracing::info;

// Result: "2025-05-01T06:37:00Z"

#[tokio::test]
async fn test_game() -> Result<()> {
    init_test_tracing();

    info!("test_game");

    // Create addresses in parallel using 100 threads
    let mut handles = Vec::new();
    for _ in 0..100 {
        let handle = tokio::spawn(async move {
            // Each thread creates 10 addresses
            (0..1).map(|_| Address::new()).collect::<Vec<Address>>()
        });
        handles.push(handle);
    }

    // Wait for all threads and collect results
    let mut addresses = Vec::new();
    for handle in handles {
        let mut thread_addresses = handle.await.unwrap();
        addresses.append(&mut thread_addresses);
    }
    info!("Created {} addresses in parallel", addresses.len());

    // Print first address as example
    if let Some(first_address) = addresses.first() {
        info!(
            "First address: {}",
            first_address
                .get_private_key()
                .unwrap()
                .to_bytes()
                .to_base58()
        );
    }

    let time = Utc::now();
    let rounded = time.with_nanosecond(0).unwrap().with_second(0).unwrap();
    let iso_string = rounded.to_rfc3339();
    info!("rounded time: {}", iso_string);

    // Wrap addresses in Arc<Mutex> for safe concurrent access
    let addresses: Vec<Arc<Mutex<Address>>> = addresses
        .into_iter()
        .map(|addr| Arc::new(Mutex::new(addr)))
        .collect();

    // Sign messages in parallel using 100 threads
    let mut handles = Vec::new();

    // Split work into chunks by index
    for idx in 0..addresses.len() {
        let iso_string = iso_string.clone();
        let addr = addresses[idx].clone();

        let handle = tokio::spawn(async move { addr.lock().await.sign(&iso_string).unwrap() });
        handles.push(handle);
    }

    // Wait for all threads and collect results
    let mut signatures = Vec::new();
    for handle in handles {
        let signature = handle.await.unwrap();
        signatures.push(signature);
    }

    // Sort signatures
    signatures.sort();

    info!("Created {} signatures in parallel", signatures.len());
    // Print first signature (now the alphabetically first one)
    if let Some(first_sig) = signatures.first() {
        info!("First signature (sorted): {}", first_sig);
    }

    // Print last signature (now the alphabetically last one)
    if let Some(last_sig) = signatures.last() {
        info!("Last signature (sorted): {}", last_sig);
    }

    // Create a HashMap of addresses to signatures
    let mut addr_sig_map: HashMap<String, String> = HashMap::with_capacity(addresses.len());
    for (addr, sig) in addresses.iter().zip(signatures.iter()) {
        let addr = addr.lock().await;
        let addr_str = addr.get_public_address().to_string();
        addr_sig_map.insert(addr_str, sig.clone());
    }

    // Convert to vec for sorting
    let mut addr_sig_pairs: Vec<_> = addr_sig_map.into_iter().collect();
    // Sort by signature instead of address
    addr_sig_pairs.sort_by(|a, b| a.1.cmp(&b.1));

    info!("Created {} address-signature pairs", addr_sig_pairs.len());

    // Print top 10 addresses (by signature)
    info!("Top 10 addresses by signature:");
    info!("| Address | Signature |");
    info!("|---------|-----------|");
    for (addr, sig) in addr_sig_pairs.iter().take(10) {
        info!("| {} | {} |", addr, sig);
    }

    info!("");
    // Print bottom 10 addresses (by signature)
    info!("Bottom 10 addresses by signature:");
    info!("| Address | Signature |");
    info!("|---------|-----------|");
    for (addr, sig) in addr_sig_pairs.iter().rev().take(10) {
        info!("| {} | {} |", addr, sig);
    }

    Ok(())
}
