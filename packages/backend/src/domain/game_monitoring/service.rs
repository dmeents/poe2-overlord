use crate::domain::game_monitoring::{
    events::{GameMonitoringEvent, GameProcessStatusUpdated},
    models::GameProcessStatus,
    traits::{GameMonitoringEventPublisher, GameMonitoringService, ProcessDetector},
};
use crate::domain::time_tracking::traits::TimeTrackingService;
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Clone)]
pub struct GameMonitoringServiceImpl {
    time_tracker: Arc<dyn TimeTrackingService>,
    event_publisher: Arc<dyn GameMonitoringEventPublisher>,
    process_detector: Arc<dyn ProcessDetector>,
    is_monitoring: Arc<RwLock<bool>>,
    current_status: Arc<RwLock<Option<GameProcessStatus>>>,
    monitoring_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl GameMonitoringServiceImpl {
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
            monitoring_task: Arc::new(RwLock::new(None)),
        }
    }

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
                info!(
                    "POE2 process started - PID: {}, Name: {}",
                    current_status.pid, current_status.name
                );

                let start_time = chrono::DateTime::from(current_status.detected_at);
                self.time_tracker
                    .set_poe_process_start_time(start_time)
                    .await;
            } else {
                info!("POE2 process stopped");
                debug!("POE2 process stopped, ending all active time tracking sessions");

                if let Err(e) = self.time_tracker.end_all_active_sessions_global().await {
                    error!("Failed to end active time tracking sessions: {}", e);
                }
                self.time_tracker.clear_poe_process_start_time().await;
            }
        }

        let status_event = GameMonitoringEvent::StatusUpdated(GameProcessStatusUpdated::new(
            current_status.clone(),
            is_state_change,
        ));
        self.event_publisher.publish_event(status_event).await?;

        {
            let mut status = self.current_status.write().await;
            *status = Some(current_status);
        }

        Ok(())
    }

    async fn start_monitoring_loop(&self) -> AppResult<()> {
        let process_detector = self.process_detector.clone();
        let is_monitoring = self.is_monitoring.clone();

        let check_interval =
            Duration::from_secs(process_detector.get_config().check_interval_seconds);
        let mut interval_timer = interval(check_interval);
        let mut previous_status: Option<GameProcessStatus> = None;

        info!(
            "Game monitoring loop started with interval: {:?}",
            check_interval
        );

        loop {
            interval_timer.tick().await;

            if !*is_monitoring.read().await {
                debug!("Game monitoring stopped, exiting monitoring loop");
                break;
            }

            match process_detector.check_game_process().await {
                Ok(current_status_value) => {
                    let is_state_change = previous_status
                        .as_ref()
                        .map(|prev| current_status_value.is_state_change(prev))
                        .unwrap_or(true);

                    if is_state_change {
                        debug!(
                            "Game process state change detected: running={}, pid={}, name={}",
                            current_status_value.running,
                            current_status_value.pid,
                            current_status_value.name
                        );

                        if let Err(e) = self
                            .handle_process_state_change(
                                current_status_value.clone(),
                                previous_status.clone(),
                            )
                            .await
                        {
                            error!("Failed to handle process status change: {}", e);
                        }
                    }

                    previous_status = Some(current_status_value);
                }
                Err(e) => {
                    error!("Error checking game process: {}", e);
                }
            }
        }

        info!("Game monitoring loop stopped");
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
        self.handle_process_state_change(current_status, previous_status)
            .await
    }

    async fn start_monitoring(&self) -> AppResult<()> {
        let mut is_monitoring = self.is_monitoring.write().await;
        if *is_monitoring {
            debug!("Game monitoring is already running");
            return Ok(());
        }

        *is_monitoring = true;
        info!("Starting game process monitoring");

        let service_clone = Arc::new(self.clone());
        let task_handle = tokio::spawn(async move {
            if let Err(e) = service_clone.start_monitoring_loop().await {
                error!("Background monitoring loop failed: {}", e);
            }
        });

        {
            let mut task = self.monitoring_task.write().await;
            *task = Some(task_handle);
        }

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

        if let Some(task_handle) = self.monitoring_task.write().await.take() {
            if let Err(e) = task_handle.await {
                error!("Error waiting for monitoring task to complete: {}", e);
            }
        }

        Ok(())
    }

    async fn is_monitoring(&self) -> bool {
        *self.is_monitoring.read().await
    }

    async fn get_current_status(&self) -> Option<GameProcessStatus> {
        self.current_status.read().await.clone()
    }
}

