use crate::infrastructure::parsing::{LogParserManager, ParserResult};
use crate::infrastructure::tauri::EventDispatcher;
use crate::domain::log_analysis::models::{LogEvent, SceneChangeEvent, CharacterLevelUpEvent, CharacterDeathEvent};
use crate::errors::{AppError, AppResult};
use log::{debug, error, info, warn};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

/// Simplified log file analyzer focused on file monitoring and parsing
/// 
/// Monitors the game's log file for new content and parses it to extract
/// various game events. Broadcasts parsed results to subscribers for further processing.
/// This version focuses only on infrastructure concerns (file monitoring and parsing)
/// and delegates domain logic to event handlers.
pub struct LogAnalyzer {
    /// Path to the POE2 client log file
    log_path: String,
    /// Event broadcaster for notifying subscribers of parsed events
    event_broadcaster: Arc<EventDispatcher>,
    /// Flag indicating whether monitoring is currently active
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogAnalyzer {
    pub fn new(log_path: String) -> Self {
        Self {
            log_path,
            event_broadcaster: Arc::new(EventDispatcher::new()),
            is_running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<LogEvent> {
        self.event_broadcaster.subscribe()
    }

    pub async fn start_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Log monitoring is already running");
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        info!("Starting log file monitoring for game events");

        self.start_monitoring_loop().await;

        Ok(())
    }

    pub async fn stop_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            warn!("Log monitoring is not running");
            return Ok(());
        }

        *is_running = false;
        drop(is_running);

        info!("Stopping log file monitoring");
        Ok(())
    }

    pub async fn is_monitoring(&self) -> bool {
        let is_running = self.is_running.read().await;
        *is_running
    }

    pub async fn get_log_file_size(&self) -> AppResult<u64> {
        let metadata = fs::metadata(&self.log_path).map_err(|e| {
            AppError::file_system_error("Failed to get log file metadata: {}", &e.to_string())
        })?;
        Ok(metadata.len())
    }

    pub async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>> {
        let file = OpenOptions::new()
            .read(true)
            .open(&self.log_path)
            .map_err(|e| {
                AppError::file_system_error("Failed to open log file: {}", &e.to_string())
            })?;

        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                if index >= start_line && index < start_line + count {
                    line.ok()
                } else {
                    None
                }
            })
            .collect();

        Ok(lines)
    }

    /// Starts the main monitoring loop in a background task
    /// 
    /// Continuously monitors the log file for new content and processes it.
    /// Uses a 500ms polling interval to balance responsiveness with performance.
    async fn start_monitoring_loop(&self) {
        let log_path = self.log_path.clone();
        let event_broadcaster = Arc::clone(&self.event_broadcaster);
        let parser_manager = LogParserManager::new();
        let is_running = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            let mut last_position = 0u64;
            let mut interval = time::interval(Duration::from_millis(500));

            loop {
                interval.tick().await;

                // Check if monitoring should continue
                {
                    let running = is_running.read().await;
                    if !*running {
                        debug!("Log monitoring loop stopped");
                        break;
                    }
                }

                // Wait for log file to exist
                if !Path::new(&log_path).exists() {
                    debug!("Log file does not exist yet: {}", log_path);
                    continue;
                }

                // Get current file size
                let current_size = match fs::metadata(&log_path) {
                    Ok(metadata) => metadata.len(),
                    Err(e) => {
                        error!("Failed to get log file metadata: {}", e);
                        continue;
                    }
                };

                // Skip if no new content
                if current_size <= last_position {
                    continue;
                }

                // Read and process new content
                match Self::read_new_content(&log_path, last_position, current_size).await {
                    Ok(new_content) => {
                        if !new_content.is_empty() {
                            debug!("Read {} bytes of new content", new_content.len());

                            // Parse each line of new content
                            for line in new_content.lines() {
                                match parser_manager.parse_line(line) {
                                    Ok(Some(parser_result)) => {
                                        debug!("Parsed event: {:?}", parser_result);
                                        
                                        // Convert to LogEvent and broadcast for domain services to handle
                                        let log_event = Self::convert_to_log_event(parser_result);
                                        if let Err(e) = event_broadcaster.broadcast_event(log_event) {
                                            error!("Failed to broadcast parsed event: {}", e);
                                        }
                                    }
                                    Ok(None) => {} // No parser matched this line
                                    Err(e) => {
                                        debug!("Parser error for line: {}", e);
                                    }
                                }
                            }
                        }
                        last_position = current_size;
                    }
                    Err(e) => {
                        error!("Failed to read new content: {}", e);
                    }
                }
            }
        });
    }

    async fn read_new_content(
        log_path: &str,
        start_position: u64,
        end_position: u64,
    ) -> AppResult<String> {
        let mut file = OpenOptions::new().read(true).open(log_path).map_err(|e| {
            AppError::file_system_error("Failed to open log file: {}", &e.to_string())
        })?;

        file.seek(io::SeekFrom::Start(start_position))
            .map_err(|e| {
                AppError::file_system_error("Failed to seek in log file: {}", &e.to_string())
            })?;

        let mut buffer = vec![0u8; (end_position - start_position) as usize];
        file.read_exact(&mut buffer).map_err(|e| {
            AppError::file_system_error("Failed to read from log file: {}", &e.to_string())
        })?;

        String::from_utf8(buffer).map_err(|e| {
            AppError::serialization_error("Invalid UTF-8 in log file: {}", &e.to_string())
        })
    }

    /// Converts a ParserResult to a LogEvent for broadcasting
    fn convert_to_log_event(parser_result: ParserResult) -> LogEvent {
        match parser_result {
            ParserResult::SceneChange(content) => {
                // For now, treat all scene changes as zone changes
                // The domain service can handle proper scene type detection
                let zone_change_event = crate::domain::log_analysis::models::ZoneChangeEvent {
                    zone_name: content,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                LogEvent::SceneChange(SceneChangeEvent::Zone(zone_change_event))
            }
            ParserResult::ServerConnection(event) => {
                LogEvent::ServerConnection(event)
            }
            ParserResult::CharacterLevel((name, class, level)) => {
                let level_up_event = CharacterLevelUpEvent {
                    character_name: name,
                    character_class: class.to_string(),
                    new_level: level,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                LogEvent::CharacterLevelUp(level_up_event)
            }
            ParserResult::CharacterDeath(name) => {
                let death_event = CharacterDeathEvent {
                    character_name: name,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };
                LogEvent::CharacterDeath(death_event)
            }
        }
    }
}
