use app_lib::parsers::{
    config::SceneTypeConfig,
    scene_type_detector::{SceneType, SceneTypeDetector},
};

fn create_test_config() -> SceneTypeConfig {
    SceneTypeConfig {
        hideout: vec!["hideout".to_string(), "sanctuary".to_string()],
        act: vec![
            "act ".to_string(),
            "atlas".to_string(),
            "interlude".to_string(),
        ],
        zone: vec!["*".to_string()],
    }
}

#[test]
fn test_detect_hideout() {
    let detector = SceneTypeDetector::new(create_test_config());

    assert_eq!(detector.detect_scene_type("My Hideout"), SceneType::Hideout);
    assert_eq!(detector.detect_scene_type("Sanctuary"), SceneType::Hideout);
    assert_eq!(
        detector.detect_scene_type("player_hideout"),
        SceneType::Hideout
    );
}

#[test]
fn test_detect_act() {
    let detector = SceneTypeDetector::new(create_test_config());

    assert_eq!(detector.detect_scene_type("Act 1"), SceneType::Act);
    assert_eq!(detector.detect_scene_type("Atlas"), SceneType::Act);
    assert_eq!(detector.detect_scene_type("Interlude"), SceneType::Act);
}

#[test]
fn test_detect_zone() {
    let detector = SceneTypeDetector::new(create_test_config());

    assert_eq!(detector.detect_scene_type("Forest"), SceneType::Zone);
    assert_eq!(detector.detect_scene_type("Town Square"), SceneType::Zone);
    assert_eq!(
        detector.detect_scene_type("Dungeon Level 1"),
        SceneType::Zone
    );
}

#[test]
fn test_create_hideout_event() {
    let detector = SceneTypeDetector::new(create_test_config());
    let event = detector.create_scene_change_event("My Hideout");

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
    let detector = SceneTypeDetector::new(create_test_config());
    let event = detector.create_scene_change_event("Act 1");

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
    let detector = SceneTypeDetector::new(create_test_config());
    let event = detector.create_scene_change_event("Forest");

    match event {
        app_lib::models::events::SceneChangeEvent::Zone(zone_event) => {
            assert_eq!(zone_event.zone_name, "Forest");
            assert!(!zone_event.timestamp.is_empty());
        }
        _ => panic!("Expected ZoneChangeEvent"),
    }
}
