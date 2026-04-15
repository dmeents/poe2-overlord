use crate::domain::character::traits::CharacterService;
use crate::domain::game_monitoring::{
    models::GameProcessStatus,
    traits::{GameMonitoringService, ProcessDetector},
};
use crate::domain::leveling::traits::LevelingService;
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
    leveling_service: Arc<dyn LevelingService>,
    is_monitoring: Arc<RwLock<bool>>,
    current_status: Arc<RwLock<Option<GameProcessStatus>>>,
    monitoring_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl GameMonitoringServiceImpl {
    pub fn new(
        event_bus: Arc<EventBus>,
        process_detector: Arc<dyn ProcessDetector>,
        character_service: Arc<dyn CharacterService>,
        leveling_service: Arc<dyn LevelingService>,
    ) -> Self {
        Self {
            event_bus,
            process_detector,
            character_service,
            leveling_service,
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
            .is_none_or(|prev| current_status.is_state_change(prev));

        if is_state_change {
            if current_status.is_running() {
                info!(
                    "POE2 process started - PID: {}, Name: {}",
                    current_status.pid, current_status.name
                );
            } else {
                info!("POE2 process stopped");

                // Capture active character IDs before finalization clears the active zones
                let active_character_ids =
                    match self.leveling_service.get_active_zone_character_ids().await {
                        Ok(ids) => ids,
                        Err(e) => {
                            error!(
                                "Failed to get active zone character IDs on game stop: {e}"
                            );
                            vec![]
                        }
                    };

                if let Err(e) = self.leveling_service.finalize_active_zone_times().await {
                    error!(
                        "Failed to finalize active zone times when game stopped: {e}"
                    );
                }

                if let Err(e) = self.character_service.finalize_all_active_zones().await {
                    let error_msg = format!(
                        "Failed to finalize character tracking when game stopped: {e}"
                    );
                    error!("{error_msg}");

                    // Publish error event so frontend can notify user
                    let error_event =
                        AppEvent::system_error(error_msg, "CharacterFinalizationError".to_string());
                    if let Err(publish_err) = self.event_bus.publish(error_event).await {
                        error!(
                            "Failed to publish finalization error event: {publish_err}"
                        );
                    }
                } else {
                    info!("Character tracking finalized after game process stopped");
                }

                // Emit updated leveling stats so frontend sees is_actively_grinding: false
                for character_id in &active_character_ids {
                    if let Err(e) = self.leveling_service.emit_stats_update(character_id).await {
                        error!(
                            "Failed to emit leveling stats update for {character_id} after game stop: {e}"
                        );
                    }
                }
            }
        }

        let event = AppEvent::game_process_status_changed(
            previous_status,
            current_status.clone(),
            is_state_change,
        );

        if let Err(e) = self.event_bus.publish(event).await {
            error!("Failed to publish game process status change event: {e}");
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
                    error!("Failed to handle initial process status: {e}");
                }

                let initial_interval = if initial_status.running {
                    Duration::from_secs(config.monitoring_interval_seconds)
                } else {
                    Duration::from_secs(config.detection_interval_seconds)
                };

                if initial_interval != current_interval {
                    current_interval = initial_interval;
                    let mut new_timer = interval(current_interval);
                    new_timer.tick().await; // Consume immediate tick
                    interval_timer = new_timer;
                }

                previous_status = Some(initial_status);
            }
            Err(e) => {
                error!("Failed to get initial game process status: {e}");
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
                        .is_none_or(|prev| current_status_value.is_state_change(prev));

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
                            error!("Failed to handle process status change: {e}");
                        }

                        let new_interval = if current_status_value.running {
                            Duration::from_secs(config.monitoring_interval_seconds)
                        } else {
                            Duration::from_secs(config.detection_interval_seconds)
                        };

                        if new_interval != current_interval {
                            current_interval = new_interval;
                            let mut new_timer = interval(current_interval);
                            new_timer.tick().await; // Consume immediate tick
                            interval_timer = new_timer;
                        }
                    } else {
                        let mut status = self.current_status.write().await;
                        *status = Some(current_status_value.clone());
                    }

                    previous_status = Some(current_status_value);
                }
                Err(e) => {
                    error!("Error checking game process: {e}");
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
                Ok(()) => {
                    info!("Game monitoring loop completed successfully");
                }
                Err(e) => {
                    error!("Background monitoring loop failed: {e}");
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
                error!("Error waiting for monitoring task to complete: {e}");
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

    async fn check_status_now(&self) -> AppResult<GameProcessStatus> {
        self.process_detector.check_game_process().await
    }
}
