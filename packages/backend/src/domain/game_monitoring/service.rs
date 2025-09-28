use crate::domain::character::traits::CharacterService;
use crate::domain::events::{AppEvent, EventBus};
use crate::domain::game_monitoring::{
    models::GameProcessStatus,
    traits::{GameMonitoringService, ProcessDetector},
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;

/// Implementation of the game monitoring service.
///
/// This service orchestrates the monitoring of Path of Exile 2 game processes,
/// integrating with time tracking services and publishing events about process
/// status changes. It runs a background monitoring loop that periodically
/// checks for game processes and handles state transitions.
#[derive(Clone)]
pub struct GameMonitoringServiceImpl {
    /// Event bus for publishing game monitoring events
    event_bus: Arc<EventBus>,
    /// Detector for finding and checking game processes
    process_detector: Arc<dyn ProcessDetector>,
    /// Character service for finalizing zones when game ends (includes tracking)
    character_service: Arc<dyn CharacterService>,
    /// Flag indicating whether monitoring is currently active
    is_monitoring: Arc<RwLock<bool>>,
    /// Current status of the game process (if detected)
    current_status: Arc<RwLock<Option<GameProcessStatus>>>,
    /// Handle to the background monitoring task
    monitoring_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl GameMonitoringServiceImpl {
    /// Creates a new game monitoring service instance.
    ///
    /// Initializes the service with the required dependencies for event publishing
    /// and process detection. All internal state is initialized to default values
    /// (not monitoring, no current status, no active task).
    ///
    /// # Arguments
    /// * `event_bus` - Event bus for publishing game monitoring events
    /// * `process_detector` - Detector for finding and checking game processes
    /// * `character_service` - Character service for finalizing zones (includes tracking)
    ///
    /// # Returns
    /// * `Self` - New GameMonitoringServiceImpl instance
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

    /// Handles a change in process state, coordinating with time tracking and event publishing.
    ///
    /// This method is the core logic for processing game process state changes. It:
    /// 1. Determines if this represents an actual state change
    /// 2. Integrates with time tracking services (start/stop sessions)
    /// 3. Publishes events to notify other parts of the system
    /// 4. Updates the internal current status
    ///
    /// # Arguments
    /// * `current_status` - The current process status
    /// * `previous_status` - The previous process status (None for first detection)
    ///
    /// # Returns
    /// * `AppResult<()>` - Success or error result
    async fn handle_process_state_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()> {
        // Determine if this represents an actual state change (running <-> stopped)
        let is_state_change = previous_status
            .as_ref()
            .map(|prev| current_status.is_state_change(prev))
            .unwrap_or(true);

        // Handle state changes by integrating with time tracking services
        if is_state_change {
            if current_status.is_running() {
                info!(
                    "POE2 process started - PID: {}, Name: {}",
                    current_status.pid, current_status.name
                );

                // Game process started - time tracking will be handled by zone changes
                // when the character enters/leaves zones during gameplay
            } else {
                info!("POE2 process stopped");

                // Finalize character tracking when game process stops
                if let Err(e) = self
                    .character_service
                    .finalize_all_active_zones()
                    .await
                {
                    error!(
                        "Failed to finalize character tracking when game stopped: {}",
                        e
                    );
                } else {
                    info!("Character tracking finalized after game process stopped");
                }
            }
        }

        // Publish game process status change event
        let event = AppEvent::game_process_status_changed(
            previous_status,
            current_status.clone(),
            is_state_change,
        );

        if let Err(e) = self.event_bus.publish(event).await {
            error!("Failed to publish game process status change event: {}", e);
        }


        // Update the internal current status
        {
            let mut status = self.current_status.write().await;
            *status = Some(current_status);
        }

        Ok(())
    }

    /// Runs the main monitoring loop that periodically checks for game processes.
    ///
    /// This method implements the core monitoring logic that runs in a background task.
    /// It uses adaptive polling: fast detection when no game is running, slow monitoring
    /// when game is running. The loop continues until the monitoring is stopped.
    ///
    /// # Returns
    /// * `AppResult<()>` - Success or error result
    async fn start_monitoring_loop(&self) -> AppResult<()> {
        let process_detector = self.process_detector.clone();
        let is_monitoring = self.is_monitoring.clone();
        let config = process_detector.get_config();

        // Start with fast detection interval (when no game is running)
        let mut current_interval = Duration::from_secs(config.detection_interval_seconds);
        let mut interval_timer = interval(current_interval);
        let mut previous_status: Option<GameProcessStatus> = None;

        info!(
            "Game monitoring loop started with adaptive polling (detection: {}s, monitoring: {}s)",
            config.detection_interval_seconds, config.monitoring_interval_seconds
        );

        // Publish initial status immediately to handle timing issues
        match process_detector.check_game_process().await {
            Ok(initial_status) => {

                // Publish initial status as a state change
                if let Err(e) = self
                    .handle_process_state_change(initial_status.clone(), None)
                    .await
                {
                    error!("Failed to handle initial process status: {}", e);
                }

                // Set initial interval based on game state
                let initial_interval = if initial_status.running {
                    Duration::from_secs(config.monitoring_interval_seconds)
                } else {
                    Duration::from_secs(config.detection_interval_seconds)
                };

                // Update interval if it's different from the default
                if initial_interval != current_interval {
                    current_interval = initial_interval;
                    interval_timer = interval(current_interval);
                }

                // Set as previous status for the loop
                previous_status = Some(initial_status);
            }
            Err(e) => {
                error!("Failed to get initial game process status: {}", e);
            }
        }

        // Main monitoring loop
        loop {
            interval_timer.tick().await;

            // Check if monitoring should continue
            if !*is_monitoring.read().await {
                break;
            }

            // Check the current game process status
            match process_detector.check_game_process().await {
                Ok(current_status_value) => {
                    // Determine if this represents a state change
                    let is_state_change = previous_status
                        .as_ref()
                        .map(|prev| current_status_value.is_state_change(prev))
                        .unwrap_or(true);

                    // Only log and process state changes, not every check
                    if is_state_change {
                        info!(
                            "Game process state change detected: running={}, pid={}, name={}",
                            current_status_value.running,
                            current_status_value.pid,
                            current_status_value.name
                        );

                        // Process the state change (time tracking, events, etc.)
                        if let Err(e) = self
                            .handle_process_state_change(
                                current_status_value.clone(),
                                previous_status.clone(),
                            )
                            .await
                        {
                            error!("Failed to handle process status change: {}", e);
                        }

                        // Switch polling interval based on game state
                        let new_interval = if current_status_value.running {
                            Duration::from_secs(config.monitoring_interval_seconds)
                        } else {
                            Duration::from_secs(config.detection_interval_seconds)
                        };

                        // Update interval if it changed
                        if new_interval != current_interval {
                            current_interval = new_interval;
                            interval_timer = interval(current_interval);
                        }
                    } else {
                        // Silently update the internal status for get_current_status()
                        // No logging for unchanged states to reduce console spam
                        {
                            let mut status = self.current_status.write().await;
                            *status = Some(current_status_value.clone());
                        }
                    }

                    // Update previous status for next iteration
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
    /// Handles a change in process status by delegating to the internal implementation.
    ///
    /// This is the public interface for handling process status changes, which delegates
    /// to the internal `handle_process_state_change` method that contains the core logic.
    async fn handle_process_status_change(
        &self,
        current_status: GameProcessStatus,
        previous_status: Option<GameProcessStatus>,
    ) -> AppResult<()> {
        self.handle_process_state_change(current_status, previous_status)
            .await
    }

    /// Starts the background monitoring loop.
    ///
    /// This method spawns a new background task that runs the monitoring loop.
    /// If monitoring is already running, this is a no-op. The monitoring task
    /// will continue running until explicitly stopped.
    async fn start_monitoring(&self) -> AppResult<()> {
        let mut is_monitoring = self.is_monitoring.write().await;
        if *is_monitoring {
            return Ok(());
        }

        *is_monitoring = true;
        info!("Starting game process monitoring");

        // Spawn the monitoring loop in a background task
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

        // Store the task handle for later cleanup
        {
            let mut task = self.monitoring_task.write().await;
            *task = Some(task_handle);
        }

        Ok(())
    }

    /// Stops the background monitoring loop.
    ///
    /// This method gracefully shuts down the monitoring by setting the stop flag
    /// and waiting for the background task to complete. If monitoring is not
    /// running, this is a no-op.
    async fn stop_monitoring(&self) -> AppResult<()> {
        let mut is_monitoring = self.is_monitoring.write().await;
        if !*is_monitoring {
            return Ok(());
        }

        *is_monitoring = false;
        info!("Stopping game process monitoring");

        // Wait for the monitoring task to complete
        if let Some(task_handle) = self.monitoring_task.write().await.take() {
            if let Err(e) = task_handle.await {
                error!("Error waiting for monitoring task to complete: {}", e);
            }
        }

        Ok(())
    }

    /// Checks if the monitoring service is currently active.
    ///
    /// Returns the current monitoring state without blocking.
    async fn is_monitoring(&self) -> bool {
        *self.is_monitoring.read().await
    }

    /// Gets the current process status if available.
    ///
    /// Returns a clone of the current process status, or None if no process
    /// has been detected yet.
    async fn get_current_status(&self) -> Option<GameProcessStatus> {
        self.current_status.read().await.clone()
    }
}
