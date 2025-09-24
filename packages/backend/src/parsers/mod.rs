pub mod config;

// Re-export main types for backward compatibility
pub use config::{ParserConfig, ParsersConfig};
// Re-export infrastructure parsing types
pub use crate::infrastructure::parsing::{
    extract_content_between_delimiters, extract_content_by_patterns, matches_patterns,
    validate_content, CharacterDeathParser, CharacterLevelParser, LogParser, LogParserManager,
    ParseError, ParserFactory, ParserResult, ParserType, SceneChangeParser, ServerConnectionParser,
};
