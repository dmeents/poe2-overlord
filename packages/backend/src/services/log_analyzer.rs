use crate::errors::{AppError, AppResult};
use crate::models::events::LogEvent;
use crate::parsers::config::ParsersConfig;
use crate::parsers::core::{LogParserManager, ParserResult};
use crate::services::{
    event_dispatcher::EventDispatcher,
    location_tracker::{LocationTracker, SceneTypeConfig},
    server_monitor::ServerMonitor,
};
use log::{debug, error, info, warn};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Seek};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

/// Log analyzer that watches POE client log files for scene changes, server connections, and character level-ups
pub struct LogAnalyzer {
    log_path: String,
    event_broadcaster: EventDispatcher,
    state_manager: LocationTracker,
    server_manager: Arc<ServerMonitor>,
    character_manager: Arc<crate::services::character_manager::CharacterManager>,
    parser_manager: LogParserManager,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogAnalyzer {
    /// Create a new log analyzer
    pub fn new(
        log_path: String, 
        server_manager: Arc<ServerMonitor>,
        character_manager: Arc<crate::services::character_manager::CharacterManager>,
    ) -> Self {
        let parser_manager = LogParserManager::new();

        // Get the scene type configuration from the parser (single source of truth)
        let parser_config = ParsersConfig::default();
        let scene_config = SceneTypeConfig {
            hideout_keywords: parser_config.hideout_keywords().clone(),
            act_keywords: parser_config.act_keywords().clone(),
            zone_keywords: parser_config.zone_keywords().clone(),
        };

        let state_manager = LocationTracker::with_config(scene_config);

        Self {
            log_path,
            event_broadcaster: EventDispatcher::new(),
            state_manager: state_manager.clone(),
            server_manager,
            character_manager,
            parser_manager,
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    /// Get the event receiver for subscribing to all log events
    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<LogEvent> {
        self.event_broadcaster.subscribe()
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

    /// Get current log file size
    pub fn get_log_file_size(&self) -> AppResult<u64> {
        Self::get_file_size(&self.log_path)
    }

    /// Read the last N lines from the log file
    pub fn read_last_lines(&self, count: usize) -> AppResult<Vec<String>> {
        Self::read_file_lines(&self.log_path, count)
    }

    /// Check if the log file exists
    pub fn file_exists(&self) -> bool {
        Path::new(&self.log_path).exists()
    }

    /// Get the log path
    pub fn get_log_path(&self) -> &str {
        &self.log_path
    }

    // Private helper methods

    /// Start the main monitoring loop
    async fn start_monitoring_loop(&self) {
        let log_path = self.log_path.clone();
        let event_broadcaster = self.event_broadcaster.clone();
        let state_manager = self.state_manager.clone();
        let server_manager = Arc::clone(&self.server_manager);
        let character_manager = Arc::clone(&self.character_manager);
        let parser_manager = self.parser_manager.clone();
        let is_running = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            if let Err(e) = Self::monitor_log_file(
                log_path,
                event_broadcaster,
                state_manager,
                server_manager,
                character_manager,
                parser_manager,
                is_running,
            )
            .await
            {
                error!("Log monitoring failed: {}", e);
            }
        });
    }

    /// Monitor the log file for changes and parse new lines
    async fn monitor_log_file(
        log_path: String,
        event_broadcaster: EventDispatcher,
        state_manager: LocationTracker,
        server_manager: Arc<ServerMonitor>,
        character_manager: Arc<crate::services::character_manager::CharacterManager>,
        parser_manager: LogParserManager,
        is_running: Arc<tokio::sync::RwLock<bool>>,
    ) -> AppResult<()> {
        let mut last_position = Self::get_file_size(&log_path)?;
        let mut check_interval = time::interval(Duration::from_millis(100));

        loop {
            check_interval.tick().await;

            // Check if we should stop monitoring
            if !*is_running.read().await {
                debug!("Log monitoring stopped, exiting monitor loop");
                break;
            }

            // Check for new content in the log file
            let current_size = Self::get_file_size(&log_path)?;
            if current_size > last_position {
                Self::process_new_lines(
                    &log_path,
                    &mut last_position,
                    &parser_manager,
                    &event_broadcaster,
                    &state_manager,
                    &server_manager,
                    &character_manager,
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

    /// Get file size helper method
    fn get_file_size(log_path: &str) -> AppResult<u64> {
        let path = Path::new(log_path);

        if !path.exists() {
            return Err(AppError::LogMonitor(format!(
                "Log file not found: {}",
                log_path
            )));
        }

        let metadata = fs::metadata(path)
            .map_err(|e| AppError::FileSystem(format!("Failed to get file metadata: {}", e)))?;
        Ok(metadata.len())
    }

    /// Read file lines helper method
    fn read_file_lines(log_path: &str, count: usize) -> AppResult<Vec<String>> {
        let path = Path::new(log_path);

        if !path.exists() {
            return Err(AppError::LogMonitor(format!(
                "Log file not found: {}",
                log_path
            )));
        }

        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| AppError::FileSystem(format!("Failed to open log file: {}", e)))?;

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

        let start = if lines.len() > count {
            lines.len() - count
        } else {
            0
        };

        Ok(lines[start..].to_vec())
    }

    /// Open file for reading helper method
    fn open_file_for_reading(log_path: &str) -> AppResult<BufReader<std::fs::File>> {
        let path = Path::new(log_path);

        if !path.exists() {
            return Err(AppError::LogMonitor(format!(
                "Log file not found: {}",
                log_path
            )));
        }

        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| AppError::FileSystem(format!("Failed to open log file: {}", e)))?;

        Ok(BufReader::new(file))
    }

    /// Process new lines from the log file starting from the last known position
    async fn process_new_lines(
        log_path: &str,
        last_position: &mut u64,
        parser_manager: &LogParserManager,
        event_broadcaster: &EventDispatcher,
        state_manager: &LocationTracker,
        server_manager: &Arc<ServerMonitor>,
        character_manager: &Arc<crate::services::character_manager::CharacterManager>,
    ) -> AppResult<()> {
        let mut reader = Self::open_file_for_reading(log_path)?;

        // Seek to last known position
        reader
            .seek(io::SeekFrom::Start(*last_position))
            .map_err(|e| AppError::FileSystem(format!("Failed to seek in log file: {}", e)))?;

        // Collect all new lines first
        let mut new_lines = Vec::new();
        for line in reader.lines() {
            let line =
                line.map_err(|e| AppError::FileSystem(format!("Failed to read line: {}", e)))?;
            new_lines.push(line);
        }

        // Process all lines in a single async task
        if !new_lines.is_empty() {
            let parser_manager = parser_manager.clone();
            let event_broadcaster = event_broadcaster.clone();
            let server_manager = Arc::clone(server_manager);
            let state_manager = state_manager.clone();
            let character_manager = Arc::clone(character_manager);

            tokio::spawn(async move {
                for line in new_lines {
                    Self::process_single_line(
                        parser_manager.clone(),
                        event_broadcaster.clone(),
                        state_manager.clone(),
                        Arc::clone(&server_manager),
                        Arc::clone(&character_manager),
                        line,
                    )
                    .await;
                }
            });
        }

        // Update position to current file size
        *last_position = Self::get_file_size(log_path)?;

        Ok(())
    }

    /// Process a single log line for scene changes, server connections, and character level-ups
    async fn process_single_line(
        parser_manager: LogParserManager,
        event_broadcaster: EventDispatcher,
        state_manager: LocationTracker,
        server_manager: Arc<ServerMonitor>,
        character_manager: Arc<crate::services::character_manager::CharacterManager>,
        line: String,
    ) {
        // Parse the line using the unified parser
        if let Ok(Some(result)) = parser_manager.parse_line(&line) {
            match result {
                ParserResult::SceneChange(content) => {
                    // Process the raw content through the location tracker (business logic layer)
                    if let Some(validated_event) =
                        state_manager.process_scene_content(&content).await
                    {
                        // Broadcast as unified log event
                        if let Err(e) = event_broadcaster
                            .broadcast_event(LogEvent::SceneChange(validated_event))
                        {
                            warn!("Failed to broadcast scene change event: {}", e);
                        }
                    }
                }
                ParserResult::ServerConnection(event) => {
                    // Update server status manager
                    if let Err(e) = server_manager.update_server_info(&event).await {
                        warn!("Failed to update server info: {}", e);
                    }

                    // Broadcast as unified log event
                    if let Err(e) =
                        event_broadcaster.broadcast_event(LogEvent::ServerConnection(event))
                    {
                        warn!("Failed to broadcast server connection event: {}", e);
                    }
                }
                ParserResult::CharacterLevel((character_name, character_class, new_level)) => {
                    // Check if this level-up is for the active character
                    if let Some(active_character) = character_manager.get_active_character().await {
                        // Verify the character name and class match
                        if active_character.name == character_name && 
                           active_character.class == character_class {
                            
                            // Update the character's level
                            if let Err(e) = character_manager.update_character_level(
                                &active_character.id, 
                                new_level
                            ).await {
                                warn!("Failed to update character level: {}", e);
                            } else {
                                // Create and broadcast the level-up event
                                let level_up_event = crate::models::events::CharacterLevelUpEvent {
                                    character_name: character_name.clone(),
                                    character_class: character_class.to_string(),
                                    new_level,
                                    timestamp: chrono::Utc::now().to_rfc3339(),
                                };

                                if let Err(e) = event_broadcaster
                                    .broadcast_event(LogEvent::CharacterLevelUp(level_up_event))
                                {
                                    warn!("Failed to broadcast character level-up event: {}", e);
                                }
                            }
                        } else {
                            debug!(
                                "Level-up detected for inactive character: {} ({}) -> level {}",
                                character_name, character_class, new_level
                            );
                        }
                    } else {
                        debug!(
                            "Level-up detected but no active character: {} ({}) -> level {}",
                            character_name, character_class, new_level
                        );
                    }
                }
                ParserResult::CharacterDeath(character_name) => {
                    // Check if this death is for the active character
                    if let Some(active_character) = character_manager.get_active_character().await {
                        // Verify the character name matches
                        if active_character.name == character_name {
                            
                            // Increment the character's death count
                            if let Err(e) = character_manager.increment_character_deaths(
                                &active_character.id
                            ).await {
                                warn!("Failed to increment character death count: {}", e);
                            } else {
                                // Create and broadcast the death event
                                let death_event = crate::models::events::CharacterDeathEvent {
                                    character_name: character_name.clone(),
                                    timestamp: chrono::Utc::now().to_rfc3339(),
                                };

                                if let Err(e) = event_broadcaster
                                    .broadcast_event(LogEvent::CharacterDeath(death_event))
                                {
                                    warn!("Failed to broadcast character death event: {}", e);
                                }
                            }
                        } else {
                            debug!(
                                "Death detected for inactive character: {} has been slain",
                                character_name
                            );
                        }
                    } else {
                        debug!(
                            "Death detected but no active character: {} has been slain",
                            character_name
                        );
                    }
                }
            }
        }
    }
}

// Implement Clone for the components we need to move into async tasks
impl Clone for EventDispatcher {
    fn clone(&self) -> Self {
        Self {
            unified_event_sender: self.unified_event_sender.clone(),
            ping_event_sender: self.ping_event_sender.clone(),
        }
    }
}
