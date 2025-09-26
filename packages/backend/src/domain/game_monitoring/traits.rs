use crate::domain::game_monitoring::models::GameProcessStatus;
use crate::errors::AppResult;
use async_trait::async_trait;

// Event publishing trait removed - using unified event system

/// Trait for detecting and checking the status of game processes.
///
/// This trait abstracts the platform-specific logic for finding and monitoring
/// Path of Exile 2 processes, allowing for different implementations across platforms.
#[async_trait]
pub trait ProcessDetector: Send + Sync {
    /// Checks the current status of the game process.
    ///
    /// Searches for running POE2 processes based on configured process names
    /// and returns the current status including PID, name, and running state.
    ///
    /// # Returns
    /// * `AppResult<GameProcessStatus>` - Current process status or error
    async fn check_game_process(&self) -> AppResult<GameProcessStatus>;

    /// Returns the configuration used for process detection.
    ///
    /// # Returns
    /// * `&GameMonitoringConfig` - Reference to the monitoring configuration
    fn get_config(&self) -> &crate::domain::game_monitoring::models::GameMonitoringConfig;
}

/// Main service trait for game process monitoring functionality.
///
/// This trait defines the core operations for monitoring POE2 game processes,
/// including starting/stopping monitoring loops and handling status changes.
#[async_trait]
pub trait GameMonitoringService: Send + Sync {
    /// Handles a change in process status, triggering appropriate actions.
    ///
    /// This method is called when a process status change is detected. It handles
    /// integration with time tracking services and publishes events to subscribers.
    ///
    /// # Arguments
    /// * `current_status` - The current process status
    /// * `previous_status` - The previous process status (None for first detection)
    ///
    /// # Returns
    /// * `AppResult<()>` - Success or error result
    async fn handle_process_status_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()>;

    /// Starts the background monitoring loop.
    ///
    /// Begins periodic checking of game process status at configured intervals.
    /// If monitoring is already running, this is a no-op.
    ///
    /// # Returns
    /// * `AppResult<()>` - Success or error result
    async fn start_monitoring(&self) -> AppResult<()>;

    /// Stops the background monitoring loop.
    ///
    /// Gracefully shuts down the monitoring task and waits for completion.
    /// If monitoring is not running, this is a no-op.
    ///
    /// # Returns
    /// * `AppResult<()>` - Success or error result
    async fn stop_monitoring(&self) -> AppResult<()>;

    /// Checks if the monitoring service is currently active.
    ///
    /// # Returns
    /// * `bool` - True if monitoring is running, false otherwise
    async fn is_monitoring(&self) -> bool;

    /// Gets the current process status if available.
    ///
    /// # Returns
    /// * `Option<GameProcessStatus>` - Current status or None if not detected
    async fn get_current_status(&self) -> Option<GameProcessStatus>;
}
