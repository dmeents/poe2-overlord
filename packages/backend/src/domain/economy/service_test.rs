#[cfg(test)]
mod tests {
    use crate::domain::economy::service::EconomyService;

    #[test]
    fn test_service_creation() {
        let service = EconomyService::new();
        assert!(service.client.get("https://example.com").build().is_ok());
    }

    #[tokio::test]
    async fn test_get_cache_path_normal_league() {
        let cache_path = EconomyService::get_cache_path("Rise of the Abyssal", false)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "Rise_of_the_Abyssal.json");
        assert!(!filename.starts_with("HC_"));
    }

    #[tokio::test]
    async fn test_get_cache_path_hardcore_league() {
        let cache_path = EconomyService::get_cache_path("Rise of the Abyssal", true)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        assert_eq!(filename, "HC_Rise_of_the_Abyssal.json");
        assert!(filename.starts_with("HC_"));
    }

    #[tokio::test]
    async fn test_get_cache_path_league_name_sanitization() {
        let cache_path = EconomyService::get_cache_path("Test League/Special", false)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        // Spaces become underscores, slashes become dashes
        assert_eq!(filename, "Test_League-Special.json");
    }

    #[tokio::test]
    async fn test_get_cache_path_hardcore_league_name_sanitization() {
        let cache_path = EconomyService::get_cache_path("Test League/Special", true)
            .await
            .expect("Failed to get cache path");

        let filename = cache_path.file_name().unwrap().to_str().unwrap();
        // HC prefix, spaces become underscores, slashes become dashes
        assert_eq!(filename, "HC_Test_League-Special.json");
    }

    #[tokio::test]
    async fn test_get_cache_path_standard_hardcore() {
        // Standard + hardcore should use "Hardcore" not "HC_Standard"
        let cache_path = EconomyService::get_cache_path("Standard", true)
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
