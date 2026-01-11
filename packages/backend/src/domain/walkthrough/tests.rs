use super::models::WalkthroughProgress;

#[test]
fn test_walkthrough_progress_new() {
    let progress = WalkthroughProgress::new();
    assert_eq!(progress.current_step_id, Some("act_1_step_1".to_string()));
    assert!(!progress.is_completed);
}

#[test]
fn test_walkthrough_progress_completed() {
    let progress = WalkthroughProgress::completed();
    assert_eq!(progress.current_step_id, None);
    assert!(progress.is_completed);
}

#[test]
fn test_walkthrough_progress_set_current_step() {
    let mut progress = WalkthroughProgress::new();
    progress.set_current_step("act_4_step_5".to_string());
    assert_eq!(progress.current_step_id, Some("act_4_step_5".to_string()));
    assert!(!progress.is_completed);
}

#[test]
fn test_walkthrough_progress_mark_completed() {
    let mut progress = WalkthroughProgress::new();
    progress.mark_completed();
    assert_eq!(progress.current_step_id, None);
    assert!(progress.is_completed);
}

#[test]
fn test_walkthrough_progress_touch() {
    let mut progress = WalkthroughProgress::new();
    let original_time = progress.last_updated;

    // Wait a small amount to ensure time difference
    std::thread::sleep(std::time::Duration::from_millis(1));

    progress.touch();
    assert!(progress.last_updated > original_time);
}
