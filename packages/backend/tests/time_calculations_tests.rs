use app_lib::utils::time_calculations::{
    calculate_active_session_duration_seconds,
    calculate_session_duration_seconds,
    calculate_session_duration_from_timestamps,
    validate_duration,
    validate_no_session_overlap,
    validate_session_data,
    validate_timestamp_order,
    ValidationResult,
};
use chrono::{TimeZone, Utc};

#[test]
fn test_calculate_session_duration_seconds() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 30).unwrap();
    
    let duration = calculate_session_duration_seconds(entry, exit);
    assert_eq!(duration, 330); // 5 minutes 30 seconds
}

#[test]
fn test_calculate_session_duration_minimum() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    
    let duration = calculate_session_duration_seconds(entry, exit);
    assert_eq!(duration, 1); // Minimum duration
}

#[test]
fn test_calculate_session_duration_millisecond_precision() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 1).unwrap() + chrono::Duration::milliseconds(500);
    
    let duration = calculate_session_duration_seconds(entry, exit);
    assert_eq!(duration, 1); // 1.5 seconds rounded down to 1 second
}

#[test]
fn test_calculate_active_session_duration() {
    let entry = Utc::now() - chrono::Duration::seconds(120);
    let duration = calculate_active_session_duration_seconds(entry);
    
    // Should be approximately 120 seconds (allowing for small timing differences)
    assert!((119..=121).contains(&duration));
}

#[test]
fn test_precision_consistency() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 1).unwrap() + chrono::Duration::milliseconds(500);
    
    // Test that both methods produce the same result
    let duration1 = calculate_session_duration_seconds(entry, exit);
    let duration2 = calculate_session_duration_from_timestamps(entry, exit);
    
    // Both should handle the 1.5 second duration the same way
    assert_eq!(duration1, duration2);
    assert_eq!(duration1, 1); // 1.5 seconds should round down to 1 second
}

#[test]
fn test_millisecond_precision_accuracy() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = entry + chrono::Duration::milliseconds(1500);
    
    let duration = calculate_session_duration_seconds(entry, exit);
    
    // 1500ms should be 1 second (truncated, not rounded)
    assert_eq!(duration, 1);
}

#[test]
fn test_negative_duration_handling() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 5).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    
    let duration = calculate_session_duration_seconds(entry, exit);
    
    // Negative duration should be clamped to minimum
    assert_eq!(duration, 1);
}

#[test]
fn test_large_duration_handling() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 13, 30, 45).unwrap();
    
    let duration = calculate_session_duration_seconds(entry, exit);
    
    // 1 hour 30 minutes 45 seconds = 5445 seconds
    assert_eq!(duration, 5445);
}

// Validation tests
#[test]
fn test_validate_timestamp_order_valid() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap();
    
    let result = validate_timestamp_order(entry, Some(exit));
    assert_eq!(result, ValidationResult::Valid);
}

#[test]
fn test_validate_timestamp_order_invalid() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    
    let result = validate_timestamp_order(entry, Some(exit));
    match result {
        ValidationResult::Error(_) => {} // Expected
        _ => panic!("Expected error for invalid timestamp order"),
    }
}

#[test]
fn test_validate_timestamp_order_long_duration_warning() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 2, 13, 0, 0).unwrap(); // 25 hours
    
    let result = validate_timestamp_order(entry, Some(exit));
    match result {
        ValidationResult::Warning(_) => {} // Expected
        _ => panic!("Expected warning for long duration"),
    }
}

#[test]
fn test_validate_duration_valid() {
    let result = validate_duration(300); // 5 minutes
    assert_eq!(result, ValidationResult::Valid);
}

#[test]
fn test_validate_duration_zero_error() {
    let result = validate_duration(0);
    match result {
        ValidationResult::Error(_) => {} // Expected
        _ => panic!("Expected error for zero duration"),
    }
}

#[test]
fn test_validate_duration_long_warning() {
    let result = validate_duration(25 * 60 * 60); // 25 hours
    match result {
        ValidationResult::Warning(_) => {} // Expected
        _ => panic!("Expected warning for long duration"),
    }
}

#[test]
fn test_validate_no_session_overlap_valid() {
    let new_entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let new_exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap();
    
    let existing_sessions = vec![
        (Utc.with_ymd_and_hms(2024, 1, 1, 11, 0, 0).unwrap(), 
         Some(Utc.with_ymd_and_hms(2024, 1, 1, 11, 30, 0).unwrap())),
    ];
    
    let result = validate_no_session_overlap(new_entry, Some(new_exit), &existing_sessions);
    assert_eq!(result, ValidationResult::Valid);
}

#[test]
fn test_validate_no_session_overlap_detected() {
    let new_entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let new_exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap();
    
    let existing_sessions = vec![
        (Utc.with_ymd_and_hms(2024, 1, 1, 11, 30, 0).unwrap(), 
         Some(Utc.with_ymd_and_hms(2024, 1, 1, 12, 2, 0).unwrap())),
    ];
    
    let result = validate_no_session_overlap(new_entry, Some(new_exit), &existing_sessions);
    match result {
        ValidationResult::Error(_) => {} // Expected
        _ => panic!("Expected error for session overlap"),
    }
}

#[test]
fn test_validate_session_data_valid() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap();
    let duration = Some(300);
    
    let result = validate_session_data(entry, Some(exit), duration);
    assert_eq!(result, ValidationResult::Valid);
}

#[test]
fn test_validate_session_data_duration_mismatch() {
    let entry = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
    let exit = Utc.with_ymd_and_hms(2024, 1, 1, 12, 5, 0).unwrap();
    let duration = Some(600); // Wrong duration (should be 300)
    
    let result = validate_session_data(entry, Some(exit), duration);
    match result {
        ValidationResult::Error(_) => {} // Expected
        _ => panic!("Expected error for duration mismatch"),
    }
}