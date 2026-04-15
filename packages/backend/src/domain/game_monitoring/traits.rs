use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ProcessDetector: Send + Sync {
    async fn check_game_process(&self) -> AppResult<GameProcessStatus>;

    fn get_config(&self) -> &crate::domain::game_monitoring::models::GameMonitoringConfig;
}

#[async_trait]
pub trait GameMonitoringService: Send + Sync {
    /// Coordinates with time tracking and publishes events on state changes
    async fn handle_process_status_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()>;

    async fn start_monitoring(&self) -> AppResult<()>;

    async fn stop_monitoring(&self) -> AppResult<()>;

    async fn is_monitoring(&self) -> bool;

    async fn get_current_status(&self) -> Option<GameProcessStatus>;

    /// Performs an immediate check of the game process status.
    /// Unlike `get_current_status()` which returns cached data, this always checks live.
    async fn check_status_now(&self) -> AppResult<GameProcessStatus>;
}
