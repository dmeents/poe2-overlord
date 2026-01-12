#[cfg(test)]
mod tests {
    use crate::infrastructure::parsing::manager::ParserResult;
    use crate::infrastructure::parsing::parsers::zone_level_parser::ZoneLevelParser;
    use crate::infrastructure::parsing::LogParser;

    fn create_parser() -> ZoneLevelParser {
        ZoneLevelParser::new()
    }

    // ============= should_parse Tests =============

    #[test]
    fn test_should_parse_valid_zone_level_line() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] Generating level 50 area";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_minimal_pattern() {
        let parser = create_parser();
        let line = "Generating level 1 area";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_with_different_levels() {
        let parser = create_parser();

        let levels = vec![1, 10, 50, 68, 75, 100];

        for level in levels {
            let line = format!("Generating level {} area", level);
            assert!(parser.should_parse(&line), "Should parse level: {}", level);
        }
    }

    #[test]
    fn test_should_parse_returns_false_for_level_up() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter (Warrior) is now level 50";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_scene_change() {
        let parser = create_parser();
        let line = "[INFO Client 1234] [SCENE] Set Source [The Coast]";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_similar_text() {
        let parser = create_parser();
        // Wrong format - missing "area"
        let line = "Generating level 50";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_empty_line() {
        let parser = create_parser();
        assert!(!parser.should_parse(""));
    }

    // ============= parse_line Tests =============

    #[test]
    fn test_parse_line_extracts_zone_level() {
        let parser = create_parser();
        let line = "Generating level 50 area";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ZoneLevel(level) => {
                assert_eq!(level, 50);
            }
            _ => panic!("Expected ZoneLevel result"),
        }
    }

    #[test]
    fn test_parse_line_with_full_log_format() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] Generating level 68 area";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ZoneLevel(level) => {
                assert_eq!(level, 68);
            }
            _ => panic!("Expected ZoneLevel result"),
        }
    }

    #[test]
    fn test_parse_line_level_1() {
        let parser = create_parser();
        let line = "Generating level 1 area";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ZoneLevel(level) => {
                assert_eq!(level, 1);
            }
            _ => panic!("Expected ZoneLevel result"),
        }
    }

    #[test]
    fn test_parse_line_level_100() {
        let parser = create_parser();
        let line = "Generating level 100 area";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ZoneLevel(level) => {
                assert_eq!(level, 100);
            }
            _ => panic!("Expected ZoneLevel result"),
        }
    }

    #[test]
    fn test_parse_line_high_level() {
        let parser = create_parser();
        let line = "Generating level 200 area";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ZoneLevel(level) => {
                assert_eq!(level, 200);
            }
            _ => panic!("Expected ZoneLevel result"),
        }
    }

    #[test]
    fn test_parse_line_returns_error_for_non_matching_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_with_surrounding_text() {
        let parser = create_parser();
        let line = "Some prefix text Generating level 75 area some suffix text";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::ZoneLevel(level) => {
                assert_eq!(level, 75);
            }
            _ => panic!("Expected ZoneLevel result"),
        }
    }

    #[test]
    fn test_parse_line_various_zone_levels() {
        let parser = create_parser();

        let test_cases = vec![
            ("Generating level 5 area", 5u32),
            ("Generating level 15 area", 15u32),
            ("Generating level 30 area", 30u32),
            ("Generating level 45 area", 45u32),
            ("Generating level 60 area", 60u32),
        ];

        for (line, expected_level) in test_cases {
            let result = parser.parse_line(line);
            assert!(result.is_ok(), "Should parse level: {}", expected_level);

            match result.unwrap() {
                ParserResult::ZoneLevel(level) => {
                    assert_eq!(level, expected_level, "Level should match");
                }
                _ => panic!("Expected ZoneLevel result"),
            }
        }
    }

    // ============= parser_name Tests =============

    #[test]
    fn test_parser_name() {
        let parser = create_parser();
        assert_eq!(parser.parser_name(), "zone_level");
    }

    // ============= Default Implementation Tests =============

    #[test]
    fn test_default_implementation() {
        let parser: ZoneLevelParser = Default::default();
        assert_eq!(parser.parser_name(), "zone_level");
    }
}
