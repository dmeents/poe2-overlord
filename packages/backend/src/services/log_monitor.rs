use crate::errors::{AppError, AppResult};
use crate::models::events::SceneChangeEvent;
use crate::parsers::LogParserManager;
use crate::services::{
    event_broadcaster::EventBroadcaster, file_monitor::FileMonitor,
    player_location_manager::PlayerLocationManager,
};
use log::{error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

/// Log monitor service that watches POE client log files for scene changes
pub struct LogMonitorService {
    file_monitor: FileMonitor,
    event_broadcaster: EventBroadcaster,
    state_manager: PlayerLocationManager,
    parser_manager: LogParserManager,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogMonitorService {
    /// Create a new log monitor service
    pub fn new(log_path: String) -> Self {
        let state_manager = PlayerLocationManager::new();

        Self {
            file_monitor: FileMonitor::new(log_path),
            event_broadcaster: EventBroadcaster::new(),
            state_manager: state_manager.clone(),
            parser_manager: LogParserManager::new(Arc::new(state_manager)),
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    /// Get the event receiver for subscribing to scene change events
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<SceneChangeEvent> {
        self.event_broadcaster.subscribe()
    }

    /// Get the event receiver for subscribing to zone change events
    pub fn subscribe_zones(
        &self,
    ) -> tokio::sync::broadcast::Receiver<crate::models::events::ZoneChangeEvent> {
        self.event_broadcaster.subscribe_zones()
    }

    /// Get the event receiver for subscribing to act change events
    pub fn subscribe_acts(
        &self,
    ) -> tokio::sync::broadcast::Receiver<crate::models::events::ActChangeEvent> {
        self.event_broadcaster.subscribe_acts()
    }

    /// Start monitoring the log file
    pub async fn start_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Log monitoring is already running");
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        let file_monitor = self.file_monitor.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        let state_manager = self.state_manager.clone();
        let parser_manager = self.parser_manager.clone();
        let is_running = Arc::clone(&self.is_running);

        info!("Starting log file monitoring for scene changes");

        tokio::spawn(async move {
            if let Err(e) = Self::monitor_log_file(
                file_monitor,
                event_broadcaster,
                state_manager,
                parser_manager,
                &is_running,
            )
            .await
            {
                error!("Log monitoring failed: {}", e);
            }
        });

        Ok(())
    }

    /// Stop monitoring the log file
    pub async fn stop_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            warn!("Log monitoring is not running");
            return Ok(());
        }

        *is_running = false;
        info!("Log monitoring stopped");
        Ok(())
    }

    /// Check if monitoring is currently active
    pub async fn is_monitoring(&self) -> bool {
        *self.is_running.read().await
    }

    /// Reset the previous scene and act tracking
    pub async fn reset_tracking(&self) {
        self.state_manager.reset_tracking().await;
    }

    /// Get the current scene and act being tracked
    pub async fn get_current_scene_and_act(&self) -> (Option<String>, Option<String>) {
        self.state_manager.get_current_scene_and_act().await
    }

    /// Get current log file size
    pub fn get_log_file_size(&self) -> AppResult<u64> {
        self.file_monitor.get_log_file_size()
    }

    /// Read the last N lines from the log file
    pub fn read_last_lines(&self, count: usize) -> AppResult<Vec<String>> {
        self.file_monitor.read_last_lines(count)
    }

    /// Main monitoring loop that watches for file changes and processes new content
    async fn monitor_log_file(
        file_monitor: FileMonitor,
        event_broadcaster: EventBroadcaster,
        state_manager: PlayerLocationManager,
        parser_manager: LogParserManager,
        is_running: &Arc<tokio::sync::RwLock<bool>>,
    ) -> AppResult<()> {
        if !file_monitor.file_exists() {
            return Err(AppError::LogMonitor(format!(
                "Log file not found: {}",
                file_monitor.get_log_path()
            )));
        }

        // Get initial file size and position
        let mut last_position = file_monitor.get_log_file_size()?;

        // Create file system event watcher
        let _watcher = file_monitor.create_watcher(|_event| {
            // Note: We can't directly call async functions from this callback
            // The actual processing will happen in the main loop
        })?;

        // Keep the watcher alive and process events
        loop {
            tokio::select! {
                _ = time::sleep(Duration::from_millis(100)) => {
                    // Check if still running
                    if !*is_running.read().await {
                        break;
                    }

                    // Check if file has new content
                    if let Ok(current_size) = file_monitor.get_log_file_size() {
                        if current_size > last_position {
                            // File has new content, process it
                            if let Err(e) = Self::process_new_lines(
                                &file_monitor,
                                &mut last_position,
                                &parser_manager,
                                &event_broadcaster,
                                &state_manager,
                            ).await {
                                warn!("Failed to process new lines: {}", e);
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process new lines from the log file starting from the last known position
    async fn process_new_lines(
        file_monitor: &FileMonitor,
        last_position: &mut u64,
        parser_manager: &LogParserManager,
        event_broadcaster: &EventBroadcaster,
        _state_manager: &PlayerLocationManager,
    ) -> AppResult<()> {
        file_monitor
            .process_new_lines(last_position, |line| {
                // Try to parse the line for scene changes
                // Note: parse_line is now async, so we need to handle this differently
                // For now, we'll spawn a task to handle the async parsing
                let parser_manager = parser_manager.clone();
                let event_broadcaster = event_broadcaster.clone();
                let line = line.to_string(); // Clone the line to avoid lifetime issues

                tokio::spawn(async move {
                    if let Some(event) = parser_manager.parse_line(&line).await {
                        // The parser manager now only returns events for actual changes
                        // so we can directly broadcast the event
                        if let Err(e) = event_broadcaster.broadcast_event(event) {
                            warn!("Failed to broadcast scene change event: {}", e);
                        }
                    }
                });
            })
            .await?;

        Ok(())
    }
}

// Implement Clone for the components we need to move into async tasks
impl Clone for FileMonitor {
    fn clone(&self) -> Self {
        Self {
            log_path: self.log_path.clone(),
        }
    }
}

impl Clone for EventBroadcaster {
    fn clone(&self) -> Self {
        Self {
            scene_event_sender: self.scene_event_sender.clone(),
        }
    }
}
