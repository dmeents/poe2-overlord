#[cfg(test)]
mod tests {
    use crate::domain::configuration::models::*;
    use std::path::PathBuf;

    // ============= ZoneRefreshInterval Tests =============

    #[test]
    fn test_zone_refresh_interval_to_seconds_five_minutes() {
        let interval = ZoneRefreshInterval::FiveMinutes;
        assert_eq!(interval.to_seconds(), 300);
    }

    #[test]
    fn test_zone_refresh_interval_to_seconds_one_hour() {
        let interval = ZoneRefreshInterval::OneHour;
        assert_eq!(interval.to_seconds(), 3600);
    }

    #[test]
    fn test_zone_refresh_interval_to_seconds_twelve_hours() {
        let interval = ZoneRefreshInterval::TwelveHours;
        assert_eq!(interval.to_seconds(), 43200);
    }

    #[test]
    fn test_zone_refresh_interval_to_seconds_twenty_four_hours() {
        let interval = ZoneRefreshInterval::TwentyFourHours;
        assert_eq!(interval.to_seconds(), 86400);
    }

    #[test]
    fn test_zone_refresh_interval_to_seconds_three_days() {
        let interval = ZoneRefreshInterval::ThreeDays;
        assert_eq!(interval.to_seconds(), 259200);
    }

    #[test]
    fn test_zone_refresh_interval_to_seconds_seven_days() {
        let interval = ZoneRefreshInterval::SevenDays;
        assert_eq!(interval.to_seconds(), 604800);
    }

    #[test]
    fn test_zone_refresh_interval_all_options() {
        let options = ZoneRefreshInterval::all_options();
        assert_eq!(options.len(), 6);
        assert!(options.contains(&ZoneRefreshInterval::FiveMinutes));
        assert!(options.contains(&ZoneRefreshInterval::OneHour));
        assert!(options.contains(&ZoneRefreshInterval::TwelveHours));
        assert!(options.contains(&ZoneRefreshInterval::TwentyFourHours));
        assert!(options.contains(&ZoneRefreshInterval::ThreeDays));
        assert!(options.contains(&ZoneRefreshInterval::SevenDays));
    }

    #[test]
    fn test_zone_refresh_interval_labels() {
        assert_eq!(ZoneRefreshInterval::FiveMinutes.label(), "5 Minutes");
        assert_eq!(ZoneRefreshInterval::OneHour.label(), "1 Hour");
        assert_eq!(ZoneRefreshInterval::TwelveHours.label(), "12 Hours");
        assert_eq!(ZoneRefreshInterval::TwentyFourHours.label(), "24 Hours");
        assert_eq!(ZoneRefreshInterval::ThreeDays.label(), "3 Days");
        assert_eq!(ZoneRefreshInterval::SevenDays.label(), "7 Days");
    }

    #[test]
    fn test_zone_refresh_interval_from_str_valid() {
        assert_eq!(
            ZoneRefreshInterval::from_str("FiveMinutes"),
            Some(ZoneRefreshInterval::FiveMinutes)
        );
        assert_eq!(
            ZoneRefreshInterval::from_str("OneHour"),
            Some(ZoneRefreshInterval::OneHour)
        );
        assert_eq!(
            ZoneRefreshInterval::from_str("TwelveHours"),
            Some(ZoneRefreshInterval::TwelveHours)
        );
        assert_eq!(
            ZoneRefreshInterval::from_str("TwentyFourHours"),
            Some(ZoneRefreshInterval::TwentyFourHours)
        );
        assert_eq!(
            ZoneRefreshInterval::from_str("ThreeDays"),
            Some(ZoneRefreshInterval::ThreeDays)
        );
        assert_eq!(
            ZoneRefreshInterval::from_str("SevenDays"),
            Some(ZoneRefreshInterval::SevenDays)
        );
    }

    #[test]
    fn test_zone_refresh_interval_from_str_invalid() {
        assert_eq!(ZoneRefreshInterval::from_str("invalid"), None);
        assert_eq!(ZoneRefreshInterval::from_str(""), None);
        assert_eq!(ZoneRefreshInterval::from_str("5 Minutes"), None);
    }

    #[test]
    fn test_zone_refresh_interval_default() {
        let interval: ZoneRefreshInterval = Default::default();
        assert_eq!(interval, ZoneRefreshInterval::SevenDays);
    }

    #[test]
    fn test_zone_refresh_interval_display() {
        assert_eq!(
            format!("{}", ZoneRefreshInterval::FiveMinutes),
            "5 Minutes"
        );
        assert_eq!(format!("{}", ZoneRefreshInterval::OneHour), "1 Hour");
    }

    #[test]
    fn test_zone_refresh_interval_serialization() {
        let interval = ZoneRefreshInterval::TwelveHours;
        let json = serde_json::to_string(&interval).unwrap();
        assert_eq!(json, "\"TwelveHours\"");
    }

    #[test]
    fn test_zone_refresh_interval_deserialization() {
        let interval: ZoneRefreshInterval =
            serde_json::from_str("\"TwentyFourHours\"").unwrap();
        assert_eq!(interval, ZoneRefreshInterval::TwentyFourHours);
    }

    // ============= AppConfig Tests =============

    #[test]
    fn test_app_config_new() {
        let config = AppConfig::new();
        assert_eq!(config.log_level, "info");
        assert_eq!(
            config.zone_refresh_interval,
            ZoneRefreshInterval::SevenDays
        );
    }

    #[test]
    fn test_app_config_with_values() {
        let config = AppConfig::with_values("/custom/path".to_string(), "debug".to_string());
        assert_eq!(config.poe_client_log_path, "/custom/path");
        assert_eq!(config.log_level, "debug");
        assert_eq!(
            config.zone_refresh_interval,
            ZoneRefreshInterval::SevenDays
        );
    }

    #[test]
    fn test_app_config_validate_valid() {
        let config = AppConfig::with_values("/some/path".to_string(), "info".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_app_config_validate_all_log_levels() {
        let valid_levels = ["trace", "debug", "info", "warn", "warning", "error"];
        for level in valid_levels {
            let config = AppConfig::with_values("/path".to_string(), level.to_string());
            assert!(
                config.validate().is_ok(),
                "Expected log level '{}' to be valid",
                level
            );
        }
    }

    #[test]
    fn test_app_config_validate_log_level_case_insensitive() {
        let config = AppConfig::with_values("/path".to_string(), "INFO".to_string());
        assert!(config.validate().is_ok());

        let config = AppConfig::with_values("/path".to_string(), "Debug".to_string());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_app_config_validate_invalid_log_level() {
        let config = AppConfig::with_values("/path".to_string(), "invalid".to_string());
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid log level"));
    }

    #[test]
    fn test_app_config_validate_empty_path() {
        let config = AppConfig::with_values("".to_string(), "info".to_string());
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_app_config_validate_whitespace_path() {
        let config = AppConfig::with_values("   ".to_string(), "info".to_string());
        let result = config.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_app_config_default_poe_path_not_empty() {
        let path = AppConfig::get_default_poe_client_log_path();
        assert!(!path.is_empty());
    }

    #[test]
    fn test_app_config_is_using_default_poe_path() {
        let config = AppConfig::new();
        assert!(config.is_using_default_poe_path());

        let custom_config = AppConfig::with_values("/custom/path".to_string(), "info".to_string());
        assert!(!custom_config.is_using_default_poe_path());
    }

    #[test]
    fn test_app_config_reset_poe_path_to_default() {
        let mut config = AppConfig::with_values("/custom/path".to_string(), "info".to_string());
        assert!(!config.is_using_default_poe_path());

        config.reset_poe_path_to_default();
        assert!(config.is_using_default_poe_path());
    }

    #[test]
    fn test_app_config_serialization_roundtrip() {
        let config = AppConfig {
            poe_client_log_path: "/test/path".to_string(),
            log_level: "debug".to_string(),
            zone_refresh_interval: ZoneRefreshInterval::OneHour,
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AppConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.poe_client_log_path, config.poe_client_log_path);
        assert_eq!(deserialized.log_level, config.log_level);
        assert_eq!(
            deserialized.zone_refresh_interval,
            config.zone_refresh_interval
        );
    }

    // ============= ConfigurationChangedEvent Tests =============

    #[test]
    fn test_configuration_changed_event_new() {
        let old_config = AppConfig::with_values("/old/path".to_string(), "info".to_string());
        let new_config = AppConfig::with_values("/new/path".to_string(), "debug".to_string());

        let event = ConfigurationChangedEvent::new(new_config.clone(), old_config.clone());

        assert_eq!(event.new_config.poe_client_log_path, "/new/path");
        assert_eq!(event.previous_config.poe_client_log_path, "/old/path");
        assert_eq!(event.new_config.log_level, "debug");
        assert_eq!(event.previous_config.log_level, "info");
    }

    // ============= ConfigurationValidationResult Tests =============

    #[test]
    fn test_validation_result_valid() {
        let result = ConfigurationValidationResult::valid();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validation_result_invalid() {
        let result = ConfigurationValidationResult::invalid(vec!["Error 1".to_string()]);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0], "Error 1");
    }

    #[test]
    fn test_validation_result_add_error() {
        let mut result = ConfigurationValidationResult::valid();
        assert!(result.is_valid);

        result.add_error("Something went wrong".to_string());

        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0], "Something went wrong");
    }

    #[test]
    fn test_validation_result_add_multiple_errors() {
        let mut result = ConfigurationValidationResult::valid();
        result.add_error("Error 1".to_string());
        result.add_error("Error 2".to_string());
        result.add_error("Error 3".to_string());

        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 3);
    }

    // ============= ConfigurationFileInfo Tests =============

    #[test]
    fn test_configuration_file_info_nonexistent_file() {
        let info =
            ConfigurationFileInfo::new(PathBuf::from("/nonexistent/path/config.json"));
        assert!(!info.exists);
        assert!(info.size.is_none());
        assert!(info.last_modified.is_none());
    }

    #[test]
    fn test_configuration_file_info_serialization() {
        let info = ConfigurationFileInfo {
            path: PathBuf::from("/test/path"),
            exists: true,
            size: Some(1024),
            last_modified: None,
        };

        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"exists\":true"));
        assert!(json.contains("\"size\":1024"));
    }
}
