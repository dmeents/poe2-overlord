//! Infrastructure layer providing concrete implementations of domain traits
//!
//! This module contains all the infrastructure concerns including:
//! - System monitoring (process detection, server monitoring)
//! - File management (file operations, JSON storage, path utilities)
//! - Log parsing and analysis
//! - Tauri integration (events, commands)
//! - Time calculations and validation

pub mod file_management;
pub mod monitoring;
pub mod parsing;
pub mod tauri;
pub mod time;

// Re-export commonly used infrastructure components
pub use file_management::expand_tilde;
pub use monitoring::process_monitor::ProcessMonitorImpl;
pub use time::{
    calculate_active_session_duration_seconds, calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
};
