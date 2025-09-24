pub mod constants;
pub mod os_detection;
pub mod time_calculations;

// Re-export commonly used items
pub use constants::PoeClientLogPaths;
pub use time_calculations::{
    calculate_active_session_duration_seconds, calculate_session_duration_seconds,
    calculate_session_duration_from_timestamps, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
};
