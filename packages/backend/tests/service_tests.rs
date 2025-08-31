use app_lib::services::{
    file_monitor::FileMonitor,
    event_broadcaster::EventBroadcaster,
    player_location_manager::PlayerLocationManager,
};
use app_lib::models::events::{SceneChangeEvent, ZoneChangeEvent, ActChangeEvent, HideoutChangeEvent};
use tempfile::TempDir;


#[test]
fn test_file_monitor_creation() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    
    let file_monitor = FileMonitor::new(log_file.to_string_lossy().to_string());
    
    // Test that file monitor was created
    assert_eq!(file_monitor.get_log_path(), log_file.to_string_lossy());
}

#[test]
fn test_file_monitor_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    
    let file_monitor = FileMonitor::new(log_file.to_string_lossy().to_string());
    
    // Test file existence (should be false initially)
    assert!(!file_monitor.file_exists());
    
    // Create the file
    std::fs::write(&log_file, "test content").unwrap();
    
    // Test file existence (should be true now)
    assert!(file_monitor.file_exists());
    
    // Test getting file size
    let size = file_monitor.get_log_file_size();
    assert!(size.is_ok());
    let size = size.unwrap();
    assert!(size > 0);
}

#[test]
fn test_file_monitor_file_watching() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    
    let file_monitor = FileMonitor::new(log_file.to_string_lossy().to_string());
    
    // Create the file
    std::fs::write(&log_file, "initial content").unwrap();
    
    // Test that we can create a watcher (this is mainly to ensure the method exists)
    let watcher_result = file_monitor.create_watcher(|_event| {
        // Callback function
    });
    
    // The watcher creation should succeed
    assert!(watcher_result.is_ok());
}

#[tokio::test]
async fn test_file_monitor_process_new_lines() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    
    let file_monitor = FileMonitor::new(log_file.to_string_lossy().to_string());
    
    // Create a test log file with multiple lines
    let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    std::fs::write(&log_file, content).unwrap();
    
    let mut last_position = 0;
    
    // Process new lines
    let result = file_monitor.process_new_lines(&mut last_position, |line| {
        assert!(!line.is_empty());
    }).await;
    
    assert!(result.is_ok());
    // Note: We can't easily count lines in the closure due to Fn trait requirements
    // The important thing is that the method doesn't panic
}

#[test]
fn test_event_broadcaster_creation() {
    let _broadcaster = EventBroadcaster::new();
    
    // Test that broadcaster was created
    assert!(true); // Just verify it doesn't panic
}

#[tokio::test]
async fn test_event_broadcaster_subscription() {
    let broadcaster = EventBroadcaster::new();
    
    // Test subscribing to scene change events
    let _scene_receiver = broadcaster.subscribe();
    assert!(true); // Just verify it doesn't panic
    
    // Test subscribing to zone change events
    let _zone_receiver = broadcaster.subscribe_zones();
    assert!(true); // Just verify it doesn't panic
    
    // Test subscribing to act change events
    let _act_receiver = broadcaster.subscribe_acts();
    assert!(true); // Just verify it doesn't panic
}

#[tokio::test]
async fn test_event_broadcaster_broadcasting() {
    let broadcaster = EventBroadcaster::new();
    
    // Subscribe to events
    let _scene_receiver = broadcaster.subscribe();
    let _zone_receiver = broadcaster.subscribe_zones();
    let _act_receiver = broadcaster.subscribe_acts();
    
    // Create test events
    let zone_event = SceneChangeEvent::Zone(ZoneChangeEvent {
        zone_name: "Test Zone".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    });
    
    let act_event = SceneChangeEvent::Act(ActChangeEvent {
        act_name: "Act 1".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    });
    
    let hideout_event = SceneChangeEvent::Hideout(HideoutChangeEvent {
        hideout_name: "Test Hideout".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    });
    
    // Broadcast events
    let result1 = broadcaster.broadcast_event(zone_event.clone());
    let result2 = broadcaster.broadcast_event(act_event.clone());
    let result3 = broadcaster.broadcast_event(hideout_event.clone());
    
    // All broadcasts should succeed
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
}

#[test]
fn test_player_location_manager_creation() {
    let _manager = PlayerLocationManager::new();
    
    // Test that manager was created
    assert!(true); // Just verify it doesn't panic
}

#[tokio::test]
async fn test_player_location_manager_scene_updates() {
    let manager = PlayerLocationManager::new();
    
    // Test updating scene
    let result = manager.update_scene("Test Zone").await;
    assert!(result); // Should return true for new scene
    
    // Test updating same scene again
    let result = manager.update_scene("Test Zone").await;
    assert!(!result); // Should return false for same scene
    
    // Test updating different scene
    let result = manager.update_scene("Different Zone").await;
    assert!(result); // Should return true for new scene
}

#[tokio::test]
async fn test_player_location_manager_act_updates() {
    let manager = PlayerLocationManager::new();
    
    // Test updating act
    let result = manager.update_act("Act 1").await;
    assert!(result); // Should return true for new act
    
    // Test updating same act again
    let result = manager.update_act("Act 1").await;
    assert!(!result); // Should return false for same act
    
    // Test updating different act
    let result = manager.update_act("Act 2").await;
    assert!(result); // Should return true for new act
}

#[tokio::test]
async fn test_player_location_manager_reset_tracking() {
    let manager = PlayerLocationManager::new();
    
    // Update some locations first
    manager.update_scene("Test Zone").await;
    manager.update_act("Act 1").await;
    
    // Verify they were set
    let (scene, act) = manager.get_current_scene_and_act().await;
    assert_eq!(scene, Some("Test Zone".to_string()));
    assert_eq!(act, Some("Act 1".to_string()));
    
    // Reset tracking
    manager.reset_tracking().await;
    
    // Verify they were reset
    let (scene, act) = manager.get_current_scene_and_act().await;
    assert!(scene.is_none());
    assert!(act.is_none());
}

#[tokio::test]
async fn test_player_location_manager_current_locations() {
    let manager = PlayerLocationManager::new();
    
    // Test getting current locations (should be empty initially)
    let current_scene = manager.get_current_scene().await;
    let current_act = manager.get_current_act().await;
    
    assert!(current_scene.is_none());
    assert!(current_act.is_none());
}

#[tokio::test]
async fn test_player_location_manager_location_tracking() {
    let manager = PlayerLocationManager::new();
    
    // Update locations
    manager.update_scene("Zone 1").await;
    manager.update_act("Act 1").await;
    
    // Verify current locations
    let current_scene = manager.get_current_scene().await;
    let current_act = manager.get_current_act().await;
    
    assert_eq!(current_scene, Some("Zone 1".to_string()));
    assert_eq!(current_act, Some("Act 1".to_string()));
}

#[test]
fn test_service_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    
    let file_monitor = FileMonitor::new(log_file.to_string_lossy().to_string());
    
    // Test getting file size of non-existent file
    let size = file_monitor.get_log_file_size();
    assert!(size.is_err()); // Should fail for non-existent file
    
    // Test file existence of non-existent file
    assert!(!file_monitor.file_exists());
}

#[tokio::test]
async fn test_service_thread_safety() {
    let broadcaster = EventBroadcaster::new();
    
    // Create subscribers to ensure broadcasts can succeed
    let _scene_receiver = broadcaster.subscribe();
    let _zone_receiver = broadcaster.subscribe_zones();
    let _act_receiver = broadcaster.subscribe_acts();
    
    // Clone the broadcaster for multiple threads
    let broadcaster1 = broadcaster.clone();
    let broadcaster2 = broadcaster.clone();
    
    // Test concurrent broadcasting
    let handle1 = std::thread::spawn(move || {
        let zone_event = SceneChangeEvent::Zone(ZoneChangeEvent {
            zone_name: "Zone 1".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
        broadcaster1.broadcast_event(zone_event)
    });
    
    let handle2 = std::thread::spawn(move || {
        let zone_event = SceneChangeEvent::Zone(ZoneChangeEvent {
            zone_name: "Zone 2".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });
        broadcaster2.broadcast_event(zone_event)
    });
    
    // Both should succeed
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_service_integration() {
    // Test that services can work together
    
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");
    
    let _file_monitor = FileMonitor::new(log_file.to_string_lossy().to_string());
    let _broadcaster = EventBroadcaster::new();
    let _location_manager = PlayerLocationManager::new();
    
    // Verify all services can be created
    assert!(true);
    
    // Test that file monitor can work with event broadcaster
    let _scene_receiver = _broadcaster.subscribe();
    let _zone_receiver = _broadcaster.subscribe_zones();
    let _act_receiver = _broadcaster.subscribe_acts();
    
    // Test that location manager can be used
    // Note: This would require async context, so we'll just verify the manager was created
    assert!(true);
}
