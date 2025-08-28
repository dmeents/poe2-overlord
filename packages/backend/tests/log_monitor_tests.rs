use app_lib::services::log_monitor::*;

#[tokio::test]
async fn test_zone_change_parser() {
    let parser = ZoneChangeParser;

    // Test valid zone change line
    let line = "[SCENE] Set Source [Felled Hideout]";
    let event = parser.parse_line(line);

    assert!(event.is_some());
    if let Some(zone_event) = event {
        assert_eq!(zone_event.zone_name, "Felled Hideout");
        assert!(!zone_event.timestamp.is_empty());
    }

    // Test invalid line
    let line = "Some other log line";
    let event = parser.parse_line(line);
    assert!(event.is_none());
}

#[tokio::test]
async fn test_zone_change_parser_edge_cases() {
    let parser = ZoneChangeParser;

    // Test with different zone names
    let test_cases = [
        "[SCENE] Set Source [Lioneye's Watch]",
        "[SCENE] Set Source [The Coast]",
        "[SCENE] Set Source [Tidal Island]",
        "[SCENE] Set Source [Submerged Passage]",
    ];

    for line in test_cases {
        let event = parser.parse_line(line);
        assert!(event.is_some(), "Failed to parse: {}", line);

        if let Some(zone_event) = event {
            assert!(!zone_event.zone_name.is_empty());
            assert!(!zone_event.timestamp.is_empty());
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
