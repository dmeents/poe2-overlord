use super::models::{WalkthroughAct, WalkthroughGuide, WalkthroughProgress, WalkthroughStep};

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

// ============= WalkthroughGuide Helper Method Tests =============

fn create_test_guide_for_navigation() -> WalkthroughGuide {
    WalkthroughGuide {
        acts: vec![
            WalkthroughAct {
                act_name: "Act 1".to_string(),
                steps: vec![
                    WalkthroughStep {
                        id: "act_1_step_1".to_string(),
                        title: "Step 1".to_string(),
                        description: "First step".to_string(),
                        objectives: vec![],
                        current_zone: "Zone 1".to_string(),
                        completion_zone: "Zone 2".to_string(),
                        links: vec![],
                    },
                    WalkthroughStep {
                        id: "act_1_step_2".to_string(),
                        title: "Step 2".to_string(),
                        description: "Second step".to_string(),
                        objectives: vec![],
                        current_zone: "Zone 2".to_string(),
                        completion_zone: "Zone 3".to_string(),
                        links: vec![],
                    },
                ],
            },
            WalkthroughAct {
                act_name: "Act 2".to_string(),
                steps: vec![
                    WalkthroughStep {
                        id: "act_2_step_1".to_string(),
                        title: "Step 3".to_string(),
                        description: "Third step".to_string(),
                        objectives: vec![],
                        current_zone: "Zone 3".to_string(),
                        completion_zone: "Zone 4".to_string(),
                        links: vec![],
                    },
                ],
            },
        ],
    }
}

#[test]
fn test_find_step_first_step() {
    let guide = create_test_guide_for_navigation();
    let result = guide.find_step("act_1_step_1");
    assert_eq!(result, Some((0, 0)));
}

#[test]
fn test_find_step_middle_step() {
    let guide = create_test_guide_for_navigation();
    let result = guide.find_step("act_1_step_2");
    assert_eq!(result, Some((0, 1)));
}

#[test]
fn test_find_step_cross_act_boundary() {
    let guide = create_test_guide_for_navigation();
    let result = guide.find_step("act_2_step_1");
    assert_eq!(result, Some((1, 0)));
}

#[test]
fn test_find_step_not_found() {
    let guide = create_test_guide_for_navigation();
    let result = guide.find_step("nonexistent_step");
    assert_eq!(result, None);
}

#[test]
fn test_step_exists_true() {
    let guide = create_test_guide_for_navigation();
    assert!(guide.step_exists("act_1_step_1"));
    assert!(guide.step_exists("act_1_step_2"));
    assert!(guide.step_exists("act_2_step_1"));
}

#[test]
fn test_step_exists_false() {
    let guide = create_test_guide_for_navigation();
    assert!(!guide.step_exists("nonexistent_step"));
}

#[test]
fn test_first_step_id() {
    let guide = create_test_guide_for_navigation();
    assert_eq!(guide.first_step_id(), Some("act_1_step_1"));
}

#[test]
fn test_first_step_id_empty_guide() {
    let guide = WalkthroughGuide { acts: vec![] };
    assert_eq!(guide.first_step_id(), None);
}

#[test]
fn test_next_step_id_within_act() {
    let guide = create_test_guide_for_navigation();
    let result = guide.next_step_id("act_1_step_1");
    assert_eq!(result, Some("act_1_step_2".to_string()));
}

#[test]
fn test_next_step_id_cross_act_boundary() {
    let guide = create_test_guide_for_navigation();
    let result = guide.next_step_id("act_1_step_2");
    assert_eq!(result, Some("act_2_step_1".to_string()));
}

#[test]
fn test_next_step_id_last_step() {
    let guide = create_test_guide_for_navigation();
    let result = guide.next_step_id("act_2_step_1");
    assert_eq!(result, None);
}

#[test]
fn test_next_step_id_not_found() {
    let guide = create_test_guide_for_navigation();
    let result = guide.next_step_id("nonexistent_step");
    assert_eq!(result, None);
}

#[test]
fn test_previous_step_id_within_act() {
    let guide = create_test_guide_for_navigation();
    let result = guide.previous_step_id("act_1_step_2");
    assert_eq!(result, Some("act_1_step_1".to_string()));
}

#[test]
fn test_previous_step_id_cross_act_boundary() {
    let guide = create_test_guide_for_navigation();
    let result = guide.previous_step_id("act_2_step_1");
    assert_eq!(result, Some("act_1_step_2".to_string()));
}

#[test]
fn test_previous_step_id_first_step() {
    let guide = create_test_guide_for_navigation();
    let result = guide.previous_step_id("act_1_step_1");
    assert_eq!(result, None);
}

#[test]
fn test_previous_step_id_not_found() {
    let guide = create_test_guide_for_navigation();
    let result = guide.previous_step_id("nonexistent_step");
    assert_eq!(result, None);
}
