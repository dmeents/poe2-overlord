//! Time calculation and validation utilities

pub mod calculations;
#[cfg(test)]
mod calculations_test;

pub use calculations::{
    calculate_active_session_duration_seconds, calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
};
