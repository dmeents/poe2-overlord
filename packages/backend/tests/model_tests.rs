// Model tests for POE2 Overlord backend
// Tests for scene change events, location sessions, and location types

use app_lib::models::{events::SceneChangeEvent, time_tracking::LocationSession, LocationType};
use chrono::Utc;

#[test]
fn test_scene_change_event_zone() {
    let zone_event = app_lib::models::events::ZoneChangeEvent {
        zone_name: "Test Zone".to_string(),
        timestamp: "2024-01-01T00:00:00Z".to_string(),
    };

    let event = SceneChangeEvent::Zone(zone_event);

    assert!(event.is_zone());
    assert!(!event.is_act());
    assert!(!event.is_hideout());
    assert_eq!(event.get_name(), "Test Zone");
    assert_eq!(event.get_timestamp(), "2024-01-01T00:00:00Z");
}

#[test]
fn test_scene_change_event_act() {
    let act_event = app_lib::models::events::ActChangeEvent {
        act_name: "Test Act".to_string(),
        timestamp: "2024-01-01T00:00:00Z".to_string(),
    };

    let event = SceneChangeEvent::Act(act_event);

    assert!(!event.is_zone());
    assert!(event.is_act());
    assert!(!event.is_hideout());
    assert_eq!(event.get_name(), "Test Act");
    assert_eq!(event.get_timestamp(), "2024-01-01T00:00:00Z");
}

#[test]
fn test_scene_change_event_hideout() {
    let hideout_event = app_lib::models::events::HideoutChangeEvent {
        hideout_name: "Test Hideout".to_string(),
        timestamp: "2024-01-01T00:00:00Z".to_string(),
    };

    let event = SceneChangeEvent::Hideout(hideout_event);

    assert!(!event.is_zone());
    assert!(!event.is_act());
    assert!(event.is_hideout());
    assert_eq!(event.get_name(), "Test Hideout");
    assert_eq!(event.get_timestamp(), "2024-01-01T00:00:00Z");
}

#[test]
fn test_location_session_creation() {
    let session = LocationSession {
        location_id: "test-zone-1".to_string(),
        location_name: "Test Zone".to_string(),
        location_type: LocationType::Zone,
        entry_timestamp: Utc::now(),
        exit_timestamp: None,
        duration_seconds: None,
    };

    assert_eq!(session.location_id, "test-zone-1");
    assert_eq!(session.location_name, "Test Zone");
    assert_eq!(session.location_type, LocationType::Zone);
    assert!(session.exit_timestamp.is_none());
    assert!(session.duration_seconds.is_none());
}

#[test]
fn test_location_type_equality() {
    let zone_type = LocationType::Zone;
    let act_type = LocationType::Act;
    let hideout_type = LocationType::Hideout;

    assert_eq!(zone_type, LocationType::Zone);
    assert_eq!(act_type, LocationType::Act);
    assert_eq!(hideout_type, LocationType::Hideout);
    assert_ne!(zone_type, act_type);
    assert_ne!(zone_type, hideout_type);
    assert_ne!(act_type, hideout_type);
}

#[test]
fn test_location_type_hash() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(LocationType::Zone, "zone_value");
    map.insert(LocationType::Act, "act_value");
    map.insert(LocationType::Hideout, "hideout_value");

    assert_eq!(map.get(&LocationType::Zone), Some(&"zone_value"));
    assert_eq!(map.get(&LocationType::Act), Some(&"act_value"));
    assert_eq!(map.get(&LocationType::Hideout), Some(&"hideout_value"));
}

#[test]
fn test_scene_change_event_serialization() {
    let zone_event = app_lib::models::events::ZoneChangeEvent {
        zone_name: "Test Zone".to_string(),
        timestamp: "2024-01-01T00:00:00Z".to_string(),
    };

    let event = SceneChangeEvent::Zone(zone_event);

    let json = serde_json::to_string(&event).unwrap();
    println!("Serialized SceneChangeEvent::Zone: {}", json);

    // The actual structure is: {"type":"Zone","zone_name":"Test Zone","timestamp":"2024-01-01T00:00:00Z"}
    assert!(json.contains("\"type\":\"Zone\""));
    assert!(json.contains("\"zone_name\":\"Test Zone\""));
    assert!(json.contains("\"timestamp\":\"2024-01-01T00:00:00Z\""));
}
