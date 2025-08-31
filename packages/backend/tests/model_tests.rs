use app_lib::errors::AppError;
use app_lib::models::{
    events::{ActChangeEvent, HideoutChangeEvent, SceneChangeEvent, ZoneChangeEvent},
    AppConfig, LocationSession, LocationStats, LocationType, TimeTrackingEvent,
};
use chrono::Utc;
use std::error::Error;

#[test]
fn test_app_config_default() {
    let config = AppConfig::default();

    // Test that default config has valid values
    assert!(!config.poe_client_log_path.is_empty());
    assert_eq!(config.log_level, "info");
}

#[test]
fn test_app_config_serialization() {
    let config = AppConfig {
        poe_client_log_path: "/test/path".to_string(),
        log_level: "debug".to_string(),
    };

    // Test serialization to JSON
    let json = serde_json::to_string(&config);
    assert!(json.is_ok());

    let json_str = json.unwrap();
    assert!(json_str.contains("/test/path"));
    assert!(json_str.contains("debug"));

    // Test deserialization from JSON
    let deserialized: AppConfig = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized.poe_client_log_path, "/test/path");
    assert_eq!(deserialized.log_level, "debug");
}

#[test]
fn test_location_type_enum() {
    // Test that all location types are distinct
    let zone = LocationType::Zone;
    let act = LocationType::Act;
    let hideout = LocationType::Hideout;

    assert_ne!(zone, act);
    assert_ne!(act, hideout);
    assert_ne!(zone, hideout);

    // Test debug formatting
    assert_eq!(format!("{:?}", zone), "Zone");
    assert_eq!(format!("{:?}", act), "Act");
    assert_eq!(format!("{:?}", hideout), "Hideout");
}

#[test]
fn test_location_session_creation() {
    let now = Utc::now();
    let session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: now,
        exit_timestamp: None,
        duration_seconds: None,
    };

    assert_eq!(session.location_id, "test-zone-1");
    assert_eq!(session.location_name, "Test Zone");
    assert_eq!(session.location_type, LocationType::Zone);
    assert_eq!(session.entry_timestamp, now);
    assert!(session.exit_timestamp.is_none());
    assert!(session.duration_seconds.is_none());
}

#[test]
fn test_location_session_completion() {
    let entry_time = Utc::now();
    let exit_time = entry_time + chrono::Duration::seconds(300); // 5 minutes later

    let session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: entry_time,
        exit_timestamp: Some(exit_time),
        duration_seconds: Some(300),
    };

    assert_eq!(session.location_id, "test-zone-1");
    assert_eq!(session.location_name, "Test Zone");
    assert_eq!(session.location_type, LocationType::Zone);
    assert_eq!(session.entry_timestamp, entry_time);
    assert_eq!(session.exit_timestamp, Some(exit_time));
    assert_eq!(session.duration_seconds, Some(300));
}

#[test]
fn test_location_stats_creation() {
    let now = Utc::now();
    let stats = LocationStats {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        total_visits: 5,
        total_time_seconds: 1500,
        average_session_seconds: 300.0,
        last_visited: Some(now),
    };

    assert_eq!(stats.location_id, "test-zone-1");
    assert_eq!(stats.location_name, "Test Zone");
    assert_eq!(stats.location_type, LocationType::Zone);
    assert_eq!(stats.total_visits, 5);
    assert_eq!(stats.total_time_seconds, 1500);
    assert_eq!(stats.average_session_seconds, 300.0);
    assert_eq!(stats.last_visited, Some(now));
}

#[test]
fn test_location_stats_default_values() {
    let stats = LocationStats {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        total_visits: 0,
        total_time_seconds: 0,
        average_session_seconds: 0.0,
        last_visited: None,
    };

    assert_eq!(stats.total_visits, 0);
    assert_eq!(stats.total_time_seconds, 0);
    assert_eq!(stats.average_session_seconds, 0.0);
    assert!(stats.last_visited.is_none());
}

#[test]
fn test_scene_change_events() {
    let now = Utc::now();

    // Test Zone event
    let zone_event = SceneChangeEvent::Zone(ZoneChangeEvent {
        zone_name: "Test Zone".to_string(),
        timestamp: now.to_rfc3339(),
    });

    match zone_event {
        SceneChangeEvent::Zone(event) => {
            assert_eq!(event.zone_name, "Test Zone");
            assert_eq!(event.timestamp, now.to_rfc3339());
        }
        _ => panic!("Expected zone event"),
    }

    // Test Act event
    let act_event = SceneChangeEvent::Act(ActChangeEvent {
        act_name: "Act 1".to_string(),
        timestamp: now.to_rfc3339(),
    });

    match act_event {
        SceneChangeEvent::Act(event) => {
            assert_eq!(event.act_name, "Act 1");
            assert_eq!(event.timestamp, now.to_rfc3339());
        }
        _ => panic!("Expected act event"),
    }

    // Test Hideout event
    let hideout_event = SceneChangeEvent::Hideout(HideoutChangeEvent {
        hideout_name: "Test Hideout".to_string(),
        timestamp: now.to_rfc3339(),
    });

    match hideout_event {
        SceneChangeEvent::Hideout(event) => {
            assert_eq!(event.hideout_name, "Test Hideout");
            assert_eq!(event.timestamp, now.to_rfc3339());
        }
        _ => panic!("Expected hideout event"),
    }
}

#[test]
fn test_time_tracking_events() {
    let now = Utc::now();
    let session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: now,
        exit_timestamp: None,
        duration_seconds: None,
    };

    // Test SessionStarted event
    let started_event = TimeTrackingEvent::SessionStarted(session.clone());

    match started_event {
        TimeTrackingEvent::SessionStarted(event) => {
            assert_eq!(event.location_id, "test-zone-1");
            assert_eq!(event.location_name, "Test Zone");
            assert_eq!(event.location_type, LocationType::Zone);
        }
        _ => panic!("Expected session started event"),
    }

    // Test SessionEnded event
    let ended_session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: now,
        exit_timestamp: Some(now + chrono::Duration::seconds(300)),
        duration_seconds: Some(300),
    };

    let ended_event = TimeTrackingEvent::SessionEnded(ended_session.clone());

    match ended_event {
        TimeTrackingEvent::SessionEnded(event) => {
            assert_eq!(event.location_id, "test-zone-1");
            assert_eq!(event.location_name, "Test Zone");
            assert_eq!(event.location_type, LocationType::Zone);
            assert!(event.exit_timestamp.is_some());
            assert_eq!(event.duration_seconds, Some(300));
        }
        _ => panic!("Expected session ended event"),
    }

    // Test StatsUpdated event
    let stats = LocationStats {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        total_visits: 1,
        total_time_seconds: 300,
        average_session_seconds: 300.0,
        last_visited: Some(now),
    };

    let stats_event = TimeTrackingEvent::StatsUpdated(stats.clone());

    match stats_event {
        TimeTrackingEvent::StatsUpdated(event) => {
            assert_eq!(event.location_id, "test-zone-1");
            assert_eq!(event.location_name, "Test Zone");
            assert_eq!(event.location_type, LocationType::Zone);
            assert_eq!(event.total_visits, 1);
            assert_eq!(event.total_time_seconds, 300);
        }
        _ => panic!("Expected stats updated event"),
    }
}

#[test]
fn test_event_serialization() {
    let now = Utc::now();

    // Test ZoneChangeEvent serialization
    let zone_event = ZoneChangeEvent {
        zone_name: "Test Zone".to_string(),
        timestamp: now.to_rfc3339(),
    };

    let json = serde_json::to_string(&zone_event);
    assert!(json.is_ok());

    let json_str = json.unwrap();
    assert!(json_str.contains("Test Zone"));
    assert!(json_str.contains(&now.to_rfc3339()));

    // Test deserialization
    let deserialized: ZoneChangeEvent = serde_json::from_str(&json_str).unwrap();
    assert_eq!(deserialized.zone_name, "Test Zone");
    assert_eq!(deserialized.timestamp, now.to_rfc3339());
}

#[test]
fn test_error_types() {
    // Test ConfigError
    let config_error = AppError::Config("Configuration file not found".to_string());
    let error_string = format!("{}", config_error);
    assert!(error_string.contains("Configuration file not found"));

    // Test LogMonitorError
    let log_error = AppError::LogMonitor("Log file not found".to_string());
    let error_string = format!("{}", log_error);
    assert!(error_string.contains("Log file not found"));

    // Test ProcessMonitorError
    let process_error = AppError::ProcessMonitor("Process not found".to_string());
    let error_string = format!("{}", process_error);
    assert!(error_string.contains("Process not found"));

    // Test FileSystemError
    let file_error = AppError::FileSystem("File read failed".to_string());
    let error_string = format!("{}", file_error);
    assert!(error_string.contains("File read failed"));

    // Test SerializationError
    let serialization_error = AppError::Serialization("JSON parse failed".to_string());
    let error_string = format!("{}", serialization_error);
    assert!(error_string.contains("JSON parse failed"));

    // Test InternalError
    let internal_error = AppError::Internal("Internal error occurred".to_string());
    let error_string = format!("{}", internal_error);
    assert!(error_string.contains("Internal error occurred"));
}

#[test]
fn test_error_source() {
    // Test that errors implement std::error::Error trait
    let config_error = AppError::Config("Test error".to_string());

    // This should compile if AppError implements std::error::Error
    let _source = config_error.source();

    // Test error conversion
    let error_string = config_error.to_string();
    assert!(error_string.contains("Test error"));
}

#[test]
fn test_model_cloning() {
    let session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: Utc::now(),
        exit_timestamp: None,
        duration_seconds: None,
    };

    // Test cloning
    let cloned_session = session.clone();
    assert_eq!(session.location_id, cloned_session.location_id);
    assert_eq!(session.location_name, cloned_session.location_name);
    assert_eq!(session.location_type, cloned_session.location_type);
    assert_eq!(session.entry_timestamp, cloned_session.entry_timestamp);
    assert_eq!(session.exit_timestamp, cloned_session.exit_timestamp);
    assert_eq!(session.duration_seconds, cloned_session.duration_seconds);
}

#[test]
fn test_model_debug_formatting() {
    let session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: Utc::now(),
        exit_timestamp: None,
        duration_seconds: None,
    };

    // Test debug formatting
    let debug_str = format!("{:?}", session);
    assert!(debug_str.contains("test-zone-1"));
    assert!(debug_str.contains("Test Zone"));
    assert!(debug_str.contains("Zone"));
}
