use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::errors::AppResult;
use serde::{Deserialize, Serialize};

/// Command to start game process monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartGameMonitoring {
    /// Optional configuration for monitoring
    pub config: Option<crate::domain::game_monitoring::models::GameMonitoringConfig>,
}

impl StartGameMonitoring {
    pub fn new() -> Self {
        Self { config: None }
    }
    
    pub fn with_config(config: crate::domain::game_monitoring::models::GameMonitoringConfig) -> Self {
        Self { config: Some(config) }
    }
}

/// Command to stop game process monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopGameMonitoring;

impl StopGameMonitoring {
    pub fn new() -> Self {
        Self
    }
}

/// Command to check current game process status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckGameProcessStatus;

impl CheckGameProcessStatus {
    pub fn new() -> Self {
        Self
    }
}

/// Command to update game monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGameMonitoringConfig {
    pub config: crate::domain::game_monitoring::models::GameMonitoringConfig,
}

impl UpdateGameMonitoringConfig {
    pub fn new(config: crate::domain::game_monitoring::models::GameMonitoringConfig) -> Self {
        Self { config }
    }
}

/// Result type for game monitoring commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameMonitoringCommandResult {
    MonitoringStarted,
    MonitoringStopped,
    ProcessStatus(GameProcessStatus),
    ConfigUpdated,
}

/// Trait for handling game monitoring commands
#[async_trait::async_trait]
pub trait GameMonitoringCommandHandler: Send + Sync {
    /// Handle a start monitoring command
    async fn handle_start_monitoring(&self, command: StartGameMonitoring) -> AppResult<GameMonitoringCommandResult>;
    
    /// Handle a stop monitoring command
    async fn handle_stop_monitoring(&self, command: StopGameMonitoring) -> AppResult<GameMonitoringCommandResult>;
    
    /// Handle a check process status command
    async fn handle_check_status(&self, command: CheckGameProcessStatus) -> AppResult<GameMonitoringCommandResult>;
    
    /// Handle an update config command
    async fn handle_update_config(&self, command: UpdateGameMonitoringConfig) -> AppResult<GameMonitoringCommandResult>;
}
