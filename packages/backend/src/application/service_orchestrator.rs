//! # Service Orchestrator
//!
//! Manages the lifecycle and coordination of background services in the POE2 Overlord application.
//! This module provides functions to start long-running background tasks that handle
//! monitoring, data processing, and event emission.
//!
//! ## Background Services
//!
//! The orchestrator manages three main types of background services:
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
//! ### 3. Server Ping Monitoring
//! - **Purpose**: Monitors server connectivity and network status
//! - **Implementation**: Periodic ping checks to game servers
//! - **Integration**: Updates server status and notifies frontend of connectivity changes
//!
//! ## Task Management
//!
//! Services manage their own task lifecycle internally. The orchestrator simply starts
//! them using Tauri's async runtime. Cleanup is handled via service shutdown methods.

use log::{error, info};
use std::sync::Arc;

use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::domain::log_analysis::traits::LogAnalysisService;
use crate::domain::server_monitoring::ServerMonitoringService;

/// Starts the game process monitoring service as a background task.
///
/// This function spawns a background task that continuously monitors for Path of Exile 2
/// game processes. When a game process is detected, it automatically starts monitoring
/// the game state, character activities, and play time tracking.
///
/// The service manages its own task lifecycle internally, including start/stop logic
/// and graceful shutdown.
///
/// # Arguments
///
/// * `game_monitoring_service` - The game monitoring service instance
pub fn start_game_process_monitoring(game_monitoring_service: Arc<dyn GameMonitoringService>) {
    tauri::async_runtime::spawn(async move {
        info!("Starting game monitoring on application startup");
        match game_monitoring_service.start_monitoring().await {
            Ok(_) => {
                info!("Game monitoring started successfully");
            }
            Err(e) => {
                error!("Failed to start game monitoring: {}", e);
            }
        }
    });
}

/// Starts the server ping monitoring service as a background task.
///
/// This function spawns a background task that periodically pings game servers to
/// monitor connectivity and server status. The service tracks network connectivity
/// and notifies the frontend of any status changes.
///
/// # Arguments
///
/// * `server_monitoring_service` - The server monitor service instance
pub fn start_ping_event_emission(server_monitoring_service: Arc<dyn ServerMonitoringService>) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = server_monitoring_service.start_ping_monitoring().await {
            error!("Failed to start server ping monitoring: {}", e);
        }
    });
}

/// Starts the log monitoring service as a background task.
///
/// This function spawns a background task that continuously monitors game log files
/// for new entries. The service parses log entries to extract game events, character
/// activities, and other relevant data for the application.
///
/// # Arguments
///
/// * `log_analysis_service` - The log analyzer service instance
pub fn start_log_monitoring(log_analysis_service: Arc<dyn LogAnalysisService>) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = log_analysis_service.start_monitoring().await {
            error!("Failed to start log monitoring: {}", e);
        }
    });
}
