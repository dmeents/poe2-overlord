use app_lib::models::events::SceneChangeEvent;
use app_lib::models::LocationType;
use app_lib::parsers::scene_change_parser::{LogParser, SceneChangeParser};
use app_lib::services::time_tracking::TimeTrackingService;
use tempfile::TempDir;

#[tokio::test]
async fn test_hideout_detection() {
    let parser = SceneChangeParser::new();

    // Test hideout detection
    let hideout_line = "[SCENE] Set Source [My Hideout]";
    let event = parser.parse_line(hideout_line);

    assert!(event.is_some());
    if let Some(SceneChangeEvent::Hideout(hideout_event)) = event {
        assert_eq!(hideout_event.hideout_name, "My Hideout");
    } else {
        panic!("Expected hideout event");
    }
}

#[tokio::test]
async fn test_hideout_time_tracking() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start a hideout session
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    // Verify hideout session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Hideout);
    assert_eq!(active_sessions[0].location_name, "My Hideout");

    // End the hideout session
    let location_id = active_sessions[0].location_id.clone();
    time_tracking
        .end_session(&location_id)
        .await
        .expect("Should end hideout session");

    // Verify session is completed
    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_type, LocationType::Hideout);
    assert_eq!(completed_sessions[0].location_name, "My Hideout");
    assert!(completed_sessions[0].duration_seconds.is_some());
}

#[tokio::test]
async fn test_hideout_ends_act_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start an act session first
    time_tracking
        .start_session("Act 1".to_string(), LocationType::Act)
        .await
        .expect("Should start act session");

    // Verify act session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Act);

    // Start a hideout session - this should end the act session
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    // Verify act session is ended and hideout session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Hideout);

    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_type, LocationType::Act);
}

#[tokio::test]
async fn test_hideout_location_id_generation() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start a hideout session
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    // Verify the location ID is generated correctly
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_id, "hideout:my_hideout");
}

#[tokio::test]
async fn test_hideout_stats_tracking() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start and end a hideout session
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    let active_sessions = time_tracking.get_active_sessions();
    let location_id = active_sessions[0].location_id.clone();

    time_tracking
        .end_session(&location_id)
        .await
        .expect("Should end hideout session");

    // Verify stats are generated
    let stats = time_tracking.get_location_stats(&location_id);
    assert!(stats.is_some());

    let stats = stats.unwrap();
    assert_eq!(stats.location_type, LocationType::Hideout);
    assert_eq!(stats.location_name, "My Hideout");
    assert_eq!(stats.total_visits, 1);
    assert!(stats.total_time_seconds > 0);
}

#[tokio::test]
async fn test_zone_ends_hideout_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start a hideout session first
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    // Verify hideout session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Hideout);

    // Start a zone session - this should end the hideout session
    time_tracking
        .start_session("The Coast".to_string(), LocationType::Zone)
        .await
        .expect("Should start zone session");

    // Verify hideout session is ended and zone session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Zone);

    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_type, LocationType::Hideout);
}

#[tokio::test]
async fn test_act_ends_hideout_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start a hideout session first
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    // Verify hideout session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Hideout);

    // Start an act session - this should end the hideout session
    time_tracking
        .start_session("Act 1".to_string(), LocationType::Act)
        .await
        .expect("Should start act session");

    // Verify hideout session is ended and act session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Act);

    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_type, LocationType::Hideout);
}

#[tokio::test]
async fn test_hideout_ends_zone_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start a zone session first
    time_tracking
        .start_session("The Coast".to_string(), LocationType::Zone)
        .await
        .expect("Should start zone session");

    // Verify zone session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Zone);

    // Start a hideout session - this should end the zone session
    time_tracking
        .start_session("My Hideout".to_string(), LocationType::Hideout)
        .await
        .expect("Should start hideout session");

    // Verify zone session is ended and hideout session is active
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_type, LocationType::Hideout);

    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
    assert_eq!(completed_sessions[0].location_type, LocationType::Zone);
}
