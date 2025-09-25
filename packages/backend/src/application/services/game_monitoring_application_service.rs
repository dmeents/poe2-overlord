use crate::domain::game_monitoring::{
    models::GameProcessStatus,
    traits::{GameMonitoringService, ProcessDetector},
};
use crate::errors::AppResult;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

/// Application service that orchestrates the game monitoring use case
/// This service coordinates between domain services and infrastructure concerns
pub struct GameMonitoringApplicationService {
    /// Domain service for game monitoring business logic
    game_monitoring_service: Arc<dyn GameMonitoringService>,
    /// Infrastructure service for process detection
    process_detector: Arc<dyn ProcessDetector>,
}

impl GameMonitoringApplicationService {
    /// Create a new game monitoring application service
    pub fn new(
        game_monitoring_service: Arc<dyn GameMonitoringService>,
        process_detector: Arc<dyn ProcessDetector>,
    ) -> Self {
        Self {
            game_monitoring_service,
            process_detector,
        }
    }

    /// Start the game monitoring use case
    /// This orchestrates the monitoring loop that checks for process changes
    pub async fn start_monitoring(&self) -> AppResult<()> {
        info!("Starting game monitoring application service");
        
        // Start the domain service
        self.game_monitoring_service.start_monitoring().await?;
        
        // Get the check interval from the process detector configuration
        let check_interval = Duration::from_secs(self.process_detector.get_config().check_interval_seconds);
        let mut interval_timer = interval(check_interval);
        let mut previous_status: Option<GameProcessStatus> = None;
        
        info!("Game monitoring loop started with interval: {:?}", check_interval);
        
        loop {
            interval_timer.tick().await;
            
            // Check if monitoring is still active
            if !self.game_monitoring_service.is_monitoring().await {
                debug!("Game monitoring stopped, exiting monitoring loop");
                break;
            }
            
            // Check current process status
            match self.process_detector.check_game_process().await {
                Ok(current_status) => {
                    // Only handle state changes to avoid unnecessary processing
                    let is_state_change = previous_status
                        .as_ref()
                        .map(|prev| current_status.is_state_change(prev))
                        .unwrap_or(true);
                    
                    if is_state_change {
                        debug!(
                            "Game process state change detected: running={}, pid={}, name={}",
                            current_status.running, current_status.pid, current_status.name
                        );
                        
                        // Delegate to domain service for business logic
                        if let Err(e) = self.game_monitoring_service
                            .handle_process_status_change(current_status.clone(), previous_status.clone())
                            .await
                        {
                            error!("Failed to handle process status change: {}", e);
                        }
                    }
                    
                    previous_status = Some(current_status);
                }
                Err(e) => {
                    error!("Error checking game process: {}", e);
                }
            }
        }
        
        info!("Game monitoring application service stopped");
        Ok(())
    }

    /// Stop the game monitoring use case
    pub async fn stop_monitoring(&self) -> AppResult<()> {
        info!("Stopping game monitoring application service");
        self.game_monitoring_service.stop_monitoring().await?;
        Ok(())
    }

    /// Check if monitoring is currently active
    pub async fn is_monitoring(&self) -> bool {
        self.game_monitoring_service.is_monitoring().await
    }

    /// Get the current game process status
    pub async fn get_current_status(&self) -> Option<GameProcessStatus> {
        self.game_monitoring_service.get_current_status().await
    }

    /// Perform a one-time check of the game process status
    pub async fn check_process_status(&self) -> AppResult<GameProcessStatus> {
        self.process_detector.check_game_process().await
    }
}
