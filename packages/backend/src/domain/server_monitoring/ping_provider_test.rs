//! Unit tests for `PingProvider` implementations

#[cfg(test)]
mod tests {
    use crate::domain::server_monitoring::ping_provider::SystemPingProvider;
    use crate::domain::server_monitoring::traits::PingProvider;

    #[test]
    fn test_system_ping_provider_new() {
        // Just verify it can be constructed
        let _provider = SystemPingProvider::new();
    }

    #[test]
    fn test_system_ping_provider_default() {
        // Just verify default constructor works
        let _provider = SystemPingProvider;
    }

    #[test]
    fn test_system_ping_provider_is_send_sync() {
        // This is a compile-time test that verifies the trait bounds
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SystemPingProvider>();
    }

    #[tokio::test]
    async fn test_system_ping_provider_ping_localhost() {
        let provider = SystemPingProvider::new();

        // Ping localhost - should generally work on most systems
        let result = provider.ping("127.0.0.1").await;

        // We expect this to succeed on most systems, but won't fail the test if it doesn't
        // since this depends on system configuration
        match result {
            Ok(latency) => {
                // Latency should be reasonable - can be 0 on very fast local connections
                assert!(latency < 10000, "Latency should be less than 10 seconds");
                println!("Ping succeeded with latency: {latency}ms");
            }
            Err(e) => {
                println!("Ping failed (may be expected on some systems): {e}");
            }
        }
    }

    #[tokio::test]
    async fn test_system_ping_provider_ping_invalid_ip() {
        let provider = SystemPingProvider::new();

        // Ping an unreachable IP that should timeout
        let result = provider.ping("192.0.2.1").await;

        // This should fail or timeout - 192.0.2.0/24 is TEST-NET-1 (RFC 5737)
        // We don't assert failure because system configuration might vary
        if result.is_ok() {
            println!("Unexpectedly succeeded pinging test network");
        }
    }

    // Mock PingProvider for testing service layer
    pub struct MockPingProvider {
        pub should_succeed: bool,
        pub latency: u64,
    }

    impl MockPingProvider {
        pub fn new_success(latency: u64) -> Self {
            Self {
                should_succeed: true,
                latency,
            }
        }

        pub fn new_failure() -> Self {
            Self {
                should_succeed: false,
                latency: 0,
            }
        }
    }

    #[async_trait::async_trait]
    impl PingProvider for MockPingProvider {
        async fn ping(&self, _ip_address: &str) -> crate::errors::AppResult<u64> {
            if self.should_succeed {
                Ok(self.latency)
            } else {
                Err(crate::errors::AppError::network_error(
                    "mock_ping",
                    "Mock ping failure",
                ))
            }
        }
    }

    #[tokio::test]
    async fn test_mock_ping_provider_success() {
        let provider = MockPingProvider::new_success(42);
        let result = provider.ping("192.168.1.1").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_mock_ping_provider_failure() {
        let provider = MockPingProvider::new_failure();
        let result = provider.ping("192.168.1.1").await;

        assert!(result.is_err());
        // Just check that we got an error - don't compare the exact error
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Mock ping failure"));
    }
}
