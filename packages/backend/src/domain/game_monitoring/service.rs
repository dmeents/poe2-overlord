use crate::domain::character::traits::CharacterService;
use crate::domain::game_monitoring::{
    models::GameProcessStatus,
    traits::{GameMonitoringService, ProcessDetector},
};
use crate::errors::AppResult;
use crate::infrastructure::events::{AppEvent, EventBus};
use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Clone)]
pub struct GameMonitoringServiceImpl {
    event_bus: Arc<EventBus>,
    process_detector: Arc<dyn ProcessDetector>,
    /// Includes time tracking through zone finalization
    character_service: Arc<dyn CharacterService>,
    is_monitoring: Arc<RwLock<bool>>,
    current_status: Arc<RwLock<Option<GameProcessStatus>>>,
    monitoring_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl GameMonitoringServiceImpl {
    pub fn new(
        event_bus: Arc<EventBus>,
        process_detector: Arc<dyn ProcessDetector>,
        character_service: Arc<dyn CharacterService>,
    ) -> Self {
        Self {
            event_bus,
            process_detector,
            character_service,
            is_monitoring: Arc::new(RwLock::new(false)),
            current_status: Arc::new(RwLock::new(None)),
            monitoring_task: Arc::new(RwLock::new(None)),
        }
    }

    /// Coordinates time tracking integration and event publishing on state changes
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
            } else {
                info!("POE2 process stopped");

                if let Err(e) = self.character_service.finalize_all_active_zones().await {
                    error!(
                        "Failed to finalize character tracking when game stopped: {}",
                        e
                    );
                } else {
                    info!("Character tracking finalized after game process stopped");
                }
            }
        }

        let event = AppEvent::game_process_status_changed(
            previous_status,
            current_status.clone(),
            is_state_change,
        );

        if let Err(e) = self.event_bus.publish(event).await {
            error!("Failed to publish game process status change event: {}", e);
        }

        {
            let mut status = self.current_status.write().await;
            *status = Some(current_status);
        }

        Ok(())
    }

    /// Uses adaptive polling: fast when no game detected, slow when game is running
    async fn start_monitoring_loop(&self) -> AppResult<()> {
        let process_detector = self.process_detector.clone();
        let is_monitoring = self.is_monitoring.clone();
        let config = process_detector.get_config();

        let mut current_interval = Duration::from_secs(config.detection_interval_seconds);
        let mut interval_timer = interval(current_interval);
        let mut previous_status: Option<GameProcessStatus> = None;

        info!(
            "Game monitoring loop started with adaptive polling (detection: {}s, monitoring: {}s)",
            config.detection_interval_seconds, config.monitoring_interval_seconds
        );

        match process_detector.check_game_process().await {
            Ok(initial_status) => {
                if let Err(e) = self
                    .handle_process_state_change(initial_status.clone(), None)
                    .await
                {
                    error!("Failed to handle initial process status: {}", e);
                }

                let initial_interval = if initial_status.running {
                    Duration::from_secs(config.monitoring_interval_seconds)
                } else {
                    Duration::from_secs(config.detection_interval_seconds)
                };

                if initial_interval != current_interval {
                    current_interval = initial_interval;
                    interval_timer = interval(current_interval);
                }

                previous_status = Some(initial_status);
            }
            Err(e) => {
                error!("Failed to get initial game process status: {}", e);
            }
        }

        loop {
            interval_timer.tick().await;

            if !*is_monitoring.read().await {
                break;
            }

            match process_detector.check_game_process().await {
                Ok(current_status_value) => {
                    let is_state_change = previous_status
                        .as_ref()
                        .map(|prev| current_status_value.is_state_change(prev))
                        .unwrap_or(true);

                    if is_state_change {
                        info!(
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

                        let new_interval = if current_status_value.running {
                            Duration::from_secs(config.monitoring_interval_seconds)
                        } else {
                            Duration::from_secs(config.detection_interval_seconds)
                        };

                        if new_interval != current_interval {
                            current_interval = new_interval;
                            interval_timer = interval(current_interval);
                        }
                    } else {
                        let mut status = self.current_status.write().await;
                        *status = Some(current_status_value.clone());
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
            return Ok(());
        }

        *is_monitoring = true;
        info!("Starting game process monitoring");

        let service_clone = Arc::new(self.clone());
        let task_handle = tokio::spawn(async move {
            match service_clone.start_monitoring_loop().await {
                Ok(_) => {
                    info!("Game monitoring loop completed successfully");
                }
                Err(e) => {
                    error!("Background monitoring loop failed: {}", e);
                }
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
