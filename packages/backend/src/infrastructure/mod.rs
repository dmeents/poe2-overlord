//! Infrastructure layer for concrete implementations

pub mod events;
pub mod file_management;
pub mod parsing;
pub mod security;
pub mod time;

pub use file_management::expand_tilde;
pub use security::PathValidator;
pub use time::{
    calculate_active_session_duration_seconds, calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
};
