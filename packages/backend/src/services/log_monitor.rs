use crate::errors::AppResult;
use crate::models::events::SceneChangeEvent;
use crate::parsers::manager::LogParserManager;
use crate::services::{
    event_broadcaster::EventBroadcaster, file_monitor::FileMonitor,
    player_location_manager::PlayerLocationManager, server_status::ServerStatusManager,
};
use log::{error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

/// Log monitor service that watches POE client log files for scene changes and server connections
pub struct LogMonitorService {
    file_monitor: FileMonitor,
    event_broadcaster: EventBroadcaster,
    state_manager: PlayerLocationManager,
    server_manager: Arc<ServerStatusManager>,
    parser_manager: LogParserManager,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogMonitorService {
    /// Create a new log monitor service
    pub fn new(log_path: String) -> Self {
        let state_manager = PlayerLocationManager::new();
        let server_manager = Arc::new(ServerStatusManager::new());

        Self {
            file_monitor: FileMonitor::new(log_path),
            event_broadcaster: EventBroadcaster::new(),
            state_manager: state_manager.clone(),
            server_manager,
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

    /// Get the server status manager
    pub fn get_server_manager(&self) -> Arc<ServerStatusManager> {
        Arc::clone(&self.server_manager)
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
        let server_manager = Arc::clone(&self.server_manager);
        let parser_manager = self.parser_manager.clone();
        let is_running = Arc::clone(&self.is_running);

        info!("Starting log file monitoring for scene changes and server connections");

        // Start server status monitoring
        if let Err(e) = server_manager.start_monitoring().await {
            warn!("Failed to start server monitoring: {}", e);
        }

        tokio::spawn(async move {
            if let Err(e) = Self::monitor_log_file(
                file_monitor,
                event_broadcaster,
                state_manager,
                server_manager,
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
}

impl LogMonitorService {
    /// Monitor the log file for changes and parse new lines
    async fn monitor_log_file(
        file_monitor: FileMonitor,
        event_broadcaster: EventBroadcaster,
        state_manager: PlayerLocationManager,
        server_manager: Arc<ServerStatusManager>,
        parser_manager: LogParserManager,
        is_running: &Arc<tokio::sync::RwLock<bool>>,
    ) -> AppResult<()> {
        let mut last_position = file_monitor.get_log_file_size()?;
        let mut check_interval = time::interval(Duration::from_millis(100));

        loop {
            check_interval.tick().await;

            // Check if we should stop monitoring
            if !*is_running.read().await {
                info!("Log monitoring stopped, exiting monitor loop");
                break;
            }

            // Check for new content in the log file
            let current_size = file_monitor.get_log_file_size()?;
            if current_size > last_position {
                Self::process_new_lines(
                    &file_monitor,
                    &mut last_position,
                    &parser_manager,
                    &event_broadcaster,
                    &state_manager,
                    &server_manager,
                )
                .await?;
            } else if current_size < last_position {
                // File was truncated, reset position
                warn!("Log file was truncated, resetting position");
                last_position = current_size;
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
        server_manager: &Arc<ServerStatusManager>,
    ) -> AppResult<()> {
        file_monitor
            .process_new_lines(last_position, |line| {
                // Try to parse the line for scene changes
                let parser_manager = parser_manager.clone();
                let event_broadcaster = event_broadcaster.clone();
                let server_manager = Arc::clone(server_manager);
                let line = line.to_string(); // Clone the line to avoid lifetime issues

                tokio::spawn(async move {
                    // Parse for scene changes
                    if let Some(event) = parser_manager.parse_line(&line).await {
                        // The parser manager now only returns events for actual changes
                        // so we can directly broadcast the event
                        if let Err(e) = event_broadcaster.broadcast_event(event) {
                            warn!("Failed to broadcast scene change event: {}", e);
                        }
                    }

                    // Parse for server connections
                    if let Some(event) = parser_manager.parse_server_connection(&line) {
                        // Update server status manager
                        if let Err(e) = server_manager.update_server_info(&event).await {
                            warn!("Failed to update server info: {}", e);
                        }

                        // Broadcast server connection event to frontend
                        if let Err(e) = event_broadcaster.broadcast_server_event(event) {
                            warn!("Failed to broadcast server connection event: {}", e);
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
            server_event_sender: self.server_event_sender.clone(),
        }
    }
}
