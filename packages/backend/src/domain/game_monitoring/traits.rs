use crate::domain::game_monitoring::{
    events::GameMonitoringEvent,
    models::GameProcessStatus,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

/// Trait for publishing game monitoring domain events
#[async_trait]
pub trait GameMonitoringEventPublisher: Send + Sync {
    /// Publish a game monitoring domain event
    async fn publish_event(&self, event: GameMonitoringEvent) -> AppResult<()>;
    
    /// Subscribe to game monitoring events
    fn subscribe_to_events(&self) -> broadcast::Receiver<GameMonitoringEvent>;
}

/// Trait for detecting game processes
#[async_trait]
pub trait ProcessDetector: Send + Sync {
    /// Check if the Path of Exile 2 game process is currently running
    async fn check_game_process(&self) -> AppResult<GameProcessStatus>;
    
    /// Get the configuration for process detection
    fn get_config(&self) -> &crate::domain::game_monitoring::models::GameMonitoringConfig;
}

/// Trait for the game monitoring domain service
#[async_trait]
pub trait GameMonitoringService: Send + Sync {
    /// Handle a detected game process status change
    async fn handle_process_status_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()>;
    
    /// Start monitoring the game process
    async fn start_monitoring(&self) -> AppResult<()>;
    
    /// Stop monitoring the game process
    async fn stop_monitoring(&self) -> AppResult<()>;
    
    /// Check if monitoring is currently active
    async fn is_monitoring(&self) -> bool;
    
    /// Get the current game process status
    async fn get_current_status(&self) -> Option<GameProcessStatus>;
}
