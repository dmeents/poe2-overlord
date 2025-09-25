use crate::domain::character::traits::CharacterService;
use crate::domain::log_analysis::models::LogEvent;
use crate::domain::log_analysis::traits::LogAnalysisService;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::monitoring::ServerMonitor;
use crate::infrastructure::parsing::{LogParserManager, ParserResult};
use crate::infrastructure::tauri::EventDispatcher;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time;

pub struct LogAnalyzer {
    log_path: String,
    event_broadcaster: Arc<EventDispatcher>,
    server_manager: Arc<ServerMonitor>,
    character_service: Arc<dyn CharacterService>,
    parser_manager: LogParserManager,
    is_running: Arc<tokio::sync::RwLock<bool>>,
}

impl LogAnalyzer {
    pub fn new(
        log_path: String,
        server_manager: Arc<ServerMonitor>,
        character_service: Arc<dyn CharacterService>,
    ) -> Self {
        let parser_manager = LogParserManager::new();

        Self {
            log_path,
            event_broadcaster: Arc::new(EventDispatcher::new()),
            server_manager,
            character_service,
            parser_manager,
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

        info!("Starting log file monitoring for scene changes and server connections");

        let server_manager = Arc::clone(&self.server_manager);
        tokio::spawn(async move {
            if let Err(e) = server_manager.load_status().await {
                warn!("Failed to load server status: {}", e);
            }

            server_manager.start_periodic_ping().await;
        });

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

    async fn start_monitoring_loop(&self) {
        let log_path = self.log_path.clone();
        let event_broadcaster = Arc::clone(&self.event_broadcaster);
        let server_manager = Arc::clone(&self.server_manager);
        let character_service = Arc::clone(&self.character_service);
        let parser_manager = self.parser_manager.clone();
        let is_running = Arc::clone(&self.is_running);

        tokio::spawn(async move {
            let mut last_position = 0u64;
            let mut interval = time::interval(Duration::from_millis(500));

            loop {
                interval.tick().await;

                {
                    let running = is_running.read().await;
                    if !*running {
                        debug!("Log monitoring loop stopped");
                        break;
                    }
                }

                if !Path::new(&log_path).exists() {
                    debug!("Log file does not exist yet: {}", log_path);
                    continue;
                }

                let current_size = match fs::metadata(&log_path) {
                    Ok(metadata) => metadata.len(),
                    Err(e) => {
                        error!("Failed to get log file metadata: {}", e);
                        continue;
                    }
                };

                if current_size <= last_position {
                    continue;
                }

                match Self::read_new_content(&log_path, last_position, current_size).await {
                    Ok(new_content) => {
                        if !new_content.is_empty() {
                            debug!("Read {} bytes of new content", new_content.len());

                            for line in new_content.lines() {
                                match parser_manager.parse_line(line) {
                                    Ok(Some(parser_result)) => {
                                        Self::handle_parser_result(
                                            parser_result,
                                            &event_broadcaster,
                                            &server_manager,
                                            &character_service,
                                        )
                                        .await;
                                    }
                                    Ok(None) => {}
                                    Err(_) => {}
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

    async fn handle_parser_result(
        parser_result: ParserResult,
        event_broadcaster: &Arc<EventDispatcher>,
        server_manager: &Arc<ServerMonitor>,
        character_service: &Arc<dyn CharacterService>,
    ) {
        match parser_result {
            ParserResult::SceneChange(scene_content) => {
                debug!("Scene change detected: {}", scene_content);

                let zone_change_event = crate::domain::log_analysis::models::ZoneChangeEvent {
                    zone_name: scene_content,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                let scene_change_event =
                    crate::domain::log_analysis::models::SceneChangeEvent::Zone(zone_change_event);
                let log_event = LogEvent::SceneChange(scene_change_event);
                if let Err(e) = event_broadcaster.broadcast_event(log_event) {
                    error!("Failed to broadcast scene change event: {}", e);
                }
            }
            ParserResult::ServerConnection(connection_event) => {
                debug!("Server connection detected: {:?}", connection_event);

                if let Err(e) = server_manager.update_server_info(&connection_event).await {
                    error!("Failed to update server info: {}", e);
                }

                let log_event = LogEvent::ServerConnection(connection_event);
                if let Err(e) = event_broadcaster.broadcast_event(log_event) {
                    error!("Failed to broadcast server connection event: {}", e);
                }
            }
            ParserResult::CharacterLevel((character_name, character_class, level)) => {
                debug!(
                    "Character level detected: {} ({}): level {}",
                    character_name, character_class, level
                );

                if let Err(e) = character_service
                    .update_character_level(&character_name, level)
                    .await
                {
                    error!("Failed to update character level: {}", e);
                }

                let level_up_event = crate::domain::log_analysis::models::CharacterLevelUpEvent {
                    character_name,
                    character_class: character_class.to_string(),
                    new_level: level,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                let log_event = LogEvent::CharacterLevelUp(level_up_event);
                if let Err(e) = event_broadcaster.broadcast_event(log_event) {
                    error!("Failed to broadcast character level up event: {}", e);
                }
            }
            ParserResult::CharacterDeath(character_name) => {
                debug!("Character death detected: {}", character_name);

                let death_event = crate::domain::log_analysis::models::CharacterDeathEvent {
                    character_name,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                };

                let log_event = LogEvent::CharacterDeath(death_event);
                if let Err(e) = event_broadcaster.broadcast_event(log_event) {
                    error!("Failed to broadcast character death event: {}", e);
                }
            }
        }
    }
}

#[async_trait]
impl LogAnalysisService for LogAnalyzer {
    async fn start_monitoring(&self) -> AppResult<()> {
        self.start_monitoring().await
    }

    async fn stop_monitoring(&self) -> AppResult<()> {
        self.stop_monitoring().await
    }

    async fn is_monitoring(&self) -> bool {
        self.is_monitoring().await
    }

    async fn get_log_file_info(
        &self,
    ) -> AppResult<crate::domain::log_analysis::models::LogFileInfo> {
        let size = self.get_log_file_size().await?;
        let exists = std::path::Path::new(&self.log_path).exists();

        Ok(crate::domain::log_analysis::models::LogFileInfo {
            path: std::path::PathBuf::from(&self.log_path),
            size,
            exists,
            last_modified: chrono::Utc::now(),
        })
    }

    async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>> {
        self.read_log_lines(start_line, count).await
    }

    async fn get_analysis_stats(
        &self,
    ) -> AppResult<crate::domain::log_analysis::models::LogAnalysisStats> {
        Ok(crate::domain::log_analysis::models::LogAnalysisStats::default())
    }

    fn subscribe_to_events(&self) -> broadcast::Receiver<LogEvent> {
        self.subscribe()
    }

    async fn update_log_path(&self, _new_path: String) -> AppResult<()> {
        Ok(())
    }

    async fn get_config(&self) -> crate::domain::log_analysis::models::LogAnalysisConfig {
        crate::domain::log_analysis::models::LogAnalysisConfig::default()
    }

    async fn update_config(
        &self,
        _config: crate::domain::log_analysis::models::LogAnalysisConfig,
    ) -> AppResult<()> {
        Ok(())
    }
}
