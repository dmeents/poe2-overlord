//! Unit tests for `ServerMonitoringService`

#[cfg(test)]
mod tests {
    use crate::domain::server_monitoring::models::ServerStatus;
    use crate::domain::server_monitoring::service::ServerMonitoringServiceImpl;
    use crate::domain::server_monitoring::traits::{
        PingProvider, ServerMonitoringService, ServerStatusRepository,
    };
    use crate::errors::AppResult;
    use crate::infrastructure::events::{AppEvent, EventBus};
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // Mock PingProvider for testing
    struct MockPingProvider {
        should_succeed: Arc<RwLock<bool>>,
        latency: Arc<RwLock<u64>>,
        call_count: Arc<RwLock<usize>>,
    }

    impl MockPingProvider {
        fn new_success(latency: u64) -> Self {
            Self {
                should_succeed: Arc::new(RwLock::new(true)),
                latency: Arc::new(RwLock::new(latency)),
                call_count: Arc::new(RwLock::new(0)),
            }
        }

        fn new_failure() -> Self {
            Self {
                should_succeed: Arc::new(RwLock::new(false)),
                latency: Arc::new(RwLock::new(0)),
                call_count: Arc::new(RwLock::new(0)),
            }
        }

        async fn set_should_succeed(&self, succeed: bool) {
            *self.should_succeed.write().await = succeed;
        }

        async fn set_latency(&self, latency: u64) {
            *self.latency.write().await = latency;
        }

        async fn get_call_count(&self) -> usize {
            *self.call_count.read().await
        }
    }

    #[async_trait]
    impl PingProvider for MockPingProvider {
        async fn ping(&self, _ip_address: &str) -> AppResult<u64> {
            *self.call_count.write().await += 1;

            if *self.should_succeed.read().await {
                Ok(*self.latency.read().await)
            } else {
                Err(crate::errors::AppError::network_error(
                    "mock_ping",
                    "Mock ping failure",
                ))
            }
        }
    }

    // Mock ServerStatusRepository for testing
    struct MockServerStatusRepository {
        saved_status: Arc<RwLock<Option<ServerStatus>>>,
    }

    impl MockServerStatusRepository {
        fn new() -> Self {
            Self {
                saved_status: Arc::new(RwLock::new(None)),
            }
        }
    }

    #[async_trait]
    impl ServerStatusRepository for MockServerStatusRepository {
        async fn save(&self, status: &ServerStatus) -> AppResult<()> {
            *self.saved_status.write().await = Some(status.clone());
            Ok(())
        }

        async fn load(&self) -> AppResult<Option<ServerStatus>> {
            Ok(self.saved_status.read().await.clone())
        }
    }

    async fn create_test_service(
        ping_provider: Arc<dyn PingProvider>,
    ) -> ServerMonitoringServiceImpl {
        let event_bus = Arc::new(EventBus::new());
        let repository = Arc::new(MockServerStatusRepository::new());
        ServerMonitoringServiceImpl::new(event_bus, ping_provider, repository)
            .await
            .expect("Failed to create test service")
    }

    #[tokio::test]
    async fn test_service_creation() {
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        // Service should be created successfully
        let _service = create_test_service(ping_provider).await;
    }

    #[tokio::test]
    async fn test_update_server_from_log() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        // Subscribe to events
        let mut receiver = event_bus
            .get_receiver(crate::infrastructure::events::EventType::ServerMonitoring)
            .await
            .expect("Failed to get receiver");

        let ip = "192.168.1.100".to_string();
        let port = 6112;

        let result = service.update_server_from_log(ip.clone(), port).await;
        assert!(result.is_ok(), "Should successfully update server from log");

        // Check that an event was published
        tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("Should receive event within timeout")
            .expect("Should receive an event");
    }

    #[tokio::test]
    async fn test_ping_current_server_with_no_server() {
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let service = create_test_service(ping_provider.clone()).await;

        // Ping without setting a server first
        let result = service.ping_current_server().await;

        // Should succeed but do nothing
        assert!(result.is_ok());

        // Ping provider should not have been called
        assert_eq!(ping_provider.get_call_count().await, 0);
    }

    #[tokio::test]
    async fn test_ping_current_server_success() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(42));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider.clone(), repository)
                .await
                .expect("Failed to create service");

        // First, set a server
        let ip = "192.168.1.100".to_string();
        let port = 6112;
        service
            .update_server_from_log(ip.clone(), port)
            .await
            .expect("Should set server");

        // Subscribe to events after initial setup
        let mut receiver = event_bus
            .get_receiver(crate::infrastructure::events::EventType::ServerMonitoring)
            .await
            .expect("Failed to get receiver");

        // Now ping it
        let result = service.ping_current_server().await;
        assert!(result.is_ok(), "Ping should succeed");

        // Verify ping was called
        assert_eq!(ping_provider.get_call_count().await, 1);

        // Check that a status changed event was published
        let event = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("Should receive event within timeout")
            .expect("Should receive an event");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event {
            assert!(new_status.is_online, "Server should be marked as online");
            assert_eq!(new_status.latency_ms, Some(42));
            assert_eq!(new_status.ip_address, ip);
            assert_eq!(new_status.port, port);
        } else {
            panic!("Expected ServerStatusChanged event");
        }
    }

    #[tokio::test]
    async fn test_ping_current_server_failure() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_failure());
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider.clone(), repository)
                .await
                .expect("Failed to create service");

        // First, set a server
        let ip = "192.168.1.100".to_string();
        let port = 6112;
        service
            .update_server_from_log(ip.clone(), port)
            .await
            .expect("Should set server");

        // Subscribe to events after initial setup
        let mut receiver = event_bus
            .get_receiver(crate::infrastructure::events::EventType::ServerMonitoring)
            .await
            .expect("Failed to get receiver");

        // Now ping it (should fail)
        let result = service.ping_current_server().await;
        assert!(
            result.is_ok(),
            "Service should handle ping failure gracefully"
        );

        // Verify ping was called
        assert_eq!(ping_provider.get_call_count().await, 1);

        // Check that a status changed event was published
        let event = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("Should receive event within timeout")
            .expect("Should receive an event");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event {
            assert!(!new_status.is_online, "Server should be marked as offline");
            assert_eq!(new_status.latency_ms, None);
            assert_eq!(new_status.ip_address, ip);
            assert_eq!(new_status.port, port);
        } else {
            panic!("Expected ServerStatusChanged event");
        }
    }

    #[tokio::test]
    async fn test_ping_invalid_server() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(42));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider.clone(), repository)
                .await
                .expect("Failed to create service");

        // Set an invalid server (empty IP)
        service
            .update_server_from_log(String::new(), 6112)
            .await
            .expect("Should set server");

        // Now ping it - should do nothing because server is invalid
        let result = service.ping_current_server().await;
        assert!(result.is_ok(), "Should handle invalid server gracefully");

        // Verify ping was NOT called
        assert_eq!(ping_provider.get_call_count().await, 0);
    }

    #[tokio::test]
    async fn test_start_ping_monitoring() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        let result = service.start_ping_monitoring().await;
        assert!(result.is_ok(), "Should start monitoring successfully");

        // Clean up
        service
            .stop_ping_monitoring()
            .await
            .expect("Should stop monitoring");
    }

    #[tokio::test]
    async fn test_start_ping_monitoring_twice() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        // Start monitoring
        let result1 = service.start_ping_monitoring().await;
        assert!(result1.is_ok(), "First start should succeed");

        // Try to start again
        let result2 = service.start_ping_monitoring().await;
        assert!(
            result2.is_ok(),
            "Second start should succeed but do nothing"
        );

        // Clean up
        service
            .stop_ping_monitoring()
            .await
            .expect("Should stop monitoring");
    }

    #[tokio::test]
    async fn test_stop_ping_monitoring() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        // Start monitoring
        service
            .start_ping_monitoring()
            .await
            .expect("Should start monitoring");

        // Stop monitoring
        let result = service.stop_ping_monitoring().await;
        assert!(result.is_ok(), "Should stop monitoring successfully");
    }

    #[tokio::test]
    async fn test_stop_ping_monitoring_when_not_started() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        // Stop without starting
        let result = service.stop_ping_monitoring().await;
        assert!(result.is_ok(), "Should handle stopping when not started");
    }

    #[tokio::test]
    async fn test_service_is_cloneable() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        // Clone the service
        let cloned_service = service.clone();

        // Both should work independently
        service
            .update_server_from_log("192.168.1.1".to_string(), 6112)
            .await
            .expect("Original service should work");

        cloned_service
            .update_server_from_log("192.168.1.2".to_string(), 6112)
            .await
            .expect("Cloned service should work");
    }

    #[tokio::test]
    async fn test_multiple_status_updates() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider, repository)
                .await
                .expect("Failed to create service");

        let mut receiver = event_bus
            .get_receiver(crate::infrastructure::events::EventType::ServerMonitoring)
            .await
            .expect("Failed to get receiver");

        // Update server multiple times
        service
            .update_server_from_log("192.168.1.1".to_string(), 6112)
            .await
            .expect("First update should succeed");

        service
            .update_server_from_log("192.168.1.2".to_string(), 6112)
            .await
            .expect("Second update should succeed");

        service
            .update_server_from_log("192.168.1.3".to_string(), 6112)
            .await
            .expect("Third update should succeed");

        // Should receive 3 events
        let event1 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event1");
        let event2 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event2");
        let event3 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event3");

        // Verify the events have different IPs
        if let AppEvent::ServerStatusChanged { new_status, .. } = event1 {
            assert_eq!(new_status.ip_address, "192.168.1.1");
        }
        if let AppEvent::ServerStatusChanged { new_status, .. } = event2 {
            assert_eq!(new_status.ip_address, "192.168.1.2");
        }
        if let AppEvent::ServerStatusChanged { new_status, .. } = event3 {
            assert_eq!(new_status.ip_address, "192.168.1.3");
        }
    }

    #[tokio::test]
    async fn test_ping_latency_updates() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(100));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider.clone(), repository)
                .await
                .expect("Failed to create service");

        // Set a server
        service
            .update_server_from_log("192.168.1.1".to_string(), 6112)
            .await
            .expect("Should set server");

        let mut receiver = event_bus
            .get_receiver(crate::infrastructure::events::EventType::ServerMonitoring)
            .await
            .expect("Failed to get receiver");

        // Ping with first latency
        service
            .ping_current_server()
            .await
            .expect("Ping should succeed");
        let event1 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event1");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event1 {
            assert_eq!(new_status.latency_ms, Some(100));
        }

        // Change latency
        ping_provider.set_latency(50).await;

        // Ping again with new latency
        service
            .ping_current_server()
            .await
            .expect("Ping should succeed");
        let event2 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event2");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event2 {
            assert_eq!(new_status.latency_ms, Some(50));
        }
    }

    #[tokio::test]
    async fn test_server_status_transitions() {
        let event_bus = Arc::new(EventBus::new());
        let ping_provider = Arc::new(MockPingProvider::new_success(50));
        let repository = Arc::new(MockServerStatusRepository::new());
        let service =
            ServerMonitoringServiceImpl::new(event_bus.clone(), ping_provider.clone(), repository)
                .await
                .expect("Failed to create service");

        // Set a server
        service
            .update_server_from_log("192.168.1.1".to_string(), 6112)
            .await
            .expect("Should set server");

        let mut receiver = event_bus
            .get_receiver(crate::infrastructure::events::EventType::ServerMonitoring)
            .await
            .expect("Failed to get receiver");

        // Ping successfully (online)
        service
            .ping_current_server()
            .await
            .expect("Ping should succeed");
        let event1 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event1");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event1 {
            assert!(new_status.is_online);
            assert!(new_status.latency_ms.is_some());
        }

        // Make ping fail (offline)
        ping_provider.set_should_succeed(false).await;
        service
            .ping_current_server()
            .await
            .expect("Service should handle failure");
        let event2 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event2");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event2 {
            assert!(!new_status.is_online);
            assert_eq!(new_status.latency_ms, None);
        }

        // Make ping succeed again (back online)
        ping_provider.set_should_succeed(true).await;
        service
            .ping_current_server()
            .await
            .expect("Ping should succeed");
        let event3 = tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv())
            .await
            .expect("timeout")
            .expect("event3");

        if let AppEvent::ServerStatusChanged { new_status, .. } = event3 {
            assert!(new_status.is_online);
            assert!(new_status.latency_ms.is_some());
        }
    }
}
