use crate::errors::AppResult;
use crate::models::events::LogEvent;
use crate::parsers::core::{LogParserManager, ParserResult};
use crate::services::{
    event_dispatcher::EventDispatcher, location_tracker::LocationTracker,
    log_file_watcher::LogFileWatcher, server_monitor::ServerMonitor,
};
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

/// Log analyzer that watches POE client log files for scene changes and server connections
pub struct LogAnalyzer {
    file_monitor: LogFileWatcher,
    event_broadcaster: EventDispatcher,
    state_manager: LocationTracker,
    server_manager: Arc<ServerMonitor>,
    parser_manager: LogParserManager,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogAnalyzer {
    /// Create a new log analyzer
    pub fn new(log_path: String, server_manager: Arc<ServerMonitor>) -> Self {
        let state_manager = LocationTracker::new();

        Self {
            file_monitor: LogFileWatcher::new(log_path),
            event_broadcaster: EventDispatcher::new(),
            state_manager: state_manager.clone(),
            server_manager,
            parser_manager: LogParserManager::new(),
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    /// Get the event receiver for subscribing to all log events
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<LogEvent> {
        self.event_broadcaster.subscribe()
    }

    /// Get the server monitor
    pub fn get_server_manager(&self) -> Arc<ServerMonitor> {
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

        info!("Starting log file monitoring for scene changes and server connections");

        // Initialize server status background tasks (load status and start ping monitoring)
        // This is safe to do here since the Tokio runtime is now available
        let server_manager = Arc::clone(&self.server_manager);
        tokio::spawn(async move {
            // Load existing server status from file
            if let Err(e) = server_manager.load_status().await {
                warn!("Failed to load server status: {}", e);
            }

            // Start periodic ping monitoring in the background
            server_manager.start_periodic_ping().await;
        });

        // Start the main monitoring loop
        self.start_monitoring_loop().await;

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

    // Private helper methods

    /// Start the main monitoring loop
    async fn start_monitoring_loop(&self) {
        let file_monitor = self.file_monitor.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        let state_manager = self.state_manager.clone();
        let server_manager = Arc::clone(&self.server_manager);
        let parser_manager = self.parser_manager.clone();
        let is_running = Arc::clone(&self.is_running);

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
    }
}

impl LogAnalyzer {
    /// Monitor the log file for changes and parse new lines
    async fn monitor_log_file(
        file_monitor: LogFileWatcher,
        event_broadcaster: EventDispatcher,
        state_manager: LocationTracker,
        server_manager: Arc<ServerMonitor>,
        parser_manager: LogParserManager,
        is_running: &Arc<tokio::sync::RwLock<bool>>,
    ) -> AppResult<()> {
        let mut last_position = file_monitor.get_log_file_size()?;
        let mut check_interval = time::interval(Duration::from_millis(100));

        loop {
            check_interval.tick().await;

            // Check if we should stop monitoring
            if !*is_running.read().await {
                debug!("Log monitoring stopped, exiting monitor loop");
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
        file_monitor: &LogFileWatcher,
        last_position: &mut u64,
        parser_manager: &LogParserManager,
        event_broadcaster: &EventDispatcher,
        state_manager: &LocationTracker,
        server_manager: &Arc<ServerMonitor>,
    ) -> AppResult<()> {
        file_monitor
            .process_new_lines(last_position, |line| {
                // Process the line asynchronously without spawning a new task for each line
                let parser_manager = parser_manager.clone();
                let event_broadcaster = event_broadcaster.clone();
                let server_manager = Arc::clone(server_manager);
                let state_manager = state_manager.clone();
                let line = line.to_string();

                // Use spawn_blocking for CPU-bound parsing work
                tokio::task::spawn_blocking(move || {
                    // Process the line in a blocking context
                    tokio::runtime::Handle::current().block_on(async {
                        Self::process_single_line(
                            parser_manager,
                            event_broadcaster,
                            state_manager,
                            server_manager,
                            line,
                        )
                        .await;
                    });
                });
            })
            .await?;

        Ok(())
    }

    /// Process a single log line for both scene changes and server connections
    async fn process_single_line(
        parser_manager: LogParserManager,
        event_broadcaster: EventDispatcher,
        state_manager: LocationTracker,
        server_manager: Arc<ServerMonitor>,
        line: String,
    ) {
        debug!("Processing log line: {}", line.trim());

        // Parse the line using the unified parser
        if let Ok(Some(result)) = parser_manager.parse_line(&line) {
            match result {
                ParserResult::SceneChange(event) => {
                    debug!("Scene change event parsed successfully: {:?}", event);

                    // Validate that this is an actual scene change using the location tracker
                    if let Some(validated_event) =
                        state_manager.validate_scene_change_event(event).await
                    {
                        debug!(
                            "Scene change validated as actual change: {:?}",
                            validated_event
                        );
                        // Broadcast as unified log event
                        if let Err(e) = event_broadcaster
                            .broadcast_event(LogEvent::SceneChange(validated_event))
                        {
                            warn!("Failed to broadcast scene change event: {}", e);
                        } else {
                            debug!("Scene change event broadcast successfully");
                        }
                    } else {
                        debug!("Scene change event was not an actual change, skipping broadcast");
                    }
                }
                ParserResult::ServerConnection(event) => {
                    debug!("Server connection event detected: {:?}", event);

                    // Update server status manager
                    if let Err(e) = server_manager.update_server_info(&event).await {
                        warn!("Failed to update server info: {}", e);
                    } else {
                        debug!("Successfully updated server status manager");
                    }

                    // Broadcast as unified log event
                    if let Err(e) =
                        event_broadcaster.broadcast_event(LogEvent::ServerConnection(event))
                    {
                        warn!("Failed to broadcast server connection event: {}", e);
                    } else {
                        debug!("Successfully broadcasted server connection event");
                    }
                }
            }
        } else {
            debug!("No events parsed from line");
        }
    }
}

// Implement Clone for the components we need to move into async tasks
impl Clone for LogFileWatcher {
    fn clone(&self) -> Self {
        Self {
            log_path: self.log_path.clone(),
        }
    }
}

impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        Self {
            unified_event_sender: self.unified_event_sender.clone(),
            ping_event_sender: self.ping_event_sender.clone(),
        }
    }
}
