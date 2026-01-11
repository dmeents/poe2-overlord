#[cfg(test)]
mod tests {
    use crate::infrastructure::parsing::parsers::character_death_parser::CharacterDeathParser;
    use crate::infrastructure::parsing::manager::ParserResult;
    use crate::infrastructure::parsing::LogParser;

    fn create_parser() -> CharacterDeathParser {
        CharacterDeathParser::new()
    }

    // ============= should_parse Tests =============

    #[test]
    fn test_should_parse_valid_death_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter has been slain.";
        assert!(parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_with_different_names() {
        let parser = create_parser();

        let names = vec![
            "SimpleChar",
            "Char123",
            "X",
            "MyAwesomeCharacterName",
        ];

        for name in names {
            let line = format!("[INFO Client 1234] : {} has been slain.", name);
            assert!(parser.should_parse(&line), "Should parse name: {}", name);
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
    fn test_should_parse_returns_false_for_similar_but_different_text() {
        let parser = create_parser();
        // Missing the period at the end
        let line = "[INFO Client 1234] : TestCharacter has been slain";
        assert!(!parser.should_parse(line));
    }

    #[test]
    fn test_should_parse_returns_false_for_empty_line() {
        let parser = create_parser();
        assert!(!parser.should_parse(""));
    }

    // ============= parse_line Tests =============

    #[test]
    fn test_parse_line_extracts_character_name() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestCharacter has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterDeath(name) => {
                assert_eq!(name, "TestCharacter");
            }
            _ => panic!("Expected CharacterDeath result"),
        }
    }

    #[test]
    fn test_parse_line_single_character_name() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : X has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterDeath(name) => {
                assert_eq!(name, "X");
            }
            _ => panic!("Expected CharacterDeath result"),
        }
    }

    #[test]
    fn test_parse_line_character_with_numbers() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar123 has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterDeath(name) => {
                assert_eq!(name, "TestChar123");
            }
            _ => panic!("Expected CharacterDeath result"),
        }
    }

    #[test]
    fn test_parse_line_character_with_underscores() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : Test_Character has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterDeath(name) => {
                assert_eq!(name, "Test_Character");
            }
            _ => panic!("Expected CharacterDeath result"),
        }
    }

    #[test]
    fn test_parse_line_returns_error_for_non_matching_line() {
        let parser = create_parser();
        let line = "[INFO Client 1234] : TestChar (Warrior) is now level 50";

        let result = parser.parse_line(line);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_line_handles_leading_whitespace() {
        let parser = create_parser();
        // Leading whitespace IS supported - regex uses is_match which finds pattern anywhere
        let line = "   [INFO Client 1234] : TestCharacter has been slain.";

        let result = parser.parse_line(line);
        assert!(result.is_ok());

        match result.unwrap() {
            ParserResult::CharacterDeath(name) => {
                assert_eq!(name, "TestCharacter");
            }
            _ => panic!("Expected CharacterDeath result"),
        }
    }

    #[test]
    fn test_should_parse_rejects_trailing_whitespace() {
        let parser = create_parser();
        // Trailing whitespace is NOT supported due to regex `$` anchor requiring end of string
        let line = "[INFO Client 1234] : TestCharacter has been slain.   ";
        assert!(!parser.should_parse(line));
    }

    // ============= parser_name Tests =============

    #[test]
    fn test_parser_name() {
        let parser = create_parser();
        assert_eq!(parser.parser_name(), "character_death");
    }

    // ============= Default Implementation Tests =============

    #[test]
    fn test_default_implementation() {
        let parser: CharacterDeathParser = Default::default();
        assert_eq!(parser.parser_name(), "character_death");
    }
}
