use app_lib::parsers::{
    config::ParsersConfig,
    manager::LogParserManager,
    scene_change_parser::{SceneChangeParser, LogParser},
};
use app_lib::models::events::SceneChangeEvent;
use app_lib::services::player_location_manager::PlayerLocationManager;
use std::sync::Arc;

#[test]
fn test_scene_change_parser_creation() {
    let parser = SceneChangeParser::new();
    // Test that the parser can handle basic input
    assert!(parser.should_parse("[SCENE] Set Source [Test Zone]"));
}

#[test]
fn test_scene_change_parser_should_parse() {
    let parser = SceneChangeParser::new();
    
    // Test valid scene change lines
    assert!(parser.should_parse("2024-01-01 12:00:00 [INFO] [SCENE] Set Source [Test Zone]"));
    assert!(parser.should_parse("[SCENE] Load Source [Act 1]"));
    
    // Test non-scene change lines
    assert!(!parser.should_parse("2024-01-01 12:00:00 [INFO] Player moved"));
    assert!(!parser.should_parse("Some other log message"));
}

#[test]
fn test_scene_change_parser_content_parsing() {
    let parser = SceneChangeParser::new();
    
    // Test parsing valid scene changes
    let event = parser.parse_line("[SCENE] Set Source [Test Zone]");
    assert!(event.is_some());
    
    let event = parser.parse_line("[SCENE] Load Source [Act 1]");
    assert!(event.is_some());
    
    // Test with null/undefined content (should be filtered out)
    let event = parser.parse_line("[SCENE] Set Source [(null)]");
    assert!(event.is_none());
    
    let event = parser.parse_line("[SCENE] Load Source [undefined]");
    assert!(event.is_none());
    
    let event = parser.parse_line("[SCENE] Set Source []");
    assert!(event.is_none());
}

#[test]
fn test_scene_change_parser_parse_line() {
    let parser = SceneChangeParser::new();
    
    // Test parsing valid scene changes
    let event = parser.parse_line("[SCENE] Set Source [Test Zone]");
    assert!(event.is_some());
    
    let event = parser.parse_line("[SCENE] Load Source [Act 1]");
    assert!(event.is_some());
    
    // Test parsing invalid lines
    let event = parser.parse_line("Some other message");
    assert!(event.is_none());
    
    let event = parser.parse_line("[SCENE] Set Source [(null)]");
    assert!(event.is_none());
}

#[test]
fn test_scene_change_parser_location_type_detection() {
    let parser = SceneChangeParser::new();
    
    // Test hideout detection
    let event = parser.parse_line("[SCENE] Set Source [My Hideout]");
    if let Some(SceneChangeEvent::Hideout(hideout_event)) = event {
        assert_eq!(hideout_event.hideout_name, "My Hideout");
    } else {
        panic!("Expected hideout event");
    }
    
    // Test act detection
    let event = parser.parse_line("[SCENE] Load Source [Act 2]");
    if let Some(SceneChangeEvent::Act(act_event)) = event {
        assert_eq!(act_event.act_name, "Act 2");
    } else {
        panic!("Expected act event");
    }
    
    // Test zone detection (default case)
    let event = parser.parse_line("[SCENE] Set Source [Some Random Zone]");
    if let Some(SceneChangeEvent::Zone(zone_event)) = event {
        assert_eq!(zone_event.zone_name, "Some Random Zone");
    } else {
        panic!("Expected zone event");
    }
}

#[test]
fn test_parser_manager_creation() {
    let state_manager = PlayerLocationManager::new();
    let parser_manager = LogParserManager::new(Arc::new(state_manager));
    
    // Verify that the parser manager was created successfully
    let active_parsers = parser_manager.get_active_parsers();
    assert!(!active_parsers.is_empty());
}

#[tokio::test]
async fn test_parser_manager_parse_line() {
    let state_manager = PlayerLocationManager::new();
    let parser_manager = LogParserManager::new(Arc::new(state_manager));
    
    // Test parsing a valid scene change line
    let result = parser_manager.parse_line("[SCENE] Set Source [Test Zone]").await;
    assert!(result.is_some());
    
    // Test parsing an invalid line
    let result = parser_manager.parse_line("Some random log message").await;
    assert!(result.is_none());
}

#[test]
fn test_parser_config_default() {
    let config = ParsersConfig::default();
    
    assert!(!config.scene_change.patterns.is_empty());
    assert!(config.scene_change.scene_types.is_some());
    
    let scene_types = config.scene_change.scene_types.as_ref().unwrap();
    assert!(!scene_types.act.is_empty());
    assert!(!scene_types.hideout.is_empty());
    assert!(!scene_types.zone.is_empty());
}

#[test]
fn test_parser_config_matches_patterns() {
    let config = ParsersConfig::default();
    
    // Test matching valid patterns
    assert!(config.matches_patterns("scene_change", "[SCENE] Set Source [Test]"));
    
    // Test non-matching patterns
    assert!(!config.matches_patterns("scene_change", "Some other message"));
    assert!(!config.matches_patterns("nonexistent", "Any message"));
}

#[test]
fn test_parser_config_get_scene_type_config() {
    let config = ParsersConfig::default();
    
    // Test getting valid scene type config
    let scene_config = config.get_scene_type_config("scene_change");
    assert!(scene_config.is_some());
    
    // Test getting invalid scene type config
    let scene_config = config.get_scene_type_config("nonexistent");
    assert!(scene_config.is_none());
}

#[test]
fn test_scene_type_config_structure() {
    let config = ParsersConfig::default();
    let scene_config = config.get_scene_type_config("scene_change").unwrap();
    
    // Verify the structure of scene type config
    assert!(!scene_config.act.is_empty());
    assert!(!scene_config.hideout.is_empty());
    assert!(!scene_config.zone.is_empty());
    
    // Verify that act keywords contain expected values
    assert!(scene_config.act.iter().any(|k| k.contains("act")));
    
    // Verify that hideout keywords contain expected values
    assert!(scene_config.hideout.iter().any(|k| k.contains("hideout")));
}

#[test]
fn test_parser_error_handling() {
    let parser = SceneChangeParser::new();
    
    // Test with malformed lines that shouldn't crash
    let event = parser.parse_line("");
    assert!(event.is_none());
    
    let event = parser.parse_line("Scene change: [");
    assert!(event.is_none());
    
    let event = parser.parse_line("Scene change: ]");
    assert!(event.is_none());
    
    let event = parser.parse_line("Scene change: [   ]");
    assert!(event.is_none());
}

#[test]
fn test_parser_case_sensitivity() {
    let parser = SceneChangeParser::new();
    
    // Test that parsing is case-insensitive for content
    let event1 = parser.parse_line("[SCENE] Set Source [Test Zone]");
    let event2 = parser.parse_line("[SCENE] Set Source [test zone]");
    
    // Both should parse successfully
    assert!(event1.is_some());
    assert!(event2.is_some());
}

#[test]
fn test_parser_manager_active_parsers() {
    let state_manager = PlayerLocationManager::new();
    let parser_manager = LogParserManager::new(Arc::new(state_manager));
    
    // Verify that active parsers are available
    let active_parsers = parser_manager.get_active_parsers();
    assert!(!active_parsers.is_empty());
    assert!(active_parsers.contains(&"scene_change"));
}
