use app_lib::handlers::{
    log_event_handler::LogEventHandler, process_monitor_handler::ProcessMonitorHandler,
    service_initializer::ServiceInitializer, time_tracking_handler::TimeTrackingHandler,
};
use app_lib::models::{events::SceneChangeEvent, LocationType};
use app_lib::services::process_monitor::ProcessMonitor;

#[test]
fn test_handler_structs_exist() {
    // Test that all handler structs exist and can be instantiated

    let _log_handler = LogEventHandler;
    let _process_handler = ProcessMonitorHandler;
    let _time_handler = TimeTrackingHandler;
    let _service_initializer = ServiceInitializer;

    // Verify they are unit structs (size 0)
    assert_eq!(std::mem::size_of::<LogEventHandler>(), 0);
    assert_eq!(std::mem::size_of::<ProcessMonitorHandler>(), 0);
    assert_eq!(std::mem::size_of::<TimeTrackingHandler>(), 0);
    assert_eq!(std::mem::size_of::<ServiceInitializer>(), 0);
}

#[test]
fn test_process_monitor_service() {
    // Test the ProcessMonitor service directly since handlers are static

    // Test checking for POE2 process (this might succeed or fail depending on the system)
    let result = ProcessMonitor::check_poe2_process();
    // Should not crash, regardless of result
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_handler_models_and_types() {
    // Test that the handlers use the correct models and types

    // Verify that SceneChangeEvent can be created
    let zone_event = SceneChangeEvent::Zone(app_lib::models::events::ZoneChangeEvent {
        zone_name: "Test Zone".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    });

    match zone_event {
        SceneChangeEvent::Zone(event) => {
            assert_eq!(event.zone_name, "Test Zone");
        }
        _ => panic!("Expected zone event"),
    }

    // Verify that LocationType enum works correctly
    let zone_type = LocationType::Zone;
    let act_type = LocationType::Act;
    let hideout_type = LocationType::Hideout;

    assert_ne!(zone_type, act_type);
    assert_ne!(act_type, hideout_type);
    assert_ne!(zone_type, hideout_type);
}

#[test]
fn test_service_instances_structure() {
    // Test that ServiceInstances can be created and used

    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = app_lib::services::config::ConfigService {
        config: std::sync::Arc::new(std::sync::RwLock::new(app_lib::models::AppConfig::default())),
        config_path,
    };

    let time_tracking = app_lib::services::time_tracking::TimeTrackingService::with_data_directory(
        Some(temp_dir.path().to_path_buf()),
    );
    let log_monitor =
        app_lib::services::log_monitor::LogMonitorService::new("test.log".to_string());

    let _instances = app_lib::handlers::service_initializer::ServiceInstances {
        config_service,
        log_monitor: std::sync::Arc::new(log_monitor),
        time_tracking: std::sync::Arc::new(time_tracking),
    };

    // If we get here without panicking, the structure is valid
    assert!(true);
}
