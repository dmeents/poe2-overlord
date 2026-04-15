#[cfg(test)]
mod tests {
    use crate::domain::game_monitoring::models::*;

    // ============= GameProcessStatus Tests =============

    #[test]
    fn test_game_process_status_new() {
        let status = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);

        assert_eq!(status.name, "poe2.exe");
        assert_eq!(status.pid, 1234);
        assert!(status.running);
    }

    #[test]
    fn test_game_process_status_not_running() {
        let status = GameProcessStatus::not_running();

        assert_eq!(status.name, "Not Found");
        assert_eq!(status.pid, 0);
        assert!(!status.running);
    }

    #[test]
    fn test_game_process_status_is_running() {
        let running = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);
        let stopped = GameProcessStatus::new("poe2.exe".to_string(), 1234, false);

        assert!(running.is_running());
        assert!(!stopped.is_running());
    }

    #[test]
    fn test_game_process_status_is_state_change_different() {
        let previous = GameProcessStatus::new("poe2.exe".to_string(), 1234, false);
        let current = GameProcessStatus::new("poe2.exe".to_string(), 5678, true);

        assert!(current.is_state_change(&previous));
    }

    #[test]
    fn test_game_process_status_is_state_change_same() {
        let previous = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);
        let current = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);

        assert!(!current.is_state_change(&previous));
    }

    #[test]
    fn test_game_process_status_is_state_change_stopped_to_running() {
        let previous = GameProcessStatus::not_running();
        let current = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);

        assert!(current.is_state_change(&previous));
    }

    #[test]
    fn test_game_process_status_is_state_change_running_to_stopped() {
        let previous = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);
        let current = GameProcessStatus::not_running();

        assert!(current.is_state_change(&previous));
    }

    #[test]
    fn test_game_process_status_equality() {
        let status1 = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);
        let status2 = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);

        // Note: detected_at will differ slightly, so we compare other fields
        assert_eq!(status1.name, status2.name);
        assert_eq!(status1.pid, status2.pid);
        assert_eq!(status1.running, status2.running);
    }

    #[test]
    fn test_game_process_status_serialization() {
        let status = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);
        let json = serde_json::to_string(&status).unwrap();

        assert!(json.contains("\"name\":\"poe2.exe\""));
        assert!(json.contains("\"pid\":1234"));
        assert!(json.contains("\"running\":true"));
    }

    #[test]
    fn test_game_process_status_deserialization() {
        // detected_at is now an RFC3339 string for frontend compatibility
        let json = r#"{"name":"poe2.exe","pid":1234,"running":true,"detected_at":"2024-01-01T00:00:00+00:00"}"#;
        let status: GameProcessStatus = serde_json::from_str(json).unwrap();

        assert_eq!(status.name, "poe2.exe");
        assert_eq!(status.pid, 1234);
        assert!(status.running);
        assert_eq!(status.detected_at, "2024-01-01T00:00:00+00:00");
    }

    #[test]
    fn test_game_process_status_serializes_timestamp_as_string() {
        let status = GameProcessStatus::new("poe2.exe".to_string(), 1234, true);
        let json = serde_json::to_string(&status).unwrap();

        // Timestamp should be an RFC3339 string, not a SystemTime object
        assert!(json.contains("\"detected_at\":\""));
        assert!(!json.contains("secs_since_epoch")); // Old SystemTime format
    }

    // ============= GameMonitoringConfig Tests =============

    #[test]
    fn test_game_monitoring_config_default() {
        let config: GameMonitoringConfig = Default::default();

        assert_eq!(config.detection_interval_seconds, 3);
        assert_eq!(config.monitoring_interval_seconds, 5);
        assert!(!config.process_names.is_empty());
    }

    #[test]
    fn test_game_monitoring_config_default_process_names() {
        let config = GameMonitoringConfig::default();

        // Should include common POE process names
        assert!(config.process_names.contains(&"pathofexile2".to_string()));
        assert!(config.process_names.contains(&"poe2".to_string()));
        assert!(config
            .process_names
            .contains(&"pathofexile2.exe".to_string()));
    }

    #[test]
    fn test_game_monitoring_config_serialization() {
        let config = GameMonitoringConfig {
            detection_interval_seconds: 5,
            monitoring_interval_seconds: 120,
            process_names: vec!["poe2".to_string()],
        };

        let json = serde_json::to_string(&config).unwrap();

        assert!(json.contains("\"detection_interval_seconds\":5"));
        assert!(json.contains("\"monitoring_interval_seconds\":120"));
        assert!(json.contains("\"process_names\":[\"poe2\"]"));
    }

    #[test]
    fn test_game_monitoring_config_deserialization() {
        let json = r#"{"detection_interval_seconds":10,"monitoring_interval_seconds":30,"process_names":["test.exe"]}"#;
        let config: GameMonitoringConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.detection_interval_seconds, 10);
        assert_eq!(config.monitoring_interval_seconds, 30);
        assert_eq!(config.process_names, vec!["test.exe"]);
    }
}
