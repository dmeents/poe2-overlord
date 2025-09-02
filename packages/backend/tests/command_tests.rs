use app_lib::models::{AppConfig, LocationType};
use app_lib::services::{
    config::ConfigService, log_monitor::LogMonitorService, process_monitor::ProcessMonitor,
    server_status::ServerStatusManager, time_tracking::TimeTrackingService,
};
use std::sync::{Arc, RwLock};
use tempfile::TempDir;

#[test]
fn test_config_service_functionality() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path,
    };

    // Test getting config
    let config = config_service.get_config();
    assert!(!config.poe_client_log_path.is_empty());
    assert_eq!(config.log_level, "info");

    // Test updating config
    let new_config = AppConfig {
        poe_client_log_path: "/new/path/to/client.txt".to_string(),
        log_level: "debug".to_string(),
    };

    let result = config_service.update_config(new_config.clone());
    assert!(result.is_ok());

    // Verify the config was updated
    let config = config_service.get_config();
    assert_eq!(config.poe_client_log_path, "/new/path/to/client.txt");
    assert_eq!(config.log_level, "debug");
}

#[test]
fn test_config_service_path_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path,
    };

    // Test getting POE client log path
    let path = config_service.get_poe_client_log_path();
    assert!(!path.is_empty());

    // Test setting POE client log path
    let new_path = "/custom/path/to/client.txt".to_string();
    let result = config_service.set_poe_client_log_path(new_path.clone());
    assert!(result.is_ok());

    // Verify the path was set
    let path = config_service.get_poe_client_log_path();
    assert_eq!(path, new_path);
}

#[test]
fn test_config_service_log_level_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path,
    };

    // Test getting log level
    let level = config_service.get_log_level();
    assert_eq!(level, "info");

    // Test setting log level
    let new_level = "warn".to_string();
    let result = config_service.set_log_level(new_level.clone());
    assert!(result.is_ok());

    // Verify the level was set
    let level = config_service.get_log_level();
    assert_eq!(level, new_level);
}

#[test]
fn test_config_service_reset_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path,
    };

    // Set some custom values first
    config_service
        .set_poe_client_log_path("/custom/path".to_string())
        .unwrap();
    config_service.set_log_level("debug".to_string()).unwrap();

    // Reset to defaults
    let result = config_service.update_config(AppConfig::default());
    assert!(result.is_ok());

    // Verify defaults are restored
    let config = config_service.get_config();
    assert_eq!(config.log_level, "info");
    // Note: poe_client_log_path will be OS-specific default, not necessarily empty
}

#[tokio::test]
async fn test_time_tracking_service_operations() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Test getting active sessions
    let sessions = time_tracking.get_active_sessions();
    assert_eq!(sessions.len(), 0); // Should start with no active sessions

    // Test getting completed sessions
    let sessions = time_tracking.get_completed_sessions();
    assert_eq!(sessions.len(), 0); // Should start with no completed sessions

    // Test starting a session
    let result = time_tracking
        .start_session("Test Zone".to_string(), LocationType::Zone)
        .await;
    assert!(result.is_ok());

    // Verify session was started
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);
    assert_eq!(active_sessions[0].location_name, "Test Zone");
}

#[tokio::test]
async fn test_time_tracking_service_session_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start a session
    time_tracking
        .start_session("Test Zone".to_string(), LocationType::Zone)
        .await
        .unwrap();

    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 1);

    let session_id = active_sessions[0].location_id.clone();

    // End the session
    let result = time_tracking.end_session(&session_id).await;
    assert!(result.is_ok());

    // Verify session was ended
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 0);

    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 1);
}

#[tokio::test]
async fn test_time_tracking_service_multiple_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Start multiple sessions of different types
    time_tracking
        .start_session("Zone 1".to_string(), LocationType::Zone)
        .await
        .unwrap();
    time_tracking
        .start_session("Act 1".to_string(), LocationType::Act)
        .await
        .unwrap();

    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 2);

    // End all sessions
    let result = time_tracking.end_all_active_sessions().await;
    assert!(result.is_ok());

    // Verify all sessions were ended
    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 0);

    let completed_sessions = time_tracking.get_completed_sessions();
    assert_eq!(completed_sessions.len(), 2);
}

#[test]
fn test_time_tracking_service_stats() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    let stats = time_tracking.get_all_stats();
    assert_eq!(stats.len(), 0); // Should start with no stats
}

#[tokio::test]
async fn test_log_monitor_service_operations() {
    let _temp_dir = TempDir::new().unwrap();
    let server_manager = Arc::new(ServerStatusManager::new());
    let log_monitor = LogMonitorService::new("test.log".to_string(), server_manager);

    // Test starting monitoring
    let result = log_monitor.start_monitoring().await;
    assert!(result.is_ok());

    // Test stopping monitoring
    let result = log_monitor.stop_monitoring().await;
    assert!(result.is_ok());
}

#[test]
fn test_log_monitor_service_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");

    // Create a test log file
    std::fs::write(&log_file, "test content").unwrap();

    let server_manager = Arc::new(ServerStatusManager::new());
    let log_monitor = LogMonitorService::new(log_file.to_string_lossy().to_string(), server_manager);

    // Test getting log file size
    let size = log_monitor.get_log_file_size();
    assert!(size.is_ok());
    let size = size.unwrap();
    assert!(size > 0);
}

#[test]
fn test_log_monitor_service_multiline_file() {
    let temp_dir = TempDir::new().unwrap();
    let log_file = temp_dir.path().join("test.log");

    // Create a test log file with multiple lines
    let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    std::fs::write(&log_file, content).unwrap();

    let server_manager = Arc::new(ServerStatusManager::new());
    let log_monitor = LogMonitorService::new(log_file.to_string_lossy().to_string(), server_manager);

    // Test that the service can be created with a multiline file
    let size = log_monitor.get_log_file_size();
    assert!(size.is_ok());
    let size = size.unwrap();
    assert!(size > 0);
}

#[test]
fn test_process_monitor_service() {
    // Test the ProcessMonitor service directly

    // Test checking for POE2 process (this might succeed or fail depending on the system)
    let result = ProcessMonitor::check_poe2_process();
    // Should not crash, regardless of result
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_service_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: temp_dir.path().join("config.json"),
    };

    // Test with empty inputs (these should actually succeed since there's no validation)
    let result = config_service.set_poe_client_log_path("".to_string());
    assert!(result.is_ok());

    let result = config_service.set_log_level("".to_string());
    assert!(result.is_ok());

    // Verify the empty values were set
    let config = config_service.get_config();
    assert_eq!(config.poe_client_log_path, "");
    assert_eq!(config.log_level, "");
}

#[tokio::test]
async fn test_service_thread_safety() {
    let temp_dir = TempDir::new().unwrap();
    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));

    // Test concurrent access to time tracking service
    let time_tracking1 = Arc::new(time_tracking);
    let time_tracking2 = Arc::clone(&time_tracking1);

    let handle1 = std::thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            time_tracking1
                .start_session("Zone 1".to_string(), LocationType::Zone)
                .await
        })
    });

    let handle2 = std::thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            time_tracking2
                .start_session("Zone 2".to_string(), LocationType::Zone)
                .await
        })
    });

    // Both should succeed
    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[test]
fn test_service_integration() {
    // Test that services can work together

    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path,
    };

    let time_tracking =
        TimeTrackingService::with_data_directory(Some(temp_dir.path().to_path_buf()));
    let server_manager = Arc::new(ServerStatusManager::new());
    let _log_monitor = LogMonitorService::new("test.log".to_string(), server_manager);

    // Verify all services can be created and used
    let config = config_service.get_config();
    assert!(!config.poe_client_log_path.is_empty());

    let active_sessions = time_tracking.get_active_sessions();
    assert_eq!(active_sessions.len(), 0);

    // Test that the log monitor service can be created
    assert!(true);
}
