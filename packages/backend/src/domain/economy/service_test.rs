#[cfg(test)]
mod tests {
    use crate::domain::economy::models::{CurrencyExchangeData, CurrencySearchResult, EconomyType};
    use crate::domain::economy::service::EconomyService;
    use crate::domain::economy::traits::EconomyRepository;
    use crate::errors::{AppError, AppResult};
    use async_trait::async_trait;
    use std::sync::Arc;
    use std::time::Duration;

    // ============= Mock Repository =============

    struct MockEconomyRepository;

    #[async_trait]
    impl EconomyRepository for MockEconomyRepository {
        async fn load_fresh_exchange_data(
            &self,
            _league: &str,
            _is_hardcore: bool,
            _economy_type: EconomyType,
            _ttl_seconds: u64,
        ) -> AppResult<Option<CurrencyExchangeData>> {
            Ok(None)
        }

        async fn load_exchange_data(
            &self,
            _league: &str,
            _is_hardcore: bool,
            _economy_type: EconomyType,
        ) -> AppResult<Option<CurrencyExchangeData>> {
            Ok(None)
        }

        async fn save_exchange_data(
            &self,
            _league: &str,
            _is_hardcore: bool,
            _economy_type: EconomyType,
            _data: &CurrencyExchangeData,
        ) -> AppResult<()> {
            Ok(())
        }

        async fn load_all_currencies(
            &self,
            _league: &str,
            _is_hardcore: bool,
        ) -> AppResult<Vec<CurrencySearchResult>> {
            Ok(Vec::new())
        }

        async fn search_currencies(
            &self,
            _league: &str,
            _is_hardcore: bool,
            _query: &str,
            _limit: u32,
        ) -> AppResult<Vec<CurrencySearchResult>> {
            Ok(Vec::new())
        }

        async fn toggle_currency_star(
            &self,
            _league: &str,
            _is_hardcore: bool,
            _economy_type: EconomyType,
            _currency_id: &str,
        ) -> AppResult<bool> {
            Ok(false)
        }

        async fn load_starred_currencies(
            &self,
            _league: &str,
            _is_hardcore: bool,
        ) -> AppResult<Vec<CurrencySearchResult>> {
            Ok(Vec::new())
        }
    }

    // ============= Helper Function Tests =============

    #[test]
    fn test_build_poe_ninja_url_basic() {
        let url = EconomyService::build_poe_ninja_url("Standard", EconomyType::Currency);
        assert_eq!(
            url,
            "https://poe.ninja/poe2/api/economy/exchange/current/overview?league=Standard&type=Currency"
        );
    }

    #[test]
    fn test_build_poe_ninja_url_with_spaces() {
        // League names with spaces should be URL-encoded
        let url =
            EconomyService::build_poe_ninja_url("Rise of the Abyssal", EconomyType::Fragments);
        assert!(url.contains("Rise%20of%20the%20Abyssal"));
        assert!(url.contains("type=Fragments"));
    }

    #[test]
    fn test_build_poe_ninja_url_different_economy_types() {
        let economy_types = [
            (EconomyType::Currency, "Currency"),
            (EconomyType::Fragments, "Fragments"),
            (EconomyType::Essences, "Essences"),
            (EconomyType::Runes, "Runes"),
            (EconomyType::Ritual, "Ritual"),
        ];

        for (economy_type, expected_type_str) in economy_types {
            let url = EconomyService::build_poe_ninja_url("TestLeague", economy_type);
            assert!(
                url.contains(&format!("type={expected_type_str}")),
                "URL should contain type={expected_type_str} for {economy_type:?}"
            );
        }
    }

    #[test]
    fn test_calculate_retry_delay_first_attempt() {
        // First attempt (attempt 0) should have no delay
        let delay = EconomyService::calculate_retry_delay(0);
        assert_eq!(delay, Duration::from_millis(0));
    }

    #[test]
    fn test_calculate_retry_delay_second_attempt() {
        // Second attempt (attempt 1) should have initial delay (500ms)
        let delay = EconomyService::calculate_retry_delay(1);
        assert_eq!(delay, Duration::from_millis(500));
    }

    #[test]
    fn test_calculate_retry_delay_third_attempt() {
        // Third attempt (attempt 2) should have 500 * 3 = 1500ms
        let delay = EconomyService::calculate_retry_delay(2);
        assert_eq!(delay, Duration::from_millis(1500));
    }

    #[test]
    fn test_calculate_retry_delay_fourth_attempt() {
        // Fourth attempt (attempt 3) should have 500 * 9 = 4500ms
        let delay = EconomyService::calculate_retry_delay(3);
        assert_eq!(delay, Duration::from_millis(4500));
    }

    #[test]
    fn test_is_retryable_error_network() {
        let error = AppError::Network {
            message: "Connection refused".to_string(),
        };
        assert!(EconomyService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_retryable_error_validation() {
        // Validation errors (4xx) should NOT be retryable
        let error = AppError::Validation {
            message: "Bad request".to_string(),
        };
        assert!(!EconomyService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_retryable_error_serialization() {
        // Serialization errors should NOT be retryable
        let error = AppError::Serialization {
            message: "Failed to parse JSON".to_string(),
        };
        assert!(!EconomyService::is_retryable_error(&error));
    }

    #[test]
    fn test_is_retryable_error_internal() {
        // Internal errors should NOT be retryable
        let error = AppError::internal_error("test_function", "Something went wrong");
        assert!(!EconomyService::is_retryable_error(&error));
    }

    // ============= Service Creation Tests =============

    #[test]
    fn test_service_creation() {
        let mock_repo = Arc::new(MockEconomyRepository) as Arc<dyn EconomyRepository + Send + Sync>;
        let service = EconomyService::new(mock_repo).expect("Failed to create service");
        assert!(service.client.get("https://example.com").build().is_ok());
    }

    #[test]
    fn test_cache_key_format() {
        // Test that cache_key generates unique keys for different combinations
        let key1 = EconomyService::cache_key("Standard", false, EconomyType::Currency);
        let key2 = EconomyService::cache_key("Standard", true, EconomyType::Currency);
        let key3 = EconomyService::cache_key("Standard", false, EconomyType::Fragments);
        let key4 = EconomyService::cache_key("Hardcore", false, EconomyType::Currency);

        // All keys should be different
        assert_ne!(key1, key2, "Hardcore flag should affect key");
        assert_ne!(key1, key3, "Economy type should affect key");
        assert_ne!(key1, key4, "League should affect key");
        assert_ne!(key2, key3);
        assert_ne!(key2, key4);
        assert_ne!(key3, key4);

        // Key format should be "{league}:{is_hardcore}:{economy_type}"
        assert_eq!(key1, "Standard:false:Currency");
        assert_eq!(key2, "Standard:true:Currency");
    }

    #[tokio::test]
    async fn test_concurrent_requests_no_deadlock() {
        // Test that multiple concurrent requests for the same cache key
        // don't cause deadlocks (they should wait and coalesce)
        let mock_repo = Arc::new(MockEconomyRepository) as Arc<dyn EconomyRepository + Send + Sync>;
        let service = Arc::new(EconomyService::new(mock_repo).expect("Failed to create service"));

        // Spawn 3 concurrent requests for same league+type
        let handle1 = tokio::spawn({
            let service = service.clone();
            async move {
                service
                    .fetch_currency_exchange_data("TestLeague", false, EconomyType::Currency)
                    .await
            }
        });

        let handle2 = tokio::spawn({
            let service = service.clone();
            async move {
                service
                    .fetch_currency_exchange_data("TestLeague", false, EconomyType::Currency)
                    .await
            }
        });

        let handle3 = tokio::spawn({
            let service = service.clone();
            async move {
                service
                    .fetch_currency_exchange_data("TestLeague", false, EconomyType::Currency)
                    .await
            }
        });

        // Wait for all requests with timeout to detect deadlock
        let timeout_result = tokio::time::timeout(std::time::Duration::from_secs(15), async {
            tokio::join!(handle1, handle2, handle3)
        })
        .await;

        // Should complete (even if with errors) - no deadlock
        assert!(
            timeout_result.is_ok(),
            "Concurrent requests should complete without deadlock"
        );
    }

    #[tokio::test]
    async fn test_different_cache_keys_dont_block() {
        // Test that requests for different leagues/types don't block each other
        let mock_repo = Arc::new(MockEconomyRepository) as Arc<dyn EconomyRepository + Send + Sync>;
        let service = Arc::new(EconomyService::new(mock_repo).expect("Failed to create service"));

        // Spawn concurrent requests for DIFFERENT cache keys
        let handle1 = tokio::spawn({
            let service = service.clone();
            async move {
                service
                    .fetch_currency_exchange_data("League1", false, EconomyType::Currency)
                    .await
            }
        });

        let handle2 = tokio::spawn({
            let service = service.clone();
            async move {
                service
                    .fetch_currency_exchange_data("League2", false, EconomyType::Currency)
                    .await
            }
        });

        let handle3 = tokio::spawn({
            let service = service.clone();
            async move {
                service
                    .fetch_currency_exchange_data("League1", false, EconomyType::Fragments)
                    .await
            }
        });

        // Wait with timeout
        let timeout_result = tokio::time::timeout(std::time::Duration::from_secs(15), async {
            tokio::join!(handle1, handle2, handle3)
        })
        .await;

        // All should complete (even if with errors)
        assert!(
            timeout_result.is_ok(),
            "Different cache keys should not block each other"
        );
    }

    #[test]
    fn test_league_name_construction_normal() {
        // Normal leagues should remain unchanged
        assert_eq!(
            EconomyService::format_league_for_api("Rise of the Abyssal", false),
            "Rise of the Abyssal"
        );
    }

    #[test]
    fn test_league_name_construction_hardcore() {
        // Non-standard hardcore leagues should have "HC " prefix
        assert_eq!(
            EconomyService::format_league_for_api("Rise of the Abyssal", true),
            "HC Rise of the Abyssal"
        );
    }

    #[test]
    fn test_league_name_construction_standard_hardcore() {
        // Standard + hardcore should be "Hardcore" not "HC Standard"
        assert_eq!(
            EconomyService::format_league_for_api("Standard", true),
            "Hardcore"
        );
    }

    #[test]
    fn test_league_name_construction_standard_case_insensitive() {
        // Should handle "STANDARD", "standard", "Standard" all the same
        for standard_variant in &["Standard", "STANDARD", "standard", "StAnDaRd"] {
            assert_eq!(
                EconomyService::format_league_for_api(standard_variant, true),
                "Hardcore",
                "Failed for variant: {standard_variant}"
            );
        }
    }

    #[test]
    fn test_league_name_construction_the_prefix_stripped() {
        // "The " prefix should be stripped before passing to poe.ninja
        assert_eq!(
            EconomyService::format_league_for_api("The Fate of the Vaal", false),
            "Fate of the Vaal"
        );
        assert_eq!(
            EconomyService::format_league_for_api("The Fate of the Vaal", true),
            "HC Fate of the Vaal"
        );
    }
}
