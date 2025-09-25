use crate::application::services::GameMonitoringApplicationService;
use crate::errors::AppResult;
use log::info;
use std::sync::Arc;

/// Command to start game monitoring
/// This represents the application-level command for starting the game monitoring use case
pub struct StartGameMonitoringCommand {
    /// The application service that will handle the command
    application_service: Arc<GameMonitoringApplicationService>,
}

impl StartGameMonitoringCommand {
    /// Create a new start game monitoring command
    pub fn new(application_service: Arc<GameMonitoringApplicationService>) -> Self {
        Self {
            application_service,
        }
    }

    /// Execute the command to start game monitoring
    pub async fn execute(&self) -> AppResult<()> {
        info!("Executing start game monitoring command");
        self.application_service.start_monitoring().await
    }
}

/// Command to stop game monitoring
pub struct StopGameMonitoringCommand {
    /// The application service that will handle the command
    application_service: Arc<GameMonitoringApplicationService>,
}

impl StopGameMonitoringCommand {
    /// Create a new stop game monitoring command
    pub fn new(application_service: Arc<GameMonitoringApplicationService>) -> Self {
        Self {
            application_service,
        }
    }

    /// Execute the command to stop game monitoring
    pub async fn execute(&self) -> AppResult<()> {
        info!("Executing stop game monitoring command");
        self.application_service.stop_monitoring().await
    }
}

/// Command to check current game process status
pub struct CheckGameProcessStatusCommand {
    /// The application service that will handle the command
    application_service: Arc<GameMonitoringApplicationService>,
}

impl CheckGameProcessStatusCommand {
    /// Create a new check game process status command
    pub fn new(application_service: Arc<GameMonitoringApplicationService>) -> Self {
        Self {
            application_service,
        }
    }

    /// Execute the command to check game process status
    pub async fn execute(&self) -> AppResult<crate::domain::game_monitoring::models::GameProcessStatus> {
        info!("Executing check game process status command");
        self.application_service.check_process_status().await
    }
}
