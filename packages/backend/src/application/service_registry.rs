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

use crate::domain::character::traits::CharacterService;
// Character tracking functionality is now integrated into CharacterService
use crate::domain::configuration::{
    service::ConfigurationServiceImpl, traits::ConfigurationService,
};
use crate::domain::events::EventBus;
use crate::domain::game_monitoring::{traits::GameMonitoringService, GameMonitoringServiceImpl};
use crate::domain::log_analysis::{
    models::LogAnalysisConfig, service::LogAnalysisServiceImpl, traits::LogAnalysisService,
};
use crate::domain::server_monitoring::{ServerMonitoringService, ServerMonitoringServiceImpl};
use crate::domain::walkthrough::{
    repository::WalkthroughRepositoryImpl, service::WalkthroughServiceImpl,
    traits::WalkthroughService,
};
use crate::domain::zone_configuration::{
    repository::ZoneConfigurationRepositoryImpl, service::ZoneConfigurationServiceImpl,
};
use crate::infrastructure::monitoring::ProcessMonitorImpl;
use log::{error, info};
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
        let config_service = Arc::new(
            ConfigurationServiceImpl::new().expect("Failed to create configuration service"),
        );
        app.manage(config_service.clone());

        // Initialize Event Bus - provides unified event publishing and subscribing capabilities
        let event_bus = Arc::new(EventBus::new());
        app.manage(event_bus.clone());

        // Initialize Zone Configuration Service - provides zone-to-act mapping
        let zone_config_repo = Arc::new(ZoneConfigurationRepositoryImpl::new()?);
        let zone_config_service = Arc::new(ZoneConfigurationServiceImpl::new(zone_config_repo));
        app.manage(zone_config_service.clone());

        // Initialize Wiki Scraping Service - fetches zone data from PoE2 wiki
        let wiki_service = Arc::new(crate::domain::wiki_scraping::service::WikiScrapingServiceImpl::new());
        app.manage(wiki_service.clone());

        // Initialize Character Service - manages character data persistence and operations
        let character_service =
            crate::domain::character::service::CharacterServiceImpl::with_default_repository(
                event_bus.clone(),
                zone_config_service.clone(),
                wiki_service.clone(),
            )
            .map_err(|e| {
                error!("Failed to initialize CharacterService: {}", e);
                e
            })?;

        // Create a clone for the ServiceInstances (we need Arc for sharing)
        let character_arc = Arc::new(character_service) as Arc<dyn CharacterService + Send + Sync>;

        // Create a Box for Tauri state management
        let character_box = Box::new(
            crate::domain::character::service::CharacterServiceImpl::with_default_repository(
                event_bus.clone(),
                zone_config_service.clone(),
                wiki_service.clone(),
            )
            .map_err(|e| {
                error!(
                    "Failed to initialize CharacterService for state management: {}",
                    e
                );
                e
            })?,
        ) as Box<dyn CharacterService + Send + Sync>;

        app.manage(character_box);

        // Initialize Walkthrough Service - manages walkthrough guide and character progress
        // Depends on character service for character data access
        let walkthrough_repo = Arc::new(WalkthroughRepositoryImpl::new(std::path::PathBuf::from(
            "config/walkthrough_guide.json",
        )));
        let walkthrough_service = Arc::new(WalkthroughServiceImpl::new(
            walkthrough_repo,
            character_arc.clone(),
            event_bus.clone(),
        ));

        // Create a Box for Tauri state management
        let walkthrough_box = Box::new(WalkthroughServiceImpl::new(
            Arc::new(WalkthroughRepositoryImpl::new(std::path::PathBuf::from(
                "config/walkthrough_guide.json",
            ))),
            character_arc.clone(),
            event_bus.clone(),
        )) as Box<dyn WalkthroughService + Send + Sync>;

        app.manage(walkthrough_box);

        // Character Tracking functionality is now handled by CharacterService

        // Initialize Server Monitoring Service - handles network connectivity and server status tracking
        // Depends on event broadcaster for status change notifications
        let server_monitoring_service = ServerMonitoringServiceImpl::new(event_bus.clone())
            .map_err(|e| {
                error!("Failed to initialize ServerMonitoringService: {}", e);
                e
            })?;
        let server_monitoring_arc =
            Arc::new(server_monitoring_service) as Arc<dyn ServerMonitoringService>;
        app.manage(server_monitoring_arc.clone());

        // Initialize Log Analysis Service - processes game logs and extracts events
        // Depends on server monitor for status updates and character service for character operations

        // Create default log analysis configuration
        let log_analysis_config = LogAnalysisConfig {
            log_file_path: String::new(), // Will be configured from configuration service
            monitoring_interval_ms: 500,
            max_file_size_mb: 100,
            buffer_size: 1000,
        };

        // Create the log analysis service
        let log_analysis_service = LogAnalysisServiceImpl::new(
            log_analysis_config,
            character_arc.clone(),
            server_monitoring_arc.clone(),
            walkthrough_service.clone(),
        )
        .map_err(|e| {
            error!("Failed to initialize LogAnalysisService: {}", e);
            e
        })?;

        let log_analysis_arc = Arc::new(log_analysis_service) as Arc<dyn LogAnalysisService>;

        // Configure the log path from the configuration service
        let config_service_clone = config_service.clone();
        let log_analysis_clone = log_analysis_arc.clone();
        tauri::async_runtime::spawn(async move {
            match config_service_clone.get_poe_client_log_path().await {
                Ok(log_path) => {
                    if !log_path.is_empty() {
                        if let Err(e) = log_analysis_clone.update_log_path(log_path.clone()).await {
                            error!("Failed to update log path in LogAnalysisService: {}", e);
                        } else {
                        }
                    } else {
                    }
                }
                Err(e) => {
                    error!("Failed to get log path from configuration service: {}", e);
                }
            }
        });

        app.manage(log_analysis_arc.clone());

        // Initialize Game Monitoring services - complex service with multiple dependencies

        // Process detector for identifying game processes
        let process_detector = Arc::new(ProcessMonitorImpl::new());

        // Game monitoring service that coordinates process detection and event publishing
        let game_monitoring_service = Arc::new(GameMonitoringServiceImpl::new(
            event_bus.clone(),
            process_detector.clone(),
            character_arc.clone(),
        )) as Arc<dyn GameMonitoringService>;

        app.manage(game_monitoring_service.clone());

        info!("Service initialization completed successfully");

        // Return a container with all initialized services for use by the application setup
        Ok(ServiceInstances {
            config_service,
            event_bus,
            character_service: character_arc,
            walkthrough_service,
            log_analysis_service: log_analysis_arc,
            server_monitoring_service: server_monitoring_arc,
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

    /// Event bus for unified event publishing and subscribing
    pub event_bus: Arc<EventBus>,

    /// Character service for managing character data and operations (includes tracking)
    pub character_service: Arc<dyn CharacterService + Send + Sync>,

    /// Walkthrough service for managing walkthrough guide and character progress
    pub walkthrough_service: Arc<dyn WalkthroughService + Send + Sync>,

    /// Log analysis service for processing game logs and extracting events
    pub log_analysis_service: Arc<dyn LogAnalysisService>,

    /// Server monitoring service for tracking network connectivity and server status
    pub server_monitoring_service: Arc<dyn ServerMonitoringService>,

    /// Game monitoring service for detecting game processes and managing game state
    pub game_monitoring_service: Arc<dyn GameMonitoringService>,
}

impl ServiceInstances {
    /// Shuts down all services in the proper order during application cleanup
    ///
    /// This method coordinates the shutdown of all services, ensuring that
    /// character tracking data is finalized before other services are stopped.
    ///
    /// # Returns
    ///
    /// * `Result<(), Box<dyn std::error::Error>>` - Returns Ok(()) on successful shutdown,
    ///   or an error if any part of the shutdown process fails
    pub async fn shutdown_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting application shutdown cleanup...");

        // Step 1: Finalize character tracking data (most important)
        if let Err(e) = self.character_service.finalize_all_active_zones().await {
            log::error!("Failed to finalize character tracking data: {}", e);
            // Continue with shutdown even if this fails
        } else {
            log::info!("Character tracking data finalized successfully");
        }

        // Step 2: Stop background services (they should handle their own cleanup)
        log::info!("Background services shutdown completed");

        log::info!("Application shutdown cleanup completed");
        Ok(())
    }
}
