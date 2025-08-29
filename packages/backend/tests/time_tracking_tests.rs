use app_lib::services::time_tracking::TimeTrackingService;
use app_lib::models::LocationType;
use tempfile::TempDir;



#[tokio::test]
async fn test_time_tracking_service_creation() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    assert_eq!(time_tracking.get_active_sessions().len(), 0);
    assert_eq!(time_tracking.get_completed_sessions().len(), 0);
    assert_eq!(time_tracking.get_all_stats().len(), 0);
}

#[tokio::test]
async fn test_start_and_end_zone_session() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start a zone session
    time_tracking.start_session("Test Zone".to_string(), LocationType::Zone).await.unwrap();
    
    // Check that session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_name, "Test Zone");
    assert_eq!(active_sessions[0].location_type, LocationType::Zone);
    assert!(active_sessions[0].exit_timestamp.is_none());
    
    // Add a small delay to ensure session has duration
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    
    // End the session
    let location_id = active_sessions[0].location_id.clone();
    time_tracking.end_session(&location_id).await.unwrap();
    
    // Check that session is no longer active
    assert_eq!(time_tracking.get_active_sessions().len(), 0);
    
    // Check that session is in completed sessions
    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert!(completed_sessions[0].exit_timestamp.is_some());
    assert!(completed_sessions[0].duration_seconds.is_some());
    assert!(completed_sessions[0].duration_seconds.unwrap() > 0);
}

#[tokio::test]
async fn test_start_and_end_act_session() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start an act session
    time_tracking.start_session("Act 1".to_string(), LocationType::Act).await.unwrap();
    
    // Check that session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_name, "Act 1");
    assert_eq!(active_sessions[0].location_type, LocationType::Act);
    
    // Add a small delay to ensure session has duration
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    
    // End the session
    let location_id = active_sessions[0].location_id.clone();
    time_tracking.end_session(&location_id).await.unwrap();
    
    // Check that session is completed
    assert_eq!(time_tracking.get_active_sessions().len(), 0);
    assert_eq!(time_tracking.get_completed_sessions().len(), 1);
}

#[tokio::test]
async fn test_zone_type_replacement() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start first zone session
    time_tracking.start_session("Zone 1".to_string(), LocationType::Zone).await.unwrap();
    
    // Start second zone session (should end the first one)
    time_tracking.start_session("Zone 2".to_string(), LocationType::Zone).await.unwrap();
    
    // Check that only one zone session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_name, "Zone 2");
    
    // Check that first session is completed
    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_name, "Zone 1");
}

#[tokio::test]
async fn test_act_type_replacement() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start first act session
    time_tracking.start_session("Act 1".to_string(), LocationType::Act).await.unwrap();
    
    // Start second act session (should end the first one)
    time_tracking.start_session("Act 2".to_string(), LocationType::Act).await.unwrap();
    
    // Check that only one act session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_name, "Act 2");
    
    // Check that first session is completed
    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_name, "Act 1");
}

#[tokio::test]
async fn test_concurrent_zone_and_act_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start both zone and act sessions
    time_tracking.start_session("Test Zone".to_string(), LocationType::Zone).await.unwrap();
    time_tracking.start_session("Act 1".to_string(), LocationType::Act).await.unwrap();
    
    // Check that both sessions are active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 2);
    
    // Verify we have one of each type
    let zone_count = active_sessions.iter().filter(|s| s.location_type == LocationType::Zone).count();
    let act_count = active_sessions.iter().filter(|s| s.location_type == LocationType::Act).count();
    assert_eq!(zone_count, 1);
    assert_eq!(act_count, 1);
}

#[tokio::test]
async fn test_location_id_generation() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start sessions and check ID generation
    time_tracking.start_session("Test Zone".to_string(), LocationType::Zone).await.unwrap();
    time_tracking.start_session("Act 1".to_string(), LocationType::Act).await.unwrap();
    
    let active_sessions = time_tracking.get_active_sessions();
    
    // Check zone ID
    let zone_session = active_sessions.iter().find(|s| s.location_type == LocationType::Zone).unwrap();
    assert_eq!(zone_session.location_id, "zone:test_zone");
    
    // Check act ID
    let act_session = active_sessions.iter().find(|s| s.location_type == LocationType::Act).unwrap();
    assert_eq!(act_session.location_id, "act:act_1");
}

#[tokio::test]
async fn test_stats_calculation() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start and end a session multiple times
    for _i in 1..=3 {
        time_tracking.start_session("Test Zone".to_string(), LocationType::Zone).await.unwrap();
        let active_sessions = time_tracking.get_active_sessions();
        let location_id = active_sessions[0].location_id.clone();
        
        // Wait a bit to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        
        time_tracking.end_session(&location_id).await.unwrap();
    }
    
    // Check stats
    let stats = time_tracking.get_all_stats();
    assert_eq!(stats.len(), 1);
    
    let zone_stats = &stats[0];
    assert_eq!(zone_stats.total_visits, 3);
    assert!(zone_stats.total_time_seconds > 0);
    assert!(zone_stats.average_session_seconds > 0.0);
    assert!(zone_stats.last_visited.is_some());
}

#[tokio::test]
async fn test_clear_all_data() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking = TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    
    // Start and end a session
    time_tracking.start_session("Test Zone".to_string(), LocationType::Zone).await.unwrap();
    let active_sessions = time_tracking.get_active_sessions();
    let location_id = active_sessions[0].location_id.clone();
    time_tracking.end_session(&location_id).await.unwrap();
    
    // Verify data exists
    assert_eq!(time_tracking.get_completed_sessions().len(), 1);
    assert_eq!(time_tracking.get_all_stats().len(), 1);
    
    // Clear all data
    time_tracking.clear_all_data().unwrap();
    
    // Verify data is cleared
    assert_eq!(time_tracking.get_active_sessions().len(), 0);
    assert_eq!(time_tracking.get_completed_sessions().len(), 0);
    assert_eq!(time_tracking.get_all_stats().len(), 0);
}
