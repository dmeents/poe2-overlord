use crate::domain::game_monitoring::{
    commands::{GameMonitoringCommandHandler, GameMonitoringCommandResult, StartGameMonitoring, StopGameMonitoring, CheckGameProcessStatus, UpdateGameMonitoringConfig},
    events::{GameMonitoringEvent, GameProcessStarted, GameProcessStopped, GameProcessStatusUpdated},
    models::{GameProcessStatus, GameMonitoringConfig},
    traits::{GameMonitoringEventPublisher, GameMonitoringService, ProcessDetector},
};
use crate::domain::time_tracking::traits::TimeTrackingService;
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Game monitoring domain service that handles business logic for game process monitoring
pub struct GameMonitoringServiceImpl {
    /// Service for tracking character sessions
    time_tracker: Arc<dyn TimeTrackingService>,
    /// Event publisher for domain events
    event_publisher: Arc<dyn GameMonitoringEventPublisher>,
    /// Process detector for checking game process status
    process_detector: Arc<dyn ProcessDetector>,
    /// Current monitoring state
    is_monitoring: Arc<RwLock<bool>>,
    /// Current game process status
    current_status: Arc<RwLock<Option<GameProcessStatus>>>,
    /// Configuration for monitoring
    config: Arc<RwLock<GameMonitoringConfig>>,
}

impl GameMonitoringServiceImpl {
    /// Create a new game monitoring service
    pub fn new(
        time_tracker: Arc<dyn TimeTrackingService>,
        event_publisher: Arc<dyn GameMonitoringEventPublisher>,
        process_detector: Arc<dyn ProcessDetector>,
    ) -> Self {
        Self {
            time_tracker,
            event_publisher,
            process_detector,
            is_monitoring: Arc::new(RwLock::new(false)),
            current_status: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(GameMonitoringConfig::default())),
        }
    }

    /// Handle the business logic when a game process status change is detected
    async fn handle_process_state_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()> {
        let is_state_change = previous_status
            .as_ref()
            .map(|prev| current_status.is_state_change(prev))
            .unwrap_or(true);

        if is_state_change {
            if current_status.is_running() {
                info!("POE2 process started - PID: {}, Name: {}", current_status.pid, current_status.name);
                
                // Business logic: Set the process start time for time tracking
                let start_time = chrono::DateTime::from(current_status.detected_at);
                self.time_tracker.set_poe_process_start_time(start_time).await;
                
                // Publish domain event
                let event = GameMonitoringEvent::ProcessStarted(GameProcessStarted::new(current_status.clone()));
                self.event_publisher.publish_event(event).await?;
            } else {
                info!("POE2 process stopped");
                debug!("POE2 process stopped, ending all active time tracking sessions");
                
                // Business logic: End all active sessions and clear process start time
                if let Err(e) = self.time_tracker.end_all_active_sessions_global().await {
                    error!("Failed to end active time tracking sessions: {}", e);
                }
                self.time_tracker.clear_poe_process_start_time().await;
                
                // Publish domain event
                let event = GameMonitoringEvent::ProcessStopped(GameProcessStopped::new(current_status.clone()));
                self.event_publisher.publish_event(event).await?;
            }
        }

        // Always publish status update event
        let status_event = GameMonitoringEvent::StatusUpdated(
            GameProcessStatusUpdated::new(current_status.clone(), is_state_change)
        );
        self.event_publisher.publish_event(status_event).await?;

        // Update current status
        {
            let mut status = self.current_status.write().await;
            *status = Some(current_status);
        }

        Ok(())
    }
}

#[async_trait]
impl GameMonitoringService for GameMonitoringServiceImpl {
    async fn handle_process_status_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()> {
        self.handle_process_state_change(current_status, previous_status).await
    }

    async fn start_monitoring(&self) -> AppResult<()> {
        let mut is_monitoring = self.is_monitoring.write().await;
        if *is_monitoring {
            debug!("Game monitoring is already running");
            return Ok(());
        }

        *is_monitoring = true;
        info!("Starting game process monitoring");
        Ok(())
    }

    async fn stop_monitoring(&self) -> AppResult<()> {
        let mut is_monitoring = self.is_monitoring.write().await;
        if !*is_monitoring {
            debug!("Game monitoring is not running");
            return Ok(());
        }

        *is_monitoring = false;
        info!("Stopping game process monitoring");
        Ok(())
    }

    async fn is_monitoring(&self) -> bool {
        *self.is_monitoring.read().await
    }

    async fn get_current_status(&self) -> Option<GameProcessStatus> {
        self.current_status.read().await.clone()
    }
}

#[async_trait]
impl GameMonitoringCommandHandler for GameMonitoringServiceImpl {
    async fn handle_start_monitoring(&self, _command: StartGameMonitoring) -> AppResult<GameMonitoringCommandResult> {
        self.start_monitoring().await?;
        Ok(GameMonitoringCommandResult::MonitoringStarted)
    }

    async fn handle_stop_monitoring(&self, _command: StopGameMonitoring) -> AppResult<GameMonitoringCommandResult> {
        self.stop_monitoring().await?;
        Ok(GameMonitoringCommandResult::MonitoringStopped)
    }

    async fn handle_check_status(&self, _command: CheckGameProcessStatus) -> AppResult<GameMonitoringCommandResult> {
        let status = self.process_detector.check_game_process().await?;
        Ok(GameMonitoringCommandResult::ProcessStatus(status))
    }

    async fn handle_update_config(&self, command: UpdateGameMonitoringConfig) -> AppResult<GameMonitoringCommandResult> {
        let mut config = self.config.write().await;
        *config = command.config;
        Ok(GameMonitoringCommandResult::ConfigUpdated)
    }
}
