#[cfg(test)]
mod tests {
    use crate::domain::log_analysis::models::*;

    // ============= ZoneChangeEvent Tests =============

    #[test]
    fn test_zone_change_event_serialization() {
        let event = ZoneChangeEvent {
            zone_name: "The Coast".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"zone_name\":\"The Coast\""));
        assert!(json.contains("\"timestamp\":\"2026-01-11T10:00:00Z\""));
    }

    #[test]
    fn test_zone_change_event_deserialization() {
        let json = r#"{"zone_name":"Clearfell","timestamp":"2026-01-11T12:00:00Z"}"#;
        let event: ZoneChangeEvent = serde_json::from_str(json).unwrap();

        assert_eq!(event.zone_name, "Clearfell");
        assert_eq!(event.timestamp, "2026-01-11T12:00:00Z");
    }

    // ============= ActChangeEvent Tests =============

    #[test]
    fn test_act_change_event_serialization() {
        let event = ActChangeEvent {
            act_name: "Act 2".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"act_name\":\"Act 2\""));
    }

    #[test]
    fn test_act_change_event_deserialization() {
        let json = r#"{"act_name":"Act 3","timestamp":"2026-01-11T15:00:00Z"}"#;
        let event: ActChangeEvent = serde_json::from_str(json).unwrap();

        assert_eq!(event.act_name, "Act 3");
        assert_eq!(event.timestamp, "2026-01-11T15:00:00Z");
    }

    // ============= HideoutChangeEvent Tests =============

    #[test]
    fn test_hideout_change_event_serialization() {
        let event = HideoutChangeEvent {
            hideout_name: "Celestial Hideout".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"hideout_name\":\"Celestial Hideout\""));
    }

    // ============= ServerConnectionEvent Tests =============

    #[test]
    fn test_server_connection_event_serialization() {
        let event = ServerConnectionEvent {
            ip_address: "192.168.1.1".to_string(),
            port: 6112,
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"ip_address\":\"192.168.1.1\""));
        assert!(json.contains("\"port\":6112"));
    }

    #[test]
    fn test_server_connection_event_deserialization() {
        let json = r#"{"ip_address":"10.0.0.1","port":8080,"timestamp":"2026-01-11T10:00:00Z"}"#;
        let event: ServerConnectionEvent = serde_json::from_str(json).unwrap();

        assert_eq!(event.ip_address, "10.0.0.1");
        assert_eq!(event.port, 8080);
    }

    // ============= CharacterLevelUpEvent Tests =============

    #[test]
    fn test_character_level_up_event_serialization() {
        let event = CharacterLevelUpEvent {
            character_name: "TestChar".to_string(),
            character_class: "Warrior".to_string(),
            new_level: 50,
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"character_name\":\"TestChar\""));
        assert!(json.contains("\"character_class\":\"Warrior\""));
        assert!(json.contains("\"new_level\":50"));
    }

    #[test]
    fn test_character_level_up_event_deserialization() {
        let json = r#"{"character_name":"MyChar","character_class":"Monk","new_level":75,"timestamp":"2026-01-11T20:00:00Z"}"#;
        let event: CharacterLevelUpEvent = serde_json::from_str(json).unwrap();

        assert_eq!(event.character_name, "MyChar");
        assert_eq!(event.character_class, "Monk");
        assert_eq!(event.new_level, 75);
    }

    // ============= CharacterDeathEvent Tests =============

    #[test]
    fn test_character_death_event_serialization() {
        let event = CharacterDeathEvent {
            character_name: "DeadChar".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"character_name\":\"DeadChar\""));
    }

    // ============= SceneChangeEvent Tests =============

    #[test]
    fn test_scene_change_event_zone() {
        let zone_event = SceneChangeEvent::Zone(ZoneChangeEvent {
            zone_name: "The Coast".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        });

        assert!(zone_event.is_zone());
        assert!(!zone_event.is_act());
        assert!(!zone_event.is_hideout());
        assert_eq!(zone_event.get_name(), "The Coast");
        assert_eq!(zone_event.get_timestamp(), "2026-01-11T10:00:00Z");
    }

    #[test]
    fn test_scene_change_event_act() {
        let act_event = SceneChangeEvent::Act(ActChangeEvent {
            act_name: "Act 2".to_string(),
            timestamp: "2026-01-11T12:00:00Z".to_string(),
        });

        assert!(!act_event.is_zone());
        assert!(act_event.is_act());
        assert!(!act_event.is_hideout());
        assert_eq!(act_event.get_name(), "Act 2");
        assert_eq!(act_event.get_timestamp(), "2026-01-11T12:00:00Z");
    }

    #[test]
    fn test_scene_change_event_hideout() {
        let hideout_event = SceneChangeEvent::Hideout(HideoutChangeEvent {
            hideout_name: "Celestial Hideout".to_string(),
            timestamp: "2026-01-11T14:00:00Z".to_string(),
        });

        assert!(!hideout_event.is_zone());
        assert!(!hideout_event.is_act());
        assert!(hideout_event.is_hideout());
        assert_eq!(hideout_event.get_name(), "Celestial Hideout");
    }

    #[test]
    fn test_scene_change_event_serialization_tagged() {
        let zone_event = SceneChangeEvent::Zone(ZoneChangeEvent {
            zone_name: "The Coast".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        });

        let json = serde_json::to_string(&zone_event).unwrap();
        assert!(json.contains("\"type\":\"Zone\""));
    }

    // ============= LogEvent Tests =============

    #[test]
    fn test_log_event_scene_change_serialization() {
        let event = LogEvent::SceneChange(SceneChangeEvent::Zone(ZoneChangeEvent {
            zone_name: "The Coast".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        }));

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event_type\":\"SceneChange\""));
    }

    #[test]
    fn test_log_event_server_connection_serialization() {
        let event = LogEvent::ServerConnection(ServerConnectionEvent {
            ip_address: "192.168.1.1".to_string(),
            port: 6112,
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        });

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event_type\":\"ServerConnection\""));
    }

    #[test]
    fn test_log_event_character_level_up_serialization() {
        let event = LogEvent::CharacterLevelUp(CharacterLevelUpEvent {
            character_name: "TestChar".to_string(),
            character_class: "Warrior".to_string(),
            new_level: 50,
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        });

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event_type\":\"CharacterLevelUp\""));
    }

    #[test]
    fn test_log_event_character_death_serialization() {
        let event = LogEvent::CharacterDeath(CharacterDeathEvent {
            character_name: "DeadChar".to_string(),
            timestamp: "2026-01-11T10:00:00Z".to_string(),
        });

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"event_type\":\"CharacterDeath\""));
    }

    // ============= LogAnalysisConfig Tests =============

    #[test]
    fn test_log_analysis_config_default() {
        let config: LogAnalysisConfig = Default::default();

        assert!(config.log_file_path.is_empty());
        assert_eq!(config.monitoring_interval_ms, 100);
        assert_eq!(config.max_file_size_mb, 100);
        assert_eq!(config.buffer_size, 1000);
        assert_eq!(config.session_gap_threshold_minutes, 30);
    }

    #[test]
    fn test_log_analysis_config_serialization() {
        let config = LogAnalysisConfig {
            log_file_path: "/path/to/Client.txt".to_string(),
            monitoring_interval_ms: 200,
            max_file_size_mb: 50,
            buffer_size: 500,
            session_gap_threshold_minutes: 60,
        };

        let json = serde_json::to_string(&config).unwrap();

        assert!(json.contains("\"log_file_path\":\"/path/to/Client.txt\""));
        assert!(json.contains("\"monitoring_interval_ms\":200"));
        assert!(json.contains("\"session_gap_threshold_minutes\":60"));
    }

    #[test]
    fn test_log_analysis_config_deserialization_with_default_threshold() {
        // Test that session_gap_threshold_minutes defaults to 30 when not specified
        let json = r#"{"log_file_path":"/test","monitoring_interval_ms":100,"max_file_size_mb":100,"buffer_size":1000}"#;
        let config: LogAnalysisConfig = serde_json::from_str(json).unwrap();

        assert_eq!(config.session_gap_threshold_minutes, 30);
    }

    // ============= LogAnalysisError Tests =============

    #[test]
    fn test_log_analysis_error_file_not_found() {
        let error = LogAnalysisError::FileNotFound {
            path: "/nonexistent/path".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("File not found"));
        assert!(message.contains("/nonexistent/path"));
    }

    #[test]
    fn test_log_analysis_error_file_access_error() {
        let error = LogAnalysisError::FileAccessError {
            message: "Permission denied".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("Permission denied"));
    }

    #[test]
    fn test_log_analysis_error_parsing_error() {
        let error = LogAnalysisError::ParsingError {
            message: "Invalid format".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("Parsing error"));
        assert!(message.contains("Invalid format"));
    }

    #[test]
    fn test_log_analysis_error_configuration_error() {
        let error = LogAnalysisError::ConfigurationError {
            message: "Invalid config".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("Configuration error"));
    }

    #[test]
    fn test_log_analysis_error_monitoring_error() {
        let error = LogAnalysisError::MonitoringError {
            message: "Watcher failed".to_string(),
        };
        let message = format!("{error}");
        assert!(message.contains("Monitoring error"));
    }
}
