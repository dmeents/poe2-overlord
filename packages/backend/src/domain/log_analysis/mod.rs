//! Log Analysis Domain Module
//!
//! This module provides functionality for monitoring and analyzing game log files.
//! It includes services for parsing log events, managing analysis sessions, and
//! coordinating with other domain services like character and server monitoring.

pub mod events;
pub mod models;
pub mod repository;
pub mod service;
pub mod traits;

// Re-export commonly used types and implementations
pub use events::LogAnalysisEvent;
pub use models::{LogAnalysisConfig, LogAnalysisError, LogFileInfo, LogLineAnalysis};
pub use repository::LogFileRepositoryImpl;
pub use service::LogAnalysisServiceImpl;
pub use traits::{LogAnalysisService, LogFileRepository};
