use app_lib::models::scene_type::SceneType;
use app_lib::parsers::config::ParsersConfig;
use app_lib::services::location_tracker::{LocationTracker, SceneTypeConfig};

fn create_test_config() -> SceneTypeConfig {
    // Use the same config as the parser system for consistency
    let parser_config = ParsersConfig::default();
    SceneTypeConfig {
        hideout_keywords: parser_config.hideout_keywords().clone(),
        act_keywords: parser_config.act_keywords().clone(),
        zone_keywords: parser_config.zone_keywords().clone(),
    }
}

#[test]
fn test_detect_hideout() {
    let tracker = LocationTracker::with_config(create_test_config());

    assert_eq!(tracker.detect_scene_type("My Hideout"), SceneType::Hideout);
    assert_eq!(tracker.detect_scene_type("Sanctuary"), SceneType::Zone); // "Sanctuary" doesn't contain "hideout" keyword
    assert_eq!(
        tracker.detect_scene_type("player_hideout"),
        SceneType::Hideout
    );
}

#[test]
fn test_detect_act() {
    let tracker = LocationTracker::with_config(create_test_config());

    assert_eq!(tracker.detect_scene_type("Act 1"), SceneType::Act);
    assert_eq!(tracker.detect_scene_type("Atlas"), SceneType::Act);
    assert_eq!(tracker.detect_scene_type("Interlude"), SceneType::Act);
}

#[test]
fn test_detect_zone() {
    let tracker = LocationTracker::with_config(create_test_config());

    assert_eq!(tracker.detect_scene_type("Forest"), SceneType::Zone);
    assert_eq!(tracker.detect_scene_type("Town Square"), SceneType::Zone);
    assert_eq!(
        tracker.detect_scene_type("Dungeon Level 1"),
        SceneType::Zone
    );
}

#[test]
fn test_create_hideout_event() {
    let tracker = LocationTracker::with_config(create_test_config());
    let event = tracker.create_scene_change_event("My Hideout");

    match event {
        app_lib::models::events::SceneChangeEvent::Hideout(hideout_event) => {
            assert_eq!(hideout_event.hideout_name, "My Hideout");
            assert!(!hideout_event.timestamp.is_empty());
        }
        _ => panic!("Expected HideoutChangeEvent"),
    }
}

#[test]
fn test_create_act_event() {
    let tracker = LocationTracker::with_config(create_test_config());
    let event = tracker.create_scene_change_event("Act 1");

    match event {
        app_lib::models::events::SceneChangeEvent::Act(act_event) => {
            assert_eq!(act_event.act_name, "Act 1");
            assert!(!act_event.timestamp.is_empty());
        }
        _ => panic!("Expected ActChangeEvent"),
    }
}

#[test]
fn test_create_zone_event() {
    let tracker = LocationTracker::with_config(create_test_config());
    let event = tracker.create_scene_change_event("Forest");

    match event {
        app_lib::models::events::SceneChangeEvent::Zone(zone_event) => {
            assert_eq!(zone_event.zone_name, "Forest");
            assert!(!zone_event.timestamp.is_empty());
        }
        _ => panic!("Expected ZoneChangeEvent"),
    }
}
