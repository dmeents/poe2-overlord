use app_lib::models::events::SceneChangeEvent;
use app_lib::parsers::{
    config::ParsersConfig,
    manager::LogParserManager,
    scene_change_parser::SceneChangeParser,
    traits::LogParser,
};
use app_lib::services::player_location_manager::PlayerLocationManager;
use std::sync::Arc;

#[test]
fn test_default_parser_config() {
    let config = ParsersConfig::default();

    assert!(config
        .scene_change
        .patterns
        .contains(&"[SCENE] Set Source [".to_string()));
    assert!(config
        .scene_change
        .patterns
        .contains(&"[SCENE] Load Source [".to_string()));

    let scene_types = config.scene_change.scene_types.as_ref().unwrap();
    assert!(scene_types.hideout.contains(&"hideout".to_string()));
    assert!(scene_types.act.contains(&"act ".to_string()));
    assert!(scene_types.zone.contains(&"*".to_string()));
}

#[tokio::test]
async fn test_parser_manager_with_default_config() {
    let state_manager = Arc::new(PlayerLocationManager::new());
    let manager = LogParserManager::new(state_manager);

    let active_parsers = manager.get_active_parsers();
    assert!(active_parsers.contains(&"scene_change"));

    // Test parsing a valid line
    let line = "[SCENE] Set Source [The Coast]";
    let event = manager.parse_line(line).await;

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Zone(zone_event)) = event {
        assert_eq!(zone_event.zone_name, "The Coast");
    } else {
        panic!("Expected Zone event");
    }
}

#[tokio::test]
async fn test_parser_manager_always_has_scene_change_parser() {
    let state_manager = Arc::new(PlayerLocationManager::new());
    let manager = LogParserManager::new(state_manager);

    let active_parsers = manager.get_active_parsers();
    assert!(active_parsers.contains(&"scene_change"));

    // Scene change parser is always active and enabled
    let line = "[SCENE] Set Source [The Coast]";
    let event = manager.parse_line(line).await;
    assert!(event.is_some());
}

#[test]
fn test_scene_change_parser_with_default_config() {
    let parser = SceneChangeParser::new();

    // Test hideout detection with default config
    let line = "[SCENE] Set Source [My Hideout]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Hideout(hideout_event)) = event {
        assert_eq!(hideout_event.hideout_name, "My Hideout");
    } else {
        panic!("Expected Hideout event");
    }

    // Test act detection with default config
    let line = "[SCENE] Set Source [Act 1]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Act(act_event)) = event {
        assert_eq!(act_event.act_name, "Act 1");
    } else {
        panic!("Expected Act event");
    }
}
