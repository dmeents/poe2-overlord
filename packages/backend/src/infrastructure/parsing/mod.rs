//! Log parsing infrastructure for analyzing POE2 game logs
//!
//! Provides comprehensive log parsing capabilities including pattern matching,
//! event extraction, and real-time log monitoring. Handles various game events
//! such as scene changes, server connections, character progression, and deaths.
//!
//! This module focuses on infrastructure concerns (file monitoring and parsing)
//! and delegates domain logic to event handlers through the event system.

// analyzer module removed - functionality moved to domain layer
pub mod config;
pub mod errors;
pub mod factory;
pub mod manager;
pub mod parsers;
pub mod traits;
pub mod utils;

// LogAnalyzer has been removed - use LogAnalysisService from domain layer instead
pub use config::ParsersConfig;
pub use errors::ParseError;
pub use factory::ParserFactory;
pub use manager::{LogParserManager, ParserResult};
pub use parsers::*;
pub use traits::LogParser;
pub use utils::*;
