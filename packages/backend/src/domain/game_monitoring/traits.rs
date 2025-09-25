use crate::domain::game_monitoring::{
    events::GameMonitoringEvent,
    models::GameProcessStatus,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait GameMonitoringEventPublisher: Send + Sync {
    async fn publish_event(&self, event: GameMonitoringEvent) -> AppResult<()>;
    
    fn subscribe_to_events(&self) -> broadcast::Receiver<GameMonitoringEvent>;
}

#[async_trait]
pub trait ProcessDetector: Send + Sync {
    async fn check_game_process(&self) -> AppResult<GameProcessStatus>;
    
    fn get_config(&self) -> &crate::domain::game_monitoring::models::GameMonitoringConfig;
}

#[async_trait]
pub trait GameMonitoringService: Send + Sync {
    async fn handle_process_status_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()>;
    
    async fn start_monitoring(&self) -> AppResult<()>;
    
    async fn stop_monitoring(&self) -> AppResult<()>;
    
    async fn is_monitoring(&self) -> bool;
    
    async fn get_current_status(&self) -> Option<GameProcessStatus>;
}
