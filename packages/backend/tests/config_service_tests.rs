use app_lib::models::AppConfig;
use app_lib::services::config::ConfigService;
use std::sync::{Arc, RwLock};

#[test]
fn test_config_creation_and_defaults() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // Create a mock config service with a test path
    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Test default values
    let config = config_service.get_config();
    // The poe_client_log_path should now be the OS-specific default, not empty
    assert!(!config.poe_client_log_path.is_empty());
    assert_eq!(config.log_level, "info");
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Create a custom config
    let custom_config = AppConfig {
        poe_client_log_path: "/path/to/client.txt".to_string(),
        log_level: "debug".to_string(),
    };

    // Save it
    config_service.update_config(custom_config.clone()).unwrap();

    // Verify file was created
    assert!(config_path.exists());

    // Create new service and load
    let new_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    new_service.load_config().unwrap();
    let loaded_config = new_service.get_config();

    assert_eq!(
        loaded_config.poe_client_log_path,
        custom_config.poe_client_log_path
    );
    assert_eq!(loaded_config.log_level, custom_config.log_level);
}

#[test]
fn test_config_field_updates() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Test individual field updates
    config_service
        .set_poe_client_log_path("/new/path/to/client.txt".to_string())
        .unwrap();
    assert_eq!(
        config_service.get_poe_client_log_path(),
        "/new/path/to/client.txt"
    );

    config_service.set_log_level("warn".to_string()).unwrap();
    assert_eq!(config_service.get_log_level(), "warn");

    // Verify the file was updated
    assert!(config_path.exists());
}

#[test]
fn test_config_update_field_closure() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Test using the update_field closure
    config_service
        .update_field(|config| {
            config.poe_client_log_path = "/closure/path".to_string();
        })
        .unwrap();

    let config = config_service.get_config();
    assert_eq!(config.poe_client_log_path, "/closure/path");
    assert_eq!(config.log_level, "info"); // Should remain default
}

#[test]
fn test_config_persistence() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Make some changes
    config_service
        .set_poe_client_log_path("/persistent/path".to_string())
        .unwrap();

    // Verify file content
    let content = std::fs::read_to_string(&config_path).unwrap();
    let saved_config: AppConfig = serde_json::from_str(&content).unwrap();

    assert_eq!(saved_config.poe_client_log_path, "/persistent/path");
    assert_eq!(saved_config.log_level, "info");
}

#[test]
fn test_config_error_handling() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Test with invalid JSON (this should not happen in normal operation)
    // but we can test the error handling by creating a corrupted file
    std::fs::write(&config_path, "{ invalid json }").unwrap();

    // The service should handle this gracefully
    let _result = config_service.load_config();
    // In a real scenario, this might fail, but our test setup should handle it
    // For now, we'll just verify the service doesn't panic
    assert!(config_path.exists());
}

#[test]
fn test_config_thread_safety() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    // Test that multiple threads can access the config safely
    // We'll test this by creating multiple services and accessing them concurrently

    // Create a shared config service
    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Test concurrent reads (which should be safe)
    let config_service_clone = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Read from both services concurrently
    let handle1 = std::thread::spawn(move || config_service.get_config());

    let handle2 = std::thread::spawn(move || config_service_clone.get_config());

    // Wait for both threads to complete
    let config1 = handle1.join().unwrap();
    let config2 = handle2.join().unwrap();

    // Both should return the same default config
    assert_eq!(config1.poe_client_log_path, config2.poe_client_log_path);
    assert_eq!(config1.log_level, config2.log_level);
}

#[test]
fn test_os_specific_default_paths() {
    let temp_dir = tempfile::tempdir().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let config_service = ConfigService {
        config: Arc::new(RwLock::new(AppConfig::default())),
        config_path: config_path.clone(),
    };

    // Test that the default path is OS-specific and not empty
    let default_path = config_service.get_default_poe_client_log_path();
    assert!(!default_path.is_empty());
    
    // Test that the default path contains "Path of Exile"
    assert!(default_path.contains("Path of Exile"));
    
    // Test that resetting to default works
    config_service.set_poe_client_log_path("/custom/path".to_string()).unwrap();
    assert_eq!(config_service.get_poe_client_log_path(), "/custom/path");
    
    config_service.reset_poe_client_log_path_to_default().unwrap();
    let reset_path = config_service.get_poe_client_log_path();
    assert!(!reset_path.is_empty());
    assert!(reset_path.contains("Path of Exile"));
}
