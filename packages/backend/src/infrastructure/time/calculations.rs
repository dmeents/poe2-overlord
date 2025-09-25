use chrono::{DateTime, Utc};
use log::{debug, warn};

/// Minimum session duration in seconds to prevent very short sessions from being recorded
const MIN_SESSION_DURATION_SECONDS: i64 = 1;

/// Calculate the duration between two timestamps in seconds
/// Uses millisecond precision for accuracy and applies minimum duration constraint
/// 
/// # Precision Guarantee
/// This function uses millisecond precision internally and converts to seconds,
/// ensuring consistent calculations across all session trackers.
/// 
/// # Arguments
/// * `entry_timestamp` - When the session started
/// * `exit_timestamp` - When the session ended
/// 
/// # Returns
/// Duration in seconds, with a minimum of 1 second
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

/// Calculate the duration from a timestamp to now in seconds
/// Uses millisecond precision for accuracy and applies minimum duration constraint
/// 
/// # Precision Guarantee
/// This function uses the same millisecond precision as calculate_session_duration_seconds,
/// ensuring consistent calculations for active sessions.
/// 
/// # Arguments
/// * `entry_timestamp` - When the session started
/// 
/// # Returns
/// Duration in seconds from entry time to now, with a minimum of 1 second
pub fn calculate_active_session_duration_seconds(entry_timestamp: DateTime<Utc>) -> u64 {
    let now = Utc::now();
    calculate_session_duration_seconds(entry_timestamp, now)
}

/// Calculate the duration between two timestamps in seconds (legacy timestamp-based method)
/// This is kept for backward compatibility but should be replaced with the DateTime-based version
pub fn calculate_session_duration_from_timestamps(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: DateTime<Utc>,
) -> u64 {
    let duration_seconds = exit_timestamp.timestamp() - entry_timestamp.timestamp();
    duration_seconds.max(MIN_SESSION_DURATION_SECONDS) as u64
}

/// Validation result for session operations
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Warning(String),
    Error(String),
}

/// Validate that timestamps are in correct chronological order
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
        
        // Check for suspiciously long durations (more than 24 hours)
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

/// Validate that a calculated duration is reasonable
pub fn validate_duration(duration_seconds: u64) -> ValidationResult {
    // Check for suspiciously long durations (more than 24 hours)
    if duration_seconds > 24 * 60 * 60 {
        let warning_msg = format!(
            "Unusually long session duration: {} seconds ({} hours)",
            duration_seconds, duration_seconds / 3600
        );
        warn!("{}", warning_msg);
        return ValidationResult::Warning(warning_msg);
    }
    
    // Check for zero duration (should be at least minimum)
    if duration_seconds == 0 {
        let error_msg = "Session duration cannot be zero".to_string();
        warn!("{}", error_msg);
        return ValidationResult::Error(error_msg);
    }
    
    ValidationResult::Valid
}

/// Validate that a new session doesn't overlap with existing sessions
pub fn validate_no_session_overlap(
    new_entry: DateTime<Utc>,
    new_exit: Option<DateTime<Utc>>,
    existing_sessions: &[(DateTime<Utc>, Option<DateTime<Utc>>)],
) -> ValidationResult {
    let new_exit = new_exit.unwrap_or_else(Utc::now);
    
    for (existing_entry, existing_exit) in existing_sessions {
        let existing_exit = existing_exit.unwrap_or_else(Utc::now);
        
        // Check for overlap: new session starts before existing ends AND new session ends after existing starts
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

/// Validate session data integrity
pub fn validate_session_data(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: Option<DateTime<Utc>>,
    duration_seconds: Option<u64>,
) -> ValidationResult {
    // Validate timestamp order
    match validate_timestamp_order(entry_timestamp, exit_timestamp) {
        ValidationResult::Error(msg) => return ValidationResult::Error(msg),
        ValidationResult::Warning(msg) => {
            // Log warning but continue validation
            warn!("Timestamp validation warning: {}", msg);
        }
        ValidationResult::Valid => {}
    }
    
    // Validate duration if provided
    if let Some(duration) = duration_seconds {
        match validate_duration(duration) {
            ValidationResult::Error(msg) => return ValidationResult::Error(msg),
            ValidationResult::Warning(msg) => {
                // Log warning but continue validation
                warn!("Duration validation warning: {}", msg);
            }
            ValidationResult::Valid => {}
        }
        
        // Cross-validate duration with timestamps if both are available
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