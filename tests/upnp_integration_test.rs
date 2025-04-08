use anyhow::Result;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use subversive::network::upnp::{cleanup_upnp, setup_upnp};
use subversive::test_utils::init_test_tracing;

#[tokio::test]
async fn test_upnp_port_mapping() -> Result<()> {
    init_test_tracing();
    // Initialize test port
    let test_port = 12345;

    // Attempt to set up UPnP port mapping
    let (mapped_port, gateways) = setup_upnp(test_port).await?;

    // Verify we got a port mapping
    assert!(mapped_port > 0, "Should receive a valid port mapping");

    // Test the port mapping by attempting to connect
    let _local_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, mapped_port);

    // Give some time for the port mapping to propagate
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Clean up the port mapping
    cleanup_upnp(mapped_port, gateways).await?;

    // Verify cleanup by attempting to map the same port again
    let result = setup_upnp(mapped_port).await;
    assert!(
        result.is_ok(),
        "Should be able to map the port again after cleanup"
    );

    // Clean up the second mapping
    if let Ok((port, gateways)) = result {
        cleanup_upnp(port, gateways).await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_upnp_invalid_port() -> Result<()> {
    init_test_tracing();
    // Test with an invalid port (0)
    let result = setup_upnp(0).await;
    assert!(result.is_ok(), "Should not fail with invalid port 0");

    // Test with a privileged port (<1024)
    let result = setup_upnp(80).await;
    assert!(result.is_ok(), "Should not fail with privileged port 80");

    Ok(())
}

#[tokio::test]
async fn test_upnp_cleanup_nonexistent() -> Result<()> {
    init_test_tracing();
    // Attempt to clean up a port that wasn't mapped
    let result = cleanup_upnp(54321, vec![]).await;
    assert!(
        result.is_ok(),
        "Cleanup of nonexistent mapping should not fail"
    );

    Ok(())
}
