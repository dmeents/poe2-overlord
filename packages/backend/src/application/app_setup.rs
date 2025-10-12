//! # Application Setup
//!
//! Handles the complete application bootstrap process for the POE2 Overlord application.
//! This module coordinates service initialization, configuration setup, logging configuration,
//! and the startup of background services.
//!
//! ## Bootstrap Process
//!
//! The application setup follows this sequence:
//! 1. **Service Initialization**: Initialize all services through the service registry
//! 2. **Configuration Loading**: Load and apply application configuration settings
//! 3. **Logging Setup**: Configure logging levels and output based on configuration
//! 4. **Data Loading**: Load persisted character time tracking data asynchronously
//! 5. **Runtime Management**: Initialize runtime and task management systems
//! 6. **Background Services**: Start all background monitoring and processing services
//!
//! ## Key Features
//!
//! - **Dynamic Logging Configuration**: Log levels are loaded from configuration and applied at runtime
//! - **Asynchronous Data Loading**: Character data is loaded in the background to avoid blocking startup
//! - **Service Orchestration**: All background services are started in a coordinated manner
//! - **Error Handling**: Comprehensive error handling with detailed logging throughout the process
//!
//! ## Background Services Started
//!
//! - **Log Monitoring**: Real-time analysis of game log files
//! - **Game Process Monitoring**: Detection and tracking of game processes
//! - **Time Tracking Emission**: Periodic emission of time tracking data to frontend
//! - **Server Ping Monitoring**: Periodic server connectivity checks

use log::{debug, error, info, warn};
use std::sync::Arc;
use tauri::Manager;

use crate::application::service_orchestrator::{
    start_character_tracking_emission, start_game_process_monitoring, start_log_monitoring,
    start_ping_event_emission,
};
use crate::application::service_registry::ServiceInitializer;
use crate::domain::configuration::traits::ConfigurationService;
use crate::infrastructure::runtime::{RuntimeManager, TaskManager};
use crate::infrastructure::tauri::TauriEventBridge;

/// Sets up the complete application with all services, configuration, and background tasks.
///
/// This is the main entry point for application initialization. It orchestrates the entire
/// bootstrap process including service initialization, configuration loading, logging setup,
/// and background service startup.
///
/// # Arguments
///
/// * `app` - Mutable reference to the Tauri application instance
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns Ok(()) on successful setup,
///   or an error if any part of the setup process fails
///
/// # Process Flow
///
/// 1. Initialize all services through the service registry
/// 2. Load configuration and set up logging
/// 3. Start asynchronous data loading
/// 4. Initialize runtime management systems
/// 5. Start all background services
pub fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Initialize all application services through the dependency injection container
    let services = ServiceInitializer::initialize_services(app)?;

    // Register services container for shutdown cleanup
    app.manage(services.clone());

    // Step 2: Load configuration and set up logging
    let config_service = services.config_service.clone();

    // Load log level from configuration - this allows dynamic logging configuration
    let log_level = tauri::async_runtime::block_on(async {
        config_service
            .get_log_level()
            .await
            .unwrap_or_else(|_| "info".to_string())
    })
    .to_lowercase();

    // Convert string log level to log::LevelFilter enum
    let level_filter = match log_level.as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" | "warning" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => {
            eprintln!("Invalid log level '{}', defaulting to Info", log_level);
            log::LevelFilter::Info
        }
    };

    // Configure logging plugin for Tauri (only in debug builds)
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(level_filter)
                .filter(|metadata| {
                    // Suppress verbose debug logging from HTML parsing crates
                    if metadata.target().starts_with("selectors") || 
                       metadata.target().starts_with("html5ever") {
                        return false;
                    }
                    true
                })
                .build(),
        )?;
    }

    info!("Starting application setup...");

    info!(
        "Logging configured with level: {} ({:?})",
        log_level, level_filter
    );

    // Step 3: Start asynchronous data loading to avoid blocking application startup
    debug!("Loading existing character tracking data...");
    tauri::async_runtime::spawn(async move {
        // Note: Character tracking data is loaded on-demand, no bulk loading needed
        info!("Character tracking service initialized");
    });

    // Step 4: Initialize runtime management systems for background task coordination
    let runtime_manager = Arc::new(RuntimeManager::new()?);
    let task_manager = Arc::new(TaskManager::new());

    // Register runtime management systems with Tauri's state management
    app.manage(runtime_manager.clone());
    app.manage(task_manager.clone());

    // Step 5: Start all background services
    if let Some(main_window) = app.get_webview_window("main") {
        info!("Starting background services");

        // Initialize Tauri Event Bridge to forward events to frontend
        let event_bridge = TauriEventBridge::new(services.event_bus.clone(), main_window.clone());
        if let Err(e) = tauri::async_runtime::block_on(event_bridge.start_forwarding()) {
            error!("Failed to start Tauri event bridge: {}", e);
        } else {
            info!("Tauri event bridge started successfully");
        }

        // Start log monitoring service - analyzes game logs in real-time
        start_log_monitoring(
            services.log_analysis_service.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start game process monitoring - detects and tracks game processes
        start_game_process_monitoring(
            main_window.clone(),
            services.game_monitoring_service.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start character tracking emission - periodically sends tracking data to frontend
        start_character_tracking_emission(
            main_window.clone(),
            services.character_service.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        // Start server ping monitoring - periodically checks server connectivity
        start_ping_event_emission(
            main_window.clone(),
            services.server_monitoring_service.clone(),
            runtime_manager.clone(),
            task_manager.clone(),
        );

        info!("Background services started successfully");
    } else {
        warn!("Main window not found during setup");
    }

    info!("Application setup completed successfully");

    // Register shutdown handler for cleanup when main window is closed
    if let Some(main_window) = app.get_webview_window("main") {
        let services_clone = services.clone();
        main_window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                log::info!("Application shutdown requested");

                // Run shutdown cleanup in a blocking manner
                tauri::async_runtime::block_on(async {
                    if let Err(e) = services_clone.shutdown_services().await {
                        log::error!("Error during application shutdown: {}", e);
                    }
                });
            }
        });
    }

    Ok(())
}
