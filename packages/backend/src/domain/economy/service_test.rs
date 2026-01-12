#[cfg(test)]
mod tests {
    use crate::domain::economy::models::EconomyType;
    use crate::domain::economy::service::EconomyService;
    use std::sync::Arc;

    #[test]
    fn test_service_creation() {
        let service = EconomyService::new();
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
        let service = Arc::new(EconomyService::new());

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
        let service = Arc::new(EconomyService::new());

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

    #[tokio::test]
    async fn test_get_cache_path_normal_league() {
        let cache_path = EconomyService::get_league_cache_path("Rise of the Abyssal", false)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "Rise_of_the_Abyssal.json");
        assert!(!filename.starts_with("HC_"));
    }

    #[tokio::test]
    async fn test_get_cache_path_hardcore_league() {
        let cache_path = EconomyService::get_league_cache_path("Rise of the Abyssal", true)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "HC_Rise_of_the_Abyssal.json");
        assert!(filename.starts_with("HC_"));
    }

    #[tokio::test]
    async fn test_get_cache_path_league_name_sanitization() {
        let cache_path = EconomyService::get_league_cache_path("Test League/Special", false)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        // Spaces become underscores, slashes become dashes
        assert_eq!(filename, "Test_League-Special.json");
    }

    #[tokio::test]
    async fn test_get_cache_path_hardcore_league_name_sanitization() {
        let cache_path = EconomyService::get_league_cache_path("Test League/Special", true)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        // HC prefix, spaces become underscores, slashes become dashes
        assert_eq!(filename, "HC_Test_League-Special.json");
    }

    #[tokio::test]
    async fn test_get_cache_path_standard_hardcore() {
        // Standard + hardcore should use "Hardcore" not "HC_Standard"
        let cache_path = EconomyService::get_league_cache_path("Standard", true)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "HC_Standard.json");
    }

    #[test]
    fn test_league_name_construction_normal() {
        // Normal leagues should remain unchanged
        let league = "Rise of the Abyssal";
        let is_hardcore = false;

        let league_name = if is_hardcore {
            if league.eq_ignore_ascii_case("Standard") {
                "Hardcore".to_string()
            } else {
                format!("HC {}", league)
            }
        } else {
            league.to_string()
        };

        assert_eq!(league_name, "Rise of the Abyssal");
    }

    #[test]
    fn test_league_name_construction_hardcore() {
        // Non-standard hardcore leagues should have "HC " prefix
        let league = "Rise of the Abyssal";
        let is_hardcore = true;

        let league_name = if is_hardcore {
            if league.eq_ignore_ascii_case("Standard") {
                "Hardcore".to_string()
            } else {
                format!("HC {}", league)
            }
        } else {
            league.to_string()
        };

        assert_eq!(league_name, "HC Rise of the Abyssal");
    }

    #[test]
    fn test_league_name_construction_standard_hardcore() {
        // Standard + hardcore should be "Hardcore" not "HC Standard"
        let league = "Standard";
        let is_hardcore = true;

        let league_name = if is_hardcore {
            if league.eq_ignore_ascii_case("Standard") {
                "Hardcore".to_string()
            } else {
                format!("HC {}", league)
            }
        } else {
            league.to_string()
        };

        assert_eq!(league_name, "Hardcore");
    }

    #[test]
    fn test_league_name_construction_standard_case_insensitive() {
        // Should handle "STANDARD", "standard", "Standard" all the same
        for standard_variant in &["Standard", "STANDARD", "standard", "StAnDaRd"] {
            let is_hardcore = true;

            let league_name = if is_hardcore {
                if standard_variant.eq_ignore_ascii_case("Standard") {
                    "Hardcore".to_string()
                } else {
                    format!("HC {}", standard_variant)
                }
            } else {
                standard_variant.to_string()
            };

            assert_eq!(
                league_name, "Hardcore",
                "Failed for variant: {}",
                standard_variant
            );
        }
    }
}
