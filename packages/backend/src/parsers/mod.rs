pub mod config;
pub mod core;
pub mod detection;
pub mod events;
pub mod parsers;
pub mod utils;

// Re-export main types for backward compatibility
pub use config::{ParserConfig, ParsersConfig, SceneTypeConfig};
pub use core::{LogParser, LogParserManager, ParseError, ParserFactory, ParserResult, ParserType};
pub use detection::{SceneType, SceneTypeDetector};
pub use events::EventFactory;
pub use parsers::{SceneChangeParser, ServerConnectionParser};
pub use utils::{
    extract_content_between_delimiters, extract_content_by_patterns, matches_patterns,
    validate_content,
};
