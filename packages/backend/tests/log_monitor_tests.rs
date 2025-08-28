use app_lib::services::log_monitor::*;

#[tokio::test]
async fn test_scene_change_parser() {
    let parser = SceneChangeParser;

    // Test valid zone change line
    let line = "[SCENE] Set Source [Felled Hideout]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Zone(zone_event)) = event {
        assert_eq!(zone_event.zone_name, "Felled Hideout");
        assert!(!zone_event.timestamp.is_empty());
    } else {
        panic!("Expected Zone event");
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
    let parser = SceneChangeParser;

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
        "[SCENE] Set Source [Prologue]",
        "[SCENE] Set Source [Epilogue]",
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
    let parser = SceneChangeParser;

    // Test null zone filtering
    let null_test_cases = [
        "[SCENE] Set Source [(null)]",
        "[SCENE] Set Source [null]",
        "[SCENE] Set Source [NULL]",
        "[SCENE] Set Source []",
        "[SCENE] Set Source [   ]",
    ];

    for line in null_test_cases {
        let event = parser.parse_line(line);
        assert!(
            event.is_none(),
            "Should filter out null/empty content: {}",
            line
        );
    }
}

#[tokio::test]
async fn test_act_detection_logic() {
    let parser = SceneChangeParser;

    // Test act detection
    let act_lines = [
        "[SCENE] Set Source [Act 1]",
        "[SCENE] Set Source [Act 2]",
        "[SCENE] Set Source [Act 3]",
        "[SCENE] Set Source [Prologue]",
        "[SCENE] Set Source [Epilogue]",
        "[SCENE] Set Source [Act 4]",
        "[SCENE] Set Source [Act 5]",
    ];

    for line in act_lines {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse act line: {}", line);

        if let Some(SceneChangeEvent::Act(_)) = event {
            // This is correct
        } else {
            panic!("Expected Act event for: {}", line);
        }
    }

    // Test zone detection (non-act content)
    let zone_lines = [
        "[SCENE] Set Source [Felled Hideout]",
        "[SCENE] Set Source [Lioneye's Watch]",
        "[SCENE] Set Source [The Coast]",
        "[SCENE] Set Source [Tidal Island]",
        "[SCENE] Set Source [Submerged Passage]",
        "[SCENE] Set Source [The Forest]",
        "[SCENE] Set Source [The Prison]",
    ];

    for line in zone_lines {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse zone line: {}", line);

        if let Some(SceneChangeEvent::Zone(_)) = event {
            // This is correct
        } else {
            panic!("Expected Zone event for: {}", line);
        }
    }
}

#[tokio::test]
async fn test_legacy_zone_change_parser() {
    let parser = ZoneChangeParser;

    // Test valid zone change line
    let line = "[SCENE] Set Source [Felled Hideout]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(zone_event) = event {
        assert_eq!(zone_event.zone_name, "Felled Hideout");
        assert!(!zone_event.timestamp.is_empty());
    }

    // Test act line should not be parsed as zone
    let line = "[SCENE] Set Source [Act 1]";
    let event = parser.parse_line(line);
    assert!(event.is_none());

    // Test null line should not be parsed
    let line = "[SCENE] Set Source [(null)]";
    let event = parser.parse_line(line);
    assert!(event.is_none());

    // Test invalid line
    let line = "Some other log line";
    let event = parser.parse_line(line);
    assert!(event.is_none());
}
