#[cfg(test)]
mod tests {
    use crate::infrastructure::parsing::parsers::scene_change_parser::SceneChangeParser;
    use crate::infrastructure::parsing::manager::ParserResult;
    use crate::infrastructure::parsing::LogParser;

    fn create_parser() -> SceneChangeParser {
        SceneChangeParser::new()
    }

    // ============= should_parse Tests =============

    #[test]
    fn test_should_parse_set_source_pattern() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Set Source [The Coast]";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_load_source_pattern() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Load Source [Clearfell]";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_non_scene_line() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] : TestChar (Warrior) is now level 50";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_scene_without_pattern() {
        let parser = create_parser();
        // Has [SCENE] but not the specific patterns
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Other Event [SomeZone]";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_empty_line() {
        let parser = create_parser();
        assert!(!parser.should_parse(""));
    }

    // ============= parse_line Tests =============

    #[test]
    fn test_parse_line_extracts_zone_name_set_source() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Set Source [The Coast]";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::SceneChange(zone_name) => {
                assert_eq!(zone_name, "The Coast");
            }
            _ => panic!("Expected SceneChange result"),
        }
    }

    #[test]
    fn test_parse_line_extracts_zone_name_load_source() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Load Source [Clearfell]";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::SceneChange(zone_name) => {
                assert_eq!(zone_name, "Clearfell");
            }
            _ => panic!("Expected SceneChange result"),
        }
    }

    #[test]
    fn test_parse_line_handles_zone_with_special_characters() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Set Source [The Mud Flats]";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::SceneChange(zone_name) => {
                assert_eq!(zone_name, "The Mud Flats");
            }
            _ => panic!("Expected SceneChange result"),
        }
    }

    #[test]
    fn test_parse_line_handles_hideout() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] [SCENE] Set Source [Celestial Hideout]";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::SceneChange(zone_name) => {
                assert_eq!(zone_name, "Celestial Hideout");
            }
            _ => panic!("Expected SceneChange result"),
        }
    }

    #[test]
    fn test_parse_line_returns_error_for_non_matching_line() {
        let parser = create_parser();
        let line = "2026/01/11 10:30:45 12345678 abc [INFO Client 1234] : TestChar has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    // ============= parser_name Tests =============

    #[test]
    fn test_parser_name() {
        let parser = create_parser();
        assert_eq!(parser.parser_name(), "scene_change");
    }

    // ============= Default Implementation Tests =============

    #[test]
    fn test_default_implementation() {
        let parser: SceneChangeParser = Default::default();
        assert_eq!(parser.parser_name(), "scene_change");
    }
}
