//! # Service Orchestrator
//!
//! Manages the lifecycle and coordination of background services in the POE2 Overlord application.
//! This module provides functions to start and manage long-running background tasks that handle
//! monitoring, data processing, and event emission.
//!
//! ## Background Services
//!
//! The orchestrator manages four main types of background services:
//!
//! ### 1. Game Process Monitoring
//! - **Purpose**: Detects when Path of Exile 2 is running and monitors game state
//! - **Implementation**: Uses process detection to identify game instances
//! - **Integration**: Coordinates with time tracking and event publishing services
//!
//! ### 2. Log Monitoring
//! - **Purpose**: Real-time analysis of game log files to extract events and data
//! - **Implementation**: File system monitoring with log parsing and event extraction
//! - **Integration**: Updates character data and server status based on log events
//!
//! ### 3. Time Tracking Emission
//! - **Purpose**: Periodically sends time tracking data to the frontend for display
//! - **Implementation**: Scheduled emission of character play time data
//! - **Integration**: Uses time tracking service to gather and format data
//!
//! ### 4. Server Ping Monitoring
//! - **Purpose**: Monitors server connectivity and network status
//! - **Implementation**: Periodic ping checks to game servers
//! - **Integration**: Updates server status and notifies frontend of connectivity changes
//!
//! ## Task Management Strategy
//!
//! - **Runtime Manager**: Handles task spawning and lifecycle management
//! - **Task Manager**: Provides task tracking and coordination capabilities
//! - **Error Handling**: Comprehensive error handling with logging for all background tasks
//! - **Graceful Shutdown**: Tasks are designed to handle shutdown signals properly

use log::{error, info};
use std::sync::Arc;
use tauri::WebviewWindow;

use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::domain::log_analysis::traits::LogAnalysisService;
use crate::domain::character_tracking::traits::CharacterTrackingService;
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::runtime::{RuntimeManager, TaskManager};

/// Starts the game process monitoring service as a background task.
///
/// This function spawns a background task that continuously monitors for Path of Exile 2
/// game processes. When a game process is detected, it automatically starts monitoring
/// the game state, character activities, and play time tracking.
///
/// # Arguments
///
/// * `_window` - Webview window (unused in current implementation)
/// * `game_monitoring_service` - The game monitoring service instance
/// * `runtime_manager` - Runtime manager for task lifecycle management
/// * `_task_manager` - Task manager (unused in current implementation)
///
/// # Behavior
///
/// - Automatically starts monitoring on application startup
/// - Runs continuously in the background
/// - Handles errors gracefully with logging
/// - Integrates with time tracking and event publishing services
pub fn start_game_process_monitoring(
    _window: WebviewWindow,
    game_monitoring_service: Arc<dyn GameMonitoringService>,
    runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let service_clone = game_monitoring_service.clone();

    // Spawn the game monitoring task using the runtime manager
    let _handle = runtime_manager.spawn_background_task(
        "game_process_monitoring_setup".to_string(),
        move || async move {
            info!("Automatically starting game monitoring on application startup");
            if let Err(e) = service_clone.start_monitoring().await {
                error!("Failed to start game monitoring: {}", e);
            }
        },
    );
}

/// Starts the time tracking data emission service as a background task.
///
/// This function spawns a background task that periodically emits time tracking data
/// to the frontend. The service gathers character play time data and sends it to the
/// frontend for display in the UI.
///
/// # Arguments
///
/// * `window` - Webview window for sending events to the frontend
/// * `time_tracking` - The time tracking service instance
/// * `_runtime_manager` - Runtime manager (unused in current implementation)
/// * `_task_manager` - Task manager (unused in current implementation)
///
/// # Behavior
///
/// - Periodically gathers time tracking data from all characters
/// - Emits data to the frontend via Tauri events
/// - Runs continuously in the background
/// - Handles frontend communication automatically
pub fn start_character_tracking_emission(
    window: WebviewWindow,
    character_tracking: Arc<dyn CharacterTrackingService>,
    runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let character_tracking_clone = character_tracking.clone();

    // Spawn the character tracking emission task using the runtime manager
    runtime_manager.spawn_background_task(
        "character_tracking_emission".to_string(),
        move || async move {
            character_tracking_clone
                .start_frontend_event_emission(window)
                .await;
        },
    );
}

/// Starts the server ping monitoring service as a background task.
///
/// This function spawns a background task that periodically pings game servers to
/// monitor connectivity and server status. The service tracks network connectivity
/// and notifies the frontend of any status changes.
///
/// # Arguments
///
/// * `_window` - Webview window (unused in current implementation)
/// * `server_status` - The server monitor service instance
/// * `_runtime_manager` - Runtime manager (unused in current implementation)
/// * `_task_manager` - Task manager (unused in current implementation)
///
/// # Behavior
///
/// - Periodically pings game servers to check connectivity
/// - Tracks server status and network health
/// - Emits status change events to the frontend
/// - Runs continuously in the background
pub fn start_ping_event_emission(
    _window: WebviewWindow,
    server_status: Arc<ServerMonitor>,
    runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let server_status_clone = server_status.clone();

    // Spawn the server ping monitoring task using the runtime manager
    runtime_manager.spawn_background_task(
        "server_ping_monitoring".to_string(),
        move || async move {
            server_status_clone.start_periodic_ping().await;
        },
    );
}

/// Starts the log monitoring service as a background task.
///
/// This function spawns a background task that continuously monitors game log files
/// for new entries. The service parses log entries to extract game events, character
/// activities, and other relevant data for the application.
///
/// # Arguments
///
/// * `log_monitor` - The log analyzer service instance
/// * `_runtime_manager` - Runtime manager (unused in current implementation)
/// * `_task_manager` - Task manager (unused in current implementation)
///
/// # Behavior
///
/// - Monitors game log files for new entries in real-time
/// - Parses log entries to extract game events and character data
/// - Updates character information and server status based on log events
/// - Handles errors gracefully with comprehensive logging
/// - Runs continuously in the background
pub fn start_log_monitoring(
    log_monitor: Arc<dyn LogAnalysisService>,
    runtime_manager: Arc<RuntimeManager>,
    _task_manager: Arc<TaskManager>,
) {
    let log_monitor_clone = log_monitor.clone();

    // Spawn the log monitoring task using the runtime manager
    runtime_manager.spawn_background_task("log_monitoring".to_string(), move || async move {
        if let Err(e) = log_monitor_clone.start_monitoring().await {
            error!("Failed to start log monitoring: {}", e);
        }
    });
}
