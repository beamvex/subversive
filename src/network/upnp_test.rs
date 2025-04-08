#[cfg(test)]
mod tests {
    use crate::network::cleanup_upnp;
    use crate::network::peer::{
        add_peer, add_peers, get_peers, remove_peer, update_peer_last_seen,
    };
    use crate::types::state::AppState;
    use crate::{db::DbContext, shutdown::ShutdownState, types::config::Config};

    #[cfg(test)]
    use mockall::predicate::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use tracing::info;

    use crate::network::upnp::{
        set_wsl_path, setup_upnp_with_search, try_setup_upnp, DefaultGatewaySearch, Gateway2,
        IGateway, MockGatewaySearch, MockIGateway,
    };
    use crate::test_utils::init_test_tracing;
    use igd::PortMappingProtocol;

    pub fn init_test_upnp() {
        init_test_tracing();
    }

    #[tokio::test]
    async fn test_try_setup_upnp() -> anyhow::Result<()> {
        let port = 12345;
        let mut mock_search = MockGatewaySearch::new();

        mock_search
            .expect_search_gateway()
            .times(1)
            .returning(move || {
                let mut mock = MockIGateway::new();
                mock.expect_root_url()
                    .returning(|| "http://mock-gateway".to_string());
                mock.expect_add_port().returning(|_, _, _, _, _| Ok(()));
                Ok(Gateway2::Mock(Arc::new(mock)))
            });

        let gateway = try_setup_upnp(port, mock_search).await?;
        assert_eq!(gateway.root_url(), "http://mock-gateway");

        Ok(())
    }

    #[tokio::test]
    async fn test_setup_upnp_wsl() -> anyhow::Result<()> {
        init_test_upnp();
        // Create a temporary file to simulate WSL environment
        let temp_dir = tempfile::tempdir()?;
        let wsl_path = temp_dir.path().join("WSLInterop");
        std::fs::write(&wsl_path, "")?;

        info!("WSL2 temp file {}", wsl_path.display());

        let _ = set_wsl_path(wsl_path.display().to_string().as_str()).await;

        let result = setup_upnp_with_search(8080, DefaultGatewaySearch).await?;
        assert_eq!(result.1.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_setup_upnp_success() -> anyhow::Result<()> {
        init_test_upnp();
        let success_port = 8080;
        let local_ipv4 = crate::network::local_ip::get_local_ipv4()?;

        // Create a mock gateway that fails for the first port but succeeds for the second
        let mut mock_gateway = MockIGateway::new();
        mock_gateway
            .expect_add_port()
            .with(
                eq(PortMappingProtocol::TCP),
                eq(success_port),
                eq(std::net::SocketAddrV4::new(local_ipv4, success_port)),
                eq(0),
                eq("P2P Network"),
            )
            .returning(|_, _, _, _, _| Ok(()));
        mock_gateway
            .expect_root_url()
            .returning(|| "http://mock-gateway".to_string());

        let mg = Gateway2::Mock(Arc::new(mock_gateway));

        // Create a mock gateway search
        let mut mock_search = MockGatewaySearch::new();
        mock_search
            .expect_search_gateway()
            .returning(move || Ok(mg.clone()));

        let result = setup_upnp_with_search(success_port, mock_search).await?;
        let (port, gateways) = result;

        assert_eq!(port, success_port);
        assert_eq!(gateways.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn test_cleanup_upnp_success() -> anyhow::Result<()> {
        init_test_upnp();
        let port = 8080;

        // Create a mock gateway
        let mut mock_gateway = MockIGateway::new();
        mock_gateway
            .expect_remove_port()
            .with(eq(PortMappingProtocol::TCP), eq(port))
            .returning(|_, _| Ok(()));
        mock_gateway
            .expect_root_url()
            .returning(|| "http://mock-gateway".to_string());

        // Cleanup should succeed even if removing port fails
        let gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];
        cleanup_upnp(port, gateways).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_cleanup_upnp_failure() -> anyhow::Result<()> {
        init_test_upnp();
        let port = 8080;

        // Create a mock gateway that fails to remove port
        let mut mock_gateway = MockIGateway::new();
        mock_gateway.expect_remove_port().returning(|_, _| {
            Err(igd::RemovePortError::RequestError(
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to remove port").into(),
            )
            .into())
        });

        mock_gateway
            .expect_root_url()
            .returning(|| "http://mock-gateway".to_string());

        // Cleanup should succeed even if removing port fails
        let gateways = vec![Gateway2::Mock(Arc::new(mock_gateway))];
        cleanup_upnp(port, gateways).await?;

        Ok(())
    }

    use std::time::Duration;

    async fn setup_test_state() -> Arc<AppState> {
        Arc::new(AppState {
            peers: Arc::new(Mutex::new(HashMap::new())),
            db: Arc::new(DbContext::new_memory().await.unwrap()),
            own_address: "http://localhost:8080".to_string(),
            shutdown: Arc::new(ShutdownState::new(8080, vec![])),
            config: Config::default_config(),
            actual_port: 8080,
        })
    }

    #[tokio::test]
    async fn test_add_peer() {
        let state = setup_test_state().await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add a peer
        add_peer(state.clone(), peer_addr.clone()).await;

        // Verify peer was added
        let peers = state.peers.lock().await;
        assert!(peers.contains_key(&peer_addr));

        // Try adding same peer again
        drop(peers);
        add_peer(state.clone(), peer_addr.clone()).await;

        // Verify no duplicate was added
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 1);
    }

    #[tokio::test]
    async fn test_add_peers() {
        let state = setup_test_state().await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
            "http://localhost:8082".to_string(),
        ];

        // Add multiple peers
        add_peers(state.clone(), peer_addrs.clone()).await;

        // Verify all peers were added
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 3);
        for addr in peer_addrs {
            assert!(peers.contains_key(&addr));
        }
    }

    #[tokio::test]
    async fn test_get_peers() {
        let state = setup_test_state().await;
        let peer_addrs = vec![
            "http://localhost:8080".to_string(),
            "http://localhost:8081".to_string(),
        ];

        // Add peers
        add_peers(state.clone(), peer_addrs.clone()).await;

        // Get peers and verify
        let result = get_peers(state.clone()).await;
        assert_eq!(result.len(), 2);
        for addr in peer_addrs {
            assert!(result.contains(&addr));
        }
    }

    #[tokio::test]
    async fn test_remove_peer() {
        let state = setup_test_state().await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add and then remove a peer
        add_peer(state.clone(), peer_addr.clone()).await;
        remove_peer(state.clone(), peer_addr.clone()).await;

        // Verify peer was removed
        let peers = state.peers.lock().await;
        assert!(!peers.contains_key(&peer_addr));

        // Try removing non-existent peer
        drop(peers);
        remove_peer(state.clone(), "http://nonexistent:8080".to_string()).await;
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 0);
    }

    #[tokio::test]
    async fn test_update_peer_last_seen() {
        init_test_tracing();
        let state = setup_test_state().await;
        let peer_addr = "http://localhost:8080".to_string();

        // Add a peer
        add_peer(state.clone(), peer_addr.clone()).await;

        // Get initial last seen time
        let peers = state.peers.lock().await;
        let initial_last_seen = peers.get(&peer_addr).unwrap().last_seen;
        drop(peers);

        // Wait a moment
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Update last seen
        update_peer_last_seen(state.clone(), peer_addr.clone()).await;

        // Verify last seen was updated
        let peers = state.peers.lock().await;
        let new_last_seen = peers.get(&peer_addr).unwrap().last_seen;
        info!(
            "Initial last seen: {}, new last seen: {}",
            initial_last_seen, new_last_seen
        );
        assert!(new_last_seen > initial_last_seen);
    }

    #[tokio::test]
    async fn test_update_peer_last_seen_non_existent() {
        init_test_tracing();
        let state = setup_test_state().await;
        // Try updating non-existent peer
        update_peer_last_seen(state.clone(), "http://nonexistent:8080".to_string()).await;
        // Should not panic or affect existing peers
        let peers = state.peers.lock().await;
        assert_eq!(peers.len(), 0);
    }
}
