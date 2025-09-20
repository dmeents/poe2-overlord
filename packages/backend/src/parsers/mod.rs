pub mod config;
pub mod core;
pub mod specific_parsers;
pub mod utils;

// Re-export main types for backward compatibility
pub use config::{ParserConfig, ParsersConfig};
pub use core::{LogParser, LogParserManager, ParseError, ParserFactory, ParserResult, ParserType};
pub use specific_parsers::{
    CharacterDeathParser, CharacterLevelParser, SceneChangeParser, ServerConnectionParser,
};
pub use utils::{
    extract_content_between_delimiters, extract_content_by_patterns, matches_patterns,
    validate_content,
};
