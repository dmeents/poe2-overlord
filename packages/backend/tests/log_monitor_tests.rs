use app_lib::parsers::scene_change_parser::SceneChangeParser;
use app_lib::parsers::traits::LogParser;
use app_lib::models::events::SceneChangeEvent;

#[tokio::test]
async fn test_scene_change_parser() {
    let parser = SceneChangeParser::new();

    // Test valid zone change line
    let line = "[SCENE] Set Source [The Coast]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Zone(zone_event)) = event {
        assert_eq!(zone_event.zone_name, "The Coast");
        assert!(!zone_event.timestamp.is_empty());
    } else {
        panic!("Expected Zone event");
    }

    // Test valid hideout change line
    let line = "[SCENE] Set Source [Felled Hideout]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Hideout(hideout_event)) = event {
        assert_eq!(hideout_event.hideout_name, "Felled Hideout");
        assert!(!hideout_event.timestamp.is_empty());
    } else {
        panic!("Expected Hideout event");
    }

    // Test valid act change line
    let line = "[SCENE] Set Source [Act 1]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Act(act_event)) = event {
        assert_eq!(act_event.act_name, "Act 1");
        assert!(!act_event.timestamp.is_empty());
    } else {
        panic!("Expected Act event");
    }

    // Test invalid line
    let line = "Some other log line";
    let event = parser.parse_line(line);
    assert!(event.is_none());
}

#[tokio::test]
async fn test_scene_change_parser_edge_cases() {
    let parser = SceneChangeParser::new();

    // Test with different zone names
    let zone_test_cases = [
        "[SCENE] Set Source [Lioneye's Watch]",
        "[SCENE] Set Source [The Coast]",
        "[SCENE] Set Source [Tidal Island]",
        "[SCENE] Set Source [Submerged Passage]",
    ];

    for line in zone_test_cases {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse zone: {}", line);

        if let Some(SceneChangeEvent::Zone(zone_event)) = event {
            assert!(!zone_event.zone_name.is_empty());
            assert!(!zone_event.timestamp.is_empty());
        } else {
            panic!("Expected Zone event for: {}", line);
        }
    }

    // Test with different act names
    let act_test_cases = [
        "[SCENE] Set Source [Act 1]",
        "[SCENE] Set Source [Act 2]",
        "[SCENE] Set Source [Act 3]",
        "[SCENE] Set Source [Atlas]",
        "[SCENE] Set Source [Interlude]",
    ];

    for line in act_test_cases {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse act: {}", line);

        if let Some(SceneChangeEvent::Act(act_event)) = event {
            assert!(!act_event.act_name.is_empty());
            assert!(!act_event.timestamp.is_empty());
        } else {
            panic!("Expected Act event for: {}", line);
        }
    }

    // Test malformed lines
    let malformed_lines = [
        "[SCENE] Set Source [",
        "[SCENE] Set Source",
        "Set Source [Zone]",
        "[SCENE] Set Source Zone]",
        "[SCENE] Set Source [Zone",
    ];

    for line in malformed_lines {
        let event = parser.parse_line(line);
        assert!(event.is_none(), "Should not parse malformed line: {}", line);
    }
}

#[tokio::test]
async fn test_scene_change_parser_null_filtering() {
    let parser = SceneChangeParser::new();

    // Test null/empty content filtering
    let null_test_cases = [
        "[SCENE] Set Source [(null)]",
        "[SCENE] Set Source [undefined]",
        "[SCENE] Set Source [null]",
        "[SCENE] Set Source [NULL]",
        "[SCENE] Set Source []",
        "[SCENE] Set Source [   ]",
    ];

    for line in null_test_cases {
        let event = parser.parse_line(line);
        assert!(event.is_none(), "Should not parse null/empty content: {}", line);
    }
}

#[tokio::test]
async fn test_scene_change_parser_act_detection() {
    let parser = SceneChangeParser::new();

    // Test act detection logic
    let act_test_cases = [
        "[SCENE] Set Source [Act 1]",
        "[SCENE] Set Source [Act 2]",
        "[SCENE] Set Source [Act 3]",
        "[SCENE] Set Source [atlas]",
        "[SCENE] Set Source [Atlas]",
        "[SCENE] Set Source [interlude]",
        "[SCENE] Set Source [Interlude]",
        "[SCENE] Set Source [ACT 1]",
        "[SCENE] Set Source [act 1]",
    ];

    for line in act_test_cases {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse act: {}", line);

        if let Some(SceneChangeEvent::Act(_)) = event {
            // This is an act event, which is what we expect
        } else {
            panic!("Expected Act event for: {}", line);
        }
    }

    // Test zone detection (should not be detected as acts)
    let zone_test_cases = [
        "[SCENE] Set Source [Lioneye's Watch]",
        "[SCENE] Set Source [The Coast]",
        "[SCENE] Set Source [Tidal Island]",
        "[SCENE] Set Source [Submerged Passage]",
        "[SCENE] Set Source [The Mud Flats]",
    ];

    for line in zone_test_cases {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse zone: {}", line);

        if let Some(SceneChangeEvent::Zone(_)) = event {
            // This is a zone event, which is what we expect
        } else {
            panic!("Expected Zone event for: {}", line);
        }
    }
}
