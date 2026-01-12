#[cfg(test)]
mod tests {
    use crate::infrastructure::parsing::manager::ParserResult;
    use crate::infrastructure::parsing::parsers::character_level_parser::CharacterLevelParser;
    use crate::infrastructure::parsing::LogParser;

    fn create_parser() -> CharacterLevelParser {
        CharacterLevelParser::new()
    }

    // ============= should_parse Tests =============

    #[test]
    fn test_should_parse_valid_level_up_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter (Warrior) is now level 50";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_with_different_classes() {
        let parser = create_parser();

        let classes = vec![
            "Warrior",
            "Sorceress",
            "Ranger",
            "Huntress",
            "Monk",
            "Mercenary",
            "Witch",
            "Druid",
        ];

        for class in classes {
            let line = format!("[INFO Client 1234] : TestChar ({}) is now level 10", class);
            assert!(parser.should_parse(&line), "Should parse class: {}", class);
        }
    }

    #[test]
    fn test_should_parse_returns_false_for_non_level_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter has been slain.";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_scene_change() {
        let parser = create_parser();
        let line = "[INFO Client 1234] [SCENE] Set Source [The Coast]";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_empty_line() {
        let parser = create_parser();
        assert!(!parser.should_parse(""));
    }

    // ============= parse_line Tests =============

    #[test]
    fn test_parse_line_extracts_character_name_and_level() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter (Warrior) is now level 50";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterLevel((name, level)) => {
                assert_eq!(name, "TestCharacter");
                assert_eq!(level, 50);
            }
            _ => panic!("Expected CharacterLevel result"),
        }
    }

    #[test]
    fn test_parse_line_level_1() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : NewChar (Monk) is now level 1";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterLevel((name, level)) => {
                assert_eq!(name, "NewChar");
                assert_eq!(level, 1);
            }
            _ => panic!("Expected CharacterLevel result"),
        }
    }

    #[test]
    fn test_parse_line_level_100() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : MaxLevelChar (Sorceress) is now level 100";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterLevel((name, level)) => {
                assert_eq!(name, "MaxLevelChar");
                assert_eq!(level, 100);
            }
            _ => panic!("Expected CharacterLevel result"),
        }
    }

    #[test]
    fn test_parse_line_character_with_spaces() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : My Cool Character (Ranger) is now level 25";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterLevel((name, level)) => {
                assert_eq!(name, "My Cool Character");
                assert_eq!(level, 25);
            }
            _ => panic!("Expected CharacterLevel result"),
        }
    }

    #[test]
    fn test_parse_line_rejects_level_over_100() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar (Warrior) is now level 101";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_rejects_level_0() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar (Warrior) is now level 0";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_returns_error_for_non_matching_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    // ============= parser_name Tests =============

    #[test]
    fn test_parser_name() {
        let parser = create_parser();
        assert_eq!(parser.parser_name(), "character_level");
    }

    // ============= Default Implementation Tests =============

    #[test]
    fn test_default_implementation() {
        let parser: CharacterLevelParser = Default::default();
        assert_eq!(parser.parser_name(), "character_level");
    }
}
