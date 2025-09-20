use app_lib::parsers::core::LogParser;
use app_lib::parsers::specific_parsers::{CharacterLevelParser, CharacterDeathParser};
use app_lib::models::character::CharacterClass;

#[test]
fn test_character_level_parser_basic() {
    let parser = CharacterLevelParser::new();

    // Test the basic level-up pattern with actual log format
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : Lylunin (Sorceress) is now level 2";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let (character_name, character_class, level) = result.unwrap();
    assert_eq!(character_name, "Lylunin");
    assert_eq!(character_class, CharacterClass::Sorceress);
    assert_eq!(level, 2);
}

#[test]
fn test_character_level_parser_different_character() {
    let parser = CharacterLevelParser::new();

    // Test with a different character
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : MyWarrior (Warrior) is now level 15";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let (character_name, character_class, level) = result.unwrap();
    assert_eq!(character_name, "MyWarrior");
    assert_eq!(character_class, CharacterClass::Warrior);
    assert_eq!(level, 15);
}

#[test]
fn test_character_level_parser_different_class() {
    let parser = CharacterLevelParser::new();

    // Test with a different class
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : RangerChar (Ranger) is now level 5";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let (character_name, character_class, level) = result.unwrap();
    assert_eq!(character_name, "RangerChar");
    assert_eq!(character_class, CharacterClass::Ranger);
    assert_eq!(level, 5);
}

#[test]
fn test_character_level_parser_high_level() {
    let parser = CharacterLevelParser::new();

    // Test with a high level character
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : EndGameChar (Monk) is now level 100";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let (character_name, character_class, level) = result.unwrap();
    assert_eq!(character_name, "EndGameChar");
    assert_eq!(character_class, CharacterClass::Monk);
    assert_eq!(level, 100);
}

#[test]
fn test_character_level_parser_invalid_line() {
    let parser = CharacterLevelParser::new();

    // Test with a line that doesn't match the pattern
    let log_line = "2025/09/03 22:43:49 246857285 91c6ccb [INFO Client 320] Some other log message";

    // Should not match the pattern
    assert!(!parser.should_parse(log_line));
}

#[test]
fn test_character_level_parser_malformed_line() {
    let parser = CharacterLevelParser::new();

    // Test with malformed level-up line (missing level number)
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : Lylunin (Sorceress) is now level";

    // Should match the pattern but fail to parse
    assert!(parser.should_parse(log_line));

    let result = parser.parse_line(log_line);
    assert!(
        result.is_err(),
        "Should have failed to parse malformed level-up line"
    );
}

#[test]
fn test_character_level_parser_with_whitespace() {
    let parser = CharacterLevelParser::new();

    // Test with extra whitespace
    let log_line = "  2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : Lylunin (Sorceress) is now level 2  ";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let (character_name, character_class, level) = result.unwrap();
    assert_eq!(character_name, "Lylunin");
    assert_eq!(character_class, CharacterClass::Sorceress);
    assert_eq!(level, 2);
}

#[test]
fn test_character_death_parser_basic() {
    let parser = CharacterDeathParser::new();

    // Test the basic death pattern with actual log format
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : Lylunin has been slain.";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let character_name = result.unwrap();
    assert_eq!(character_name, "Lylunin");
}

#[test]
fn test_character_death_parser_different_character() {
    let parser = CharacterDeathParser::new();

    // Test with a different character
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : MyWarrior has been slain.";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let character_name = result.unwrap();
    assert_eq!(character_name, "MyWarrior");
}

#[test]
fn test_character_death_parser_complex_name() {
    let parser = CharacterDeathParser::new();

    // Test with a character name that has spaces or special characters
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : My Character Name has been slain.";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let character_name = result.unwrap();
    assert_eq!(character_name, "My Character Name");
}

#[test]
fn test_character_death_parser_invalid_line() {
    let parser = CharacterDeathParser::new();

    // Test with a line that doesn't match the pattern
    let log_line = "2025/09/03 22:43:49 246857285 91c6ccb [INFO Client 320] Some other log message";

    // Should not match the pattern
    assert!(!parser.should_parse(log_line));
}

#[test]
fn test_character_death_parser_malformed_line() {
    let parser = CharacterDeathParser::new();

    // Test with malformed death line (missing period)
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : Lylunin has been slain";

    // Should match the pattern (contains "has been slain") but fail to parse (missing period)
    assert!(parser.should_parse(log_line));
    
    let result = parser.parse_line(log_line);
    assert!(
        result.is_err(),
        "Should have failed to parse malformed death line (missing period)"
    );
}

#[test]
fn test_character_death_parser_with_whitespace() {
    let parser = CharacterDeathParser::new();

    // Test with extra whitespace
    let log_line = "  2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : Lylunin has been slain.  ";

    // Should match the pattern
    assert!(parser.should_parse(log_line));

    // Should successfully parse
    let result = parser.parse_line(log_line);
    assert!(result.is_ok(), "Failed to parse log line: {:?}", result);

    let character_name = result.unwrap();
    assert_eq!(character_name, "Lylunin");
}

#[test]
fn test_character_death_parser_empty_name() {
    let parser = CharacterDeathParser::new();

    // Test with empty character name
    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] :  has been slain.";

    // Should match the pattern but fail to parse
    assert!(parser.should_parse(log_line));

    let result = parser.parse_line(log_line);
    assert!(
        result.is_err(),
        "Should have failed to parse empty character name"
    );
}

#[test]
fn test_character_level_parser_parser_name() {
    let parser = CharacterLevelParser::new();
    assert_eq!(parser.parser_name(), "character_level");
}

#[test]
fn test_character_death_parser_parser_name() {
    let parser = CharacterDeathParser::new();
    assert_eq!(parser.parser_name(), "character_death");
}

#[test]
fn test_character_level_parser_with_config() {
    use app_lib::parsers::config::ParsersConfig;
    
    let config = ParsersConfig::default();
    let parser = CharacterLevelParser::with_config(config);

    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : TestChar (Witch) is now level 10";
    
    assert!(parser.should_parse(log_line));
    let result = parser.parse_line(log_line);
    assert!(result.is_ok());
    
    let (character_name, character_class, level) = result.unwrap();
    assert_eq!(character_name, "TestChar");
    assert_eq!(character_class, CharacterClass::Witch);
    assert_eq!(level, 10);
}

#[test]
fn test_character_death_parser_with_config() {
    use app_lib::parsers::config::ParsersConfig;
    
    let config = ParsersConfig::default();
    let parser = CharacterDeathParser::with_config(config);

    let log_line = "2025/09/14 02:56:13 25066654 3ef232c2 [INFO Client 316] : TestChar has been slain.";
    
    assert!(parser.should_parse(log_line));
    let result = parser.parse_line(log_line);
    assert!(result.is_ok());
    
    let character_name = result.unwrap();
    assert_eq!(character_name, "TestChar");
}