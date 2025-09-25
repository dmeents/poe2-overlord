//! # Service Registry
//! 
//! Implements the dependency injection container for the POE2 Overlord application.
//! This module is responsible for initializing, configuring, and registering all application services
//! with Tauri's application state management system.
//! 
//! ## Key Responsibilities
//! 
//! - **Service Initialization**: Creates and configures all domain and infrastructure services
//! - **Dependency Injection**: Manages service dependencies and ensures proper initialization order
//! - **State Management**: Registers services with Tauri's app state for global access
//! - **Error Handling**: Provides comprehensive error handling during service initialization
//! 
//! ## Service Initialization Order
//! 
//! Services are initialized in a specific order to respect dependencies:
//! 1. Configuration Service (no dependencies)
//! 2. Event Dispatcher (no dependencies) 
//! 3. Character Service (depends on configuration)
//! 4. Time Tracking Service (depends on configuration)
//! 5. Server Monitor (depends on event dispatcher)
//! 6. Log Analyzer (depends on server monitor and character service)
//! 7. Game Monitoring Service (depends on time tracking, event publisher, and process detector)
//! 
//! ## Error Handling Strategy
//! 
//! - Each service initialization is wrapped in error handling
//! - Failed service initialization causes the entire application startup to fail
//! - Detailed logging provides visibility into initialization progress and failures

use crate::domain::character::service::CharacterService;
use crate::domain::configuration::service::ConfigurationServiceImpl;
use crate::domain::game_monitoring::{traits::GameMonitoringService, GameMonitoringServiceImpl};
use crate::domain::time_tracking::{service::TimeTrackingServiceImpl, traits::TimeTrackingService};
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::parsing::LogAnalyzer;
use crate::infrastructure::tauri::EventDispatcher;
use crate::infrastructure::{
    monitoring::ProcessMonitorImpl, tauri::TauriGameMonitoringEventPublisher,
};
use log::{debug, error, info};
use std::sync::Arc;
use tauri::Manager;

/// Service initializer that implements the dependency injection container pattern.
/// 
/// This struct provides a centralized way to initialize all application services
/// in the correct dependency order and register them with Tauri's state management.
pub struct ServiceInitializer;

impl ServiceInitializer {
    /// Initializes all application services and registers them with Tauri's app state.
    /// 
    /// This method follows a specific initialization order to respect service dependencies:
    /// 1. Core services (configuration, event dispatcher) are initialized first
    /// 2. Domain services (character, time tracking) are initialized next
    /// 3. Infrastructure services (monitoring, logging) are initialized last
    /// 
    /// # Arguments
    /// 
    /// * `app` - Mutable reference to the Tauri application instance
    /// 
    /// # Returns
    /// 
    /// * `Result<ServiceInstances, Box<dyn std::error::Error>>` - Returns a container
    ///   with all initialized services on success, or an error if initialization fails
    /// 
    /// # Errors
    /// 
    /// This function will return an error if any service fails to initialize,
    /// as the application cannot function without all required services.
    pub fn initialize_services(
        app: &mut tauri::App,
    ) -> Result<ServiceInstances, Box<dyn std::error::Error>> {
        info!("Starting service initialization...");

        // Initialize Configuration Service first - it has no dependencies and is needed by other services
        debug!("Initializing ConfigurationService...");
        let config_service = Arc::new(
            ConfigurationServiceImpl::new().expect("Failed to create configuration service"),
        );
        app.manage(config_service.clone());
        debug!("ConfigurationService managed successfully");

        // Initialize Event Dispatcher - provides event broadcasting capabilities to other services
        debug!("Initializing EventDispatcher...");
        let event_broadcaster = Arc::new(EventDispatcher::new());
        app.manage(event_broadcaster.clone());
        debug!("EventDispatcher managed successfully");

        // Initialize Character Service - manages character data persistence and operations
        debug!("Initializing CharacterService...");
        let character_service = CharacterService::new().map_err(|e| {
            error!("Failed to initialize CharacterService: {}", e);
            e
        })?;
        let character_arc = Arc::new(character_service);
        app.manage(character_arc.clone());
        debug!("CharacterService managed successfully");

        // Initialize Time Tracking Service - handles play time monitoring and analytics
        debug!("Initializing TimeTrackingService...");
        let time_tracking_service = TimeTrackingServiceImpl::new().map_err(|e| {
            error!("Failed to initialize TimeTrackingService: {}", e);
            e
        })?;
        let time_tracking_arc = Arc::new(time_tracking_service) as Arc<dyn TimeTrackingService>;
        app.manage(time_tracking_arc.clone());
        debug!("TimeTrackingService managed successfully");

        // Initialize Server Monitor - handles network connectivity and server status tracking
        // Depends on event broadcaster for status change notifications
        debug!("Initializing ServerMonitor...");
        let server_status_manager = ServerMonitor::new(event_broadcaster.clone());
        let server_status_arc = Arc::new(server_status_manager);
        app.manage(server_status_arc.clone());
        debug!("ServerMonitor managed successfully");

        // Initialize Log Analyzer - processes game logs and extracts events
        // Depends on server monitor for status updates and character service for character operations
        debug!("Initializing LogAnalyzer...");
        let log_monitor_service = LogAnalyzer::new(
            "".to_string(), // Log path will be configured later
        );
        let log_monitor_arc = Arc::new(log_monitor_service);
        app.manage(log_monitor_arc.clone());
        debug!("LogAnalyzer managed successfully");

        // Initialize Game Monitoring services - complex service with multiple dependencies
        debug!("Initializing Game Monitoring services...");

        // Process detector for identifying game processes
        let process_detector = Arc::new(ProcessMonitorImpl::new());

        // Event publisher for sending game monitoring events to the frontend
        let event_publisher = Arc::new(TauriGameMonitoringEventPublisher::new(
            app.get_webview_window("main")
                .unwrap_or_else(|| panic!("Main window not found during service initialization")),
        ));

        // Game monitoring service that coordinates process detection, time tracking, and event publishing
        let game_monitoring_service = Arc::new(GameMonitoringServiceImpl::new(
            time_tracking_arc.clone(),
            event_publisher.clone(),
            process_detector.clone(),
        ));

        app.manage(game_monitoring_service.clone());
        debug!("Game Monitoring services managed successfully");

        info!("Service initialization completed successfully");

        // Return a container with all initialized services for use by the application setup
        Ok(ServiceInstances {
            config_service,
            event_broadcaster,
            character_service: character_arc,
            time_tracking_service: time_tracking_arc,
            log_monitor: log_monitor_arc,
            server_status: server_status_arc,
            game_monitoring_service,
        })
    }
}

/// Container for all initialized application services.
/// 
/// This struct holds references to all the services that have been initialized
/// and registered with Tauri's state management system. It provides a convenient
/// way to pass service instances around during application setup and orchestration.
/// 
/// All services are wrapped in `Arc` for thread-safe sharing across the application.
/// Domain services use trait objects (`dyn Trait`) to allow for different implementations
/// and better testability.
#[derive(Clone)]
pub struct ServiceInstances {
    /// Configuration service for managing application settings and preferences
    pub config_service: Arc<ConfigurationServiceImpl>,
    
    /// Event dispatcher for broadcasting events across the application
    pub event_broadcaster: Arc<EventDispatcher>,
    
    /// Character service for managing character data and operations
    pub character_service: Arc<CharacterService>,
    
    /// Time tracking service for monitoring and analyzing play time
    pub time_tracking_service: Arc<dyn TimeTrackingService>,
    
    /// Log analyzer for processing game logs and extracting events
    pub log_monitor: Arc<LogAnalyzer>,
    
    /// Server monitor for tracking network connectivity and server status
    pub server_status: Arc<ServerMonitor>,
    
    /// Game monitoring service for detecting game processes and managing game state
    pub game_monitoring_service: Arc<dyn GameMonitoringService>,
}
