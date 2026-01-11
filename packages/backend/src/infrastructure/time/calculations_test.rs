#[cfg(test)]
mod tests {
    use crate::infrastructure::time::calculations::*;
    use chrono::{Duration, Utc};

    // ============= calculate_session_duration_seconds Tests =============

    #[test]
    fn test_calculate_session_duration_seconds_basic() {
        let entry = Utc::now();
        let exit = entry + Duration::seconds(100);

        let duration = calculate_session_duration_seconds(entry, exit);

        assert_eq!(duration, 100);
    }

    #[test]
    fn test_calculate_session_duration_seconds_minimum() {
        let entry = Utc::now();
        let exit = entry + Duration::milliseconds(500); // Less than 1 second

        let duration = calculate_session_duration_seconds(entry, exit);

        // Should be minimum 1 second
        assert_eq!(duration, 1);
    }

    #[test]
    fn test_calculate_session_duration_seconds_negative_clamped() {
        let entry = Utc::now();
        let exit = entry - Duration::seconds(10); // Exit before entry

        let duration = calculate_session_duration_seconds(entry, exit);

        // Should be clamped to minimum 1 second
        assert_eq!(duration, 1);
    }

    #[test]
    fn test_calculate_session_duration_seconds_large_value() {
        let entry = Utc::now();
        let exit = entry + Duration::hours(24);

        let duration = calculate_session_duration_seconds(entry, exit);

        assert_eq!(duration, 86400); // 24 hours in seconds
    }

    // ============= calculate_active_session_duration_seconds Tests =============

    #[test]
    fn test_calculate_active_session_duration_seconds() {
        let entry = Utc::now() - Duration::seconds(60);

        let duration = calculate_active_session_duration_seconds(entry);

        // Should be approximately 60 seconds (allow some tolerance)
        assert!(duration >= 60 && duration <= 62);
    }

    // ============= calculate_session_duration_from_timestamps Tests =============

    #[test]
    fn test_calculate_session_duration_from_timestamps_basic() {
        let entry = Utc::now();
        let exit = entry + Duration::seconds(120);

        let duration = calculate_session_duration_from_timestamps(entry, exit);

        assert_eq!(duration, 120);
    }

    #[test]
    fn test_calculate_session_duration_from_timestamps_minimum() {
        let entry = Utc::now();
        let exit = entry + Duration::milliseconds(100);

        let duration = calculate_session_duration_from_timestamps(entry, exit);

        // Should be minimum 1 second
        assert_eq!(duration, 1);
    }

    // ============= ValidationResult Tests =============

    #[test]
    fn test_validation_result_equality() {
        assert_eq!(ValidationResult::Valid, ValidationResult::Valid);
        assert_eq!(
            ValidationResult::Warning("test".to_string()),
            ValidationResult::Warning("test".to_string())
        );
        assert_eq!(
            ValidationResult::Error("test".to_string()),
            ValidationResult::Error("test".to_string())
        );
    }

    #[test]
    fn test_validation_result_inequality() {
        assert_ne!(
            ValidationResult::Valid,
            ValidationResult::Warning("test".to_string())
        );
        assert_ne!(
            ValidationResult::Warning("a".to_string()),
            ValidationResult::Warning("b".to_string())
        );
    }

    // ============= validate_timestamp_order Tests =============

    #[test]
    fn test_validate_timestamp_order_valid() {
        let entry = Utc::now();
        let exit = entry + Duration::hours(1);

        let result = validate_timestamp_order(entry, Some(exit));

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_timestamp_order_no_exit() {
        let entry = Utc::now();

        let result = validate_timestamp_order(entry, None);

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_timestamp_order_exit_before_entry() {
        let entry = Utc::now();
        let exit = entry - Duration::hours(1);

        let result = validate_timestamp_order(entry, Some(exit));

        match result {
            ValidationResult::Error(msg) => {
                assert!(msg.contains("Invalid timestamp order"));
            }
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_timestamp_order_long_session_warning() {
        let entry = Utc::now();
        let exit = entry + Duration::hours(25); // More than 24 hours

        let result = validate_timestamp_order(entry, Some(exit));

        match result {
            ValidationResult::Warning(msg) => {
                assert!(msg.contains("Unusually long session"));
            }
            _ => panic!("Expected Warning, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_timestamp_order_exactly_24_hours() {
        let entry = Utc::now();
        let exit = entry + Duration::hours(24);

        let result = validate_timestamp_order(entry, Some(exit));

        // 24 hours exactly should be valid (not > 24)
        assert_eq!(result, ValidationResult::Valid);
    }

    // ============= validate_duration Tests =============

    #[test]
    fn test_validate_duration_valid() {
        let result = validate_duration(3600); // 1 hour

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_duration_zero() {
        let result = validate_duration(0);

        match result {
            ValidationResult::Error(msg) => {
                assert!(msg.contains("cannot be zero"));
            }
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_duration_too_long() {
        let result = validate_duration(25 * 60 * 60); // 25 hours

        match result {
            ValidationResult::Warning(msg) => {
                assert!(msg.contains("Unusually long"));
            }
            _ => panic!("Expected Warning, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_duration_exactly_24_hours() {
        let result = validate_duration(24 * 60 * 60); // Exactly 24 hours

        // Exactly 24 hours should be valid (not > 24)
        assert_eq!(result, ValidationResult::Valid);
    }

    // ============= validate_no_session_overlap Tests =============

    #[test]
    fn test_validate_no_session_overlap_no_overlap() {
        let new_entry = Utc::now();
        let new_exit = new_entry + Duration::hours(1);

        let existing = vec![(new_exit + Duration::hours(1), Some(new_exit + Duration::hours(2)))];

        let result = validate_no_session_overlap(new_entry, Some(new_exit), &existing);

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_no_session_overlap_with_overlap() {
        let base = Utc::now();
        let new_entry = base;
        let new_exit = base + Duration::hours(2);

        // Existing session that overlaps
        let existing = vec![(base + Duration::hours(1), Some(base + Duration::hours(3)))];

        let result = validate_no_session_overlap(new_entry, Some(new_exit), &existing);

        match result {
            ValidationResult::Error(msg) => {
                assert!(msg.contains("overlap"));
            }
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_no_session_overlap_empty_existing() {
        let new_entry = Utc::now();
        let new_exit = new_entry + Duration::hours(1);

        let result = validate_no_session_overlap(new_entry, Some(new_exit), &[]);

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_no_session_overlap_adjacent_sessions() {
        let base = Utc::now();
        let new_entry = base;
        let new_exit = base + Duration::hours(1);

        // Existing session immediately after
        let existing = vec![(new_exit, Some(new_exit + Duration::hours(1)))];

        let result = validate_no_session_overlap(new_entry, Some(new_exit), &existing);

        // Adjacent sessions (touching but not overlapping) should be valid
        assert_eq!(result, ValidationResult::Valid);
    }

    // ============= validate_session_data Tests =============

    #[test]
    fn test_validate_session_data_all_valid() {
        let entry = Utc::now();
        let exit = entry + Duration::seconds(100);

        let result = validate_session_data(entry, Some(exit), Some(100));

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_session_data_no_exit_no_duration() {
        let entry = Utc::now();

        let result = validate_session_data(entry, None, None);

        assert_eq!(result, ValidationResult::Valid);
    }

    #[test]
    fn test_validate_session_data_timestamp_error() {
        let entry = Utc::now();
        let exit = entry - Duration::hours(1); // Exit before entry

        let result = validate_session_data(entry, Some(exit), None);

        match result {
            ValidationResult::Error(msg) => {
                assert!(msg.contains("Invalid timestamp order"));
            }
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_session_data_duration_error() {
        let entry = Utc::now();

        let result = validate_session_data(entry, None, Some(0));

        match result {
            ValidationResult::Error(msg) => {
                assert!(msg.contains("cannot be zero"));
            }
            _ => panic!("Expected Error, got {:?}", result),
        }
    }

    #[test]
    fn test_validate_session_data_duration_mismatch() {
        let entry = Utc::now();
        let exit = entry + Duration::seconds(100);

        // Provide mismatched duration
        let result = validate_session_data(entry, Some(exit), Some(50));

        match result {
            ValidationResult::Error(msg) => {
                assert!(msg.contains("Duration mismatch"));
            }
            _ => panic!("Expected Error, got {:?}", result),
        }
    }
}
