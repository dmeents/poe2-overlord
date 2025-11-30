use log::{error, info};
use std::sync::Arc;

use crate::domain::game_monitoring::traits::GameMonitoringService;
use crate::domain::log_analysis::traits::LogAnalysisService;
use crate::domain::server_monitoring::ServerMonitoringService;

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

pub fn start_ping_event_emission(server_monitoring_service: Arc<dyn ServerMonitoringService>) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = server_monitoring_service.start_ping_monitoring().await {
            error!("Failed to start server ping monitoring: {}", e);
        }
    });
}

pub fn start_log_monitoring(log_analysis_service: Arc<dyn LogAnalysisService>) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = log_analysis_service.start_monitoring().await {
            error!("Failed to start log monitoring: {}", e);
        }
    });
}
