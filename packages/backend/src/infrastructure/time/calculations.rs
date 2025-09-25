use chrono::{DateTime, Utc};
use log::{debug, warn};

/// Minimum session duration in seconds to prevent zero-duration sessions
const MIN_SESSION_DURATION_SECONDS: i64 = 1;

/// Calculates session duration in seconds between two timestamps
/// 
/// Computes the duration between entry and exit timestamps, ensuring a minimum
/// duration of 1 second to prevent zero-duration sessions. Handles negative
/// durations by clamping to zero.
pub fn calculate_session_duration_seconds(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: DateTime<Utc>,
) -> u64 {
    let duration = exit_timestamp.signed_duration_since(entry_timestamp);
    let milliseconds = duration.num_milliseconds().max(0);
    let seconds = (milliseconds / 1000).max(MIN_SESSION_DURATION_SECONDS) as u64;
    
    debug!(
        "Calculated session duration: {}ms -> {}s (entry: {}, exit: {})",
        milliseconds, seconds, entry_timestamp, exit_timestamp
    );
    
    seconds
}

/// Calculates duration for an active (ongoing) session
/// 
/// Uses the current UTC time as the exit timestamp to calculate
/// the duration of a session that is still in progress.
pub fn calculate_active_session_duration_seconds(entry_timestamp: DateTime<Utc>) -> u64 {
    let now = Utc::now();
    calculate_session_duration_seconds(entry_timestamp, now)
}

/// Calculates session duration using Unix timestamps
/// 
/// Alternative calculation method using timestamp arithmetic.
/// Ensures minimum duration and handles negative durations.
pub fn calculate_session_duration_from_timestamps(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: DateTime<Utc>,
) -> u64 {
    let duration_seconds = exit_timestamp.timestamp() - entry_timestamp.timestamp();
    duration_seconds.max(MIN_SESSION_DURATION_SECONDS) as u64
}

/// Result of time validation operations
/// 
/// Provides structured feedback for validation operations with different
/// severity levels: Valid, Warning (non-fatal issues), and Error (fatal issues).
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Warning(String),
    Error(String),
}

/// Validates the chronological order of timestamps
/// 
/// Ensures that exit timestamps come after entry timestamps and flags
/// unusually long sessions (>24 hours) as warnings. Returns appropriate
/// validation results with descriptive messages.
pub fn validate_timestamp_order(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: Option<DateTime<Utc>>,
) -> ValidationResult {
    if let Some(exit) = exit_timestamp {
        if exit < entry_timestamp {
            let error_msg = format!(
                "Invalid timestamp order: exit time {} is before entry time {}",
                exit, entry_timestamp
            );
            warn!("{}", error_msg);
            return ValidationResult::Error(error_msg);
        }
        
        let duration = exit.signed_duration_since(entry_timestamp);
        if duration.num_hours() > 24 {
            let warning_msg = format!(
                "Unusually long session duration: {} hours (entry: {}, exit: {})",
                duration.num_hours(), entry_timestamp, exit
            );
            warn!("{}", warning_msg);
            return ValidationResult::Warning(warning_msg);
        }
    }
    
    ValidationResult::Valid
}

/// Validates session duration values
/// 
/// Checks for zero durations (error) and unusually long sessions >24 hours (warning).
/// Provides appropriate validation results with descriptive messages.
pub fn validate_duration(duration_seconds: u64) -> ValidationResult {
    if duration_seconds > 24 * 60 * 60 {
        let warning_msg = format!(
            "Unusually long session duration: {} seconds ({} hours)",
            duration_seconds, duration_seconds / 3600
        );
        warn!("{}", warning_msg);
        return ValidationResult::Warning(warning_msg);
    }
    
    if duration_seconds == 0 {
        let error_msg = "Session duration cannot be zero".to_string();
        warn!("{}", error_msg);
        return ValidationResult::Error(error_msg);
    }
    
    ValidationResult::Valid
}

/// Validates that a new session doesn't overlap with existing sessions
/// 
/// Checks for temporal overlaps between the new session and all existing sessions.
/// Uses current time for ongoing sessions (where exit timestamp is None).
/// Returns an error if any overlap is detected.
pub fn validate_no_session_overlap(
    new_entry: DateTime<Utc>,
    new_exit: Option<DateTime<Utc>>,
    existing_sessions: &[(DateTime<Utc>, Option<DateTime<Utc>>)],
) -> ValidationResult {
    let new_exit = new_exit.unwrap_or_else(Utc::now);
    
    for (existing_entry, existing_exit) in existing_sessions {
        let existing_exit = existing_exit.unwrap_or_else(Utc::now);
        
        if new_entry < existing_exit && new_exit > *existing_entry {
            let error_msg = format!(
                "Session overlap detected: new session ({}-{}) overlaps with existing session ({}-{})",
                new_entry, new_exit, existing_entry, existing_exit
            );
            warn!("{}", error_msg);
            return ValidationResult::Error(error_msg);
        }
    }
    
    ValidationResult::Valid
}

/// Comprehensive validation of session data
/// 
/// Performs multiple validation checks on session data including timestamp order,
/// duration validity, and consistency between provided and calculated durations.
/// Returns the first error encountered or Valid if all checks pass.
pub fn validate_session_data(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: Option<DateTime<Utc>>,
    duration_seconds: Option<u64>,
) -> ValidationResult {
    // Validate timestamp order first
    match validate_timestamp_order(entry_timestamp, exit_timestamp) {
        ValidationResult::Error(msg) => return ValidationResult::Error(msg),
        ValidationResult::Warning(msg) => {
            warn!("Timestamp validation warning: {}", msg);
        }
        ValidationResult::Valid => {}
    }
    
    // Validate duration if provided
    if let Some(duration) = duration_seconds {
        match validate_duration(duration) {
            ValidationResult::Error(msg) => return ValidationResult::Error(msg),
            ValidationResult::Warning(msg) => {
                warn!("Duration validation warning: {}", msg);
            }
            ValidationResult::Valid => {}
        }
        
        // Check consistency between provided and calculated duration
        if let Some(exit) = exit_timestamp {
            let calculated_duration = calculate_session_duration_seconds(entry_timestamp, exit);
            if calculated_duration != duration {
                let error_msg = format!(
                    "Duration mismatch: provided {}s, calculated {}s (entry: {}, exit: {})",
                    duration, calculated_duration, entry_timestamp, exit
                );
                warn!("{}", error_msg);
                return ValidationResult::Error(error_msg);
            }
        }
    }
    
    ValidationResult::Valid
}