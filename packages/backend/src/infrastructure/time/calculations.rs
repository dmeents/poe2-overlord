use chrono::{DateTime, Utc};
use log::{debug, warn};

const MIN_SESSION_DURATION_SECONDS: i64 = 1;

/// Ensures minimum 1 second duration, clamps negatives to zero
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

/// Uses current time as exit timestamp for ongoing sessions
pub fn calculate_active_session_duration_seconds(entry_timestamp: DateTime<Utc>) -> u64 {
    let now = Utc::now();
    calculate_session_duration_seconds(entry_timestamp, now)
}

pub fn calculate_session_duration_from_timestamps(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: DateTime<Utc>,
) -> u64 {
    let duration_seconds = exit_timestamp.timestamp() - entry_timestamp.timestamp();
    duration_seconds.max(MIN_SESSION_DURATION_SECONDS) as u64
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    Valid,
    Warning(String),
    Error(String),
}

/// Flags unusually long sessions (>24h) as warnings
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
                duration.num_hours(),
                entry_timestamp,
                exit
            );
            warn!("{}", warning_msg);
            return ValidationResult::Warning(warning_msg);
        }
    }

    ValidationResult::Valid
}

pub fn validate_duration(duration_seconds: u64) -> ValidationResult {
    if duration_seconds > 24 * 60 * 60 {
        let warning_msg = format!(
            "Unusually long session duration: {} seconds ({} hours)",
            duration_seconds,
            duration_seconds / 3600
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

/// Uses current time for ongoing sessions when exit is None
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

/// Validates timestamp order, duration, and consistency between provided and calculated values
pub fn validate_session_data(
    entry_timestamp: DateTime<Utc>,
    exit_timestamp: Option<DateTime<Utc>>,
    duration_seconds: Option<u64>,
) -> ValidationResult {
    match validate_timestamp_order(entry_timestamp, exit_timestamp) {
        ValidationResult::Error(msg) => return ValidationResult::Error(msg),
        ValidationResult::Warning(msg) => {
            warn!("Timestamp validation warning: {}", msg);
        }
        ValidationResult::Valid => {}
    }

    if let Some(duration) = duration_seconds {
        match validate_duration(duration) {
            ValidationResult::Error(msg) => return ValidationResult::Error(msg),
            ValidationResult::Warning(msg) => {
                warn!("Duration validation warning: {}", msg);
            }
            ValidationResult::Valid => {}
        }

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
