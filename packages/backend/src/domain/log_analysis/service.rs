use crate::domain::character::traits::CharacterService as CharacterServiceTrait;
use crate::domain::events::{AppEvent, EventBus, EventType};
use crate::domain::log_analysis::models::LogEvent;
use crate::domain::log_analysis::models::{
    LogAnalysisConfig, LogAnalysisSession, LogAnalysisStats, LogFileInfo,
};
use crate::domain::log_analysis::repository::{
    LogAnalysisSessionRepositoryImpl, LogAnalysisStatsRepositoryImpl, LogFileRepositoryImpl,
};
use crate::domain::log_analysis::traits::{
    LogAnalysisService, LogAnalysisSessionRepository, LogAnalysisStatsRepository, LogFileRepository,
};
use crate::domain::server_monitoring::traits::ServerMonitoringService as ServerMonitoringServiceTrait;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::parsing::LogParserManager;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time;

/// Main implementation of the log analysis service
/// Handles monitoring game log files, parsing events, and coordinating with other services
pub struct LogAnalysisServiceImpl {
    /// Configuration for log analysis operations
    config: Arc<RwLock<LogAnalysisConfig>>,
    /// Repository for file system operations on log files
    log_file_repository: Arc<dyn LogFileRepository>,
    /// Repository for managing log analysis sessions
    session_repository: Arc<dyn LogAnalysisSessionRepository>,
    /// Repository for managing log analysis statistics
    stats_repository: Arc<dyn LogAnalysisStatsRepository>,
    /// Event bus for publishing log events
    event_bus: Arc<EventBus>,
    /// Service for character-related operations
    character_service: Arc<dyn CharacterServiceTrait>,
    /// Service for server monitoring operations
    server_monitoring_service: Arc<dyn ServerMonitoringServiceTrait>,
    /// Parser manager for processing log lines
    parser_manager: LogParserManager,
    /// Flag indicating whether log monitoring is currently active
    is_running: Arc<RwLock<bool>>,
    /// The currently active log analysis session
    current_session: Arc<RwLock<Option<LogAnalysisSession>>>,
    /// Last position read in the log file (for incremental reading)
    last_position: Arc<RwLock<u64>>,
}

impl LogAnalysisServiceImpl {
    /// Creates a new LogAnalysisServiceImpl with default repositories
    pub fn new(
        config: LogAnalysisConfig,
        character_service: Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: Arc<dyn ServerMonitoringServiceTrait>,
        event_bus: Arc<EventBus>,
    ) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(config));
        let log_file_repository = Arc::new(LogFileRepositoryImpl::new(String::new()));

        let session_repository = Arc::new(LogAnalysisSessionRepositoryImpl::new()?);
        let stats_repository = Arc::new(LogAnalysisStatsRepositoryImpl::new()?);
        let parser_manager = LogParserManager::new();
        Ok(Self {
            config,
            log_file_repository,
            session_repository,
            stats_repository,
            event_bus,
            character_service,
            server_monitoring_service,
            parser_manager,
            is_running: Arc::new(RwLock::new(false)),
            current_session: Arc::new(RwLock::new(None)),
            last_position: Arc::new(RwLock::new(0)),
        })
    }

    /// Creates a new LogAnalysisServiceImpl with custom repositories (for testing)
    pub fn with_repositories(
        config: LogAnalysisConfig,
        log_file_repository: Arc<dyn LogFileRepository>,
        session_repository: Arc<dyn LogAnalysisSessionRepository>,
        stats_repository: Arc<dyn LogAnalysisStatsRepository>,
        character_service: Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: Arc<dyn ServerMonitoringServiceTrait>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let parser_manager = LogParserManager::new();
        Self {
            config,
            log_file_repository,
            session_repository,
            stats_repository,
            event_bus,
            character_service,
            server_monitoring_service,
            parser_manager,
            is_running: Arc::new(RwLock::new(false)),
            current_session: Arc::new(RwLock::new(None)),
            last_position: Arc::new(RwLock::new(0)),
        }
    }

    /// Starts the main monitoring loop for log file analysis
    async fn start_monitoring_loop(&self) -> AppResult<()> {
        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        drop(config);

        if log_path.is_empty() {
            return Err(AppError::config_error(
                "get_log_file_info",
                "Log file path not configured",
            ));
        }

        // Initialize the last position to the current file size
        let file_size = self.log_file_repository.get_file_size(&log_path).await?;
        {
            let mut last_pos = self.last_position.write().await;
            *last_pos = file_size;
        }

        // Create and save a new analysis session
        let session = LogAnalysisSession::new();
        {
            let mut current_session = self.current_session.write().await;
            *current_session = Some(session.clone());
        }
        self.session_repository.save_session(&session).await?;

        info!("Starting log file monitoring for: {}", log_path);

        let monitoring_task = self.create_monitoring_task();
        monitoring_task.await?;

        Ok(())
    }

    /// Creates and spawns the background monitoring task
    async fn create_monitoring_task(&self) -> AppResult<()> {
        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        let interval_ms = config.monitoring_interval_ms;
        drop(config);

        // Clone all necessary dependencies for the spawned task
        let log_file_repository = Arc::clone(&self.log_file_repository);
        let event_bus = Arc::clone(&self.event_bus);
        let character_service = Arc::clone(&self.character_service);
        let server_monitoring_service = Arc::clone(&self.server_monitoring_service);
        let is_running = Arc::clone(&self.is_running);
        let last_position = Arc::clone(&self.last_position);
        let current_session = Arc::clone(&self.current_session);
        let session_repository = Arc::clone(&self.session_repository);
        let stats_repository = Arc::clone(&self.stats_repository);
        let parser_manager = self.parser_manager.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(interval_ms));

            loop {
                interval.tick().await;

                // Check if monitoring should continue
                if !*is_running.read().await {
                    debug!("Log monitoring stopped, exiting monitor loop");
                    break;
                }

                // Check for new content in the log file
                match log_file_repository.get_file_size(&log_path).await {
                    Ok(current_size) => {
                        let last_pos = *last_position.read().await;
                        if current_size > last_pos {
                            // New content detected, process it
                            if let Err(e) = Self::process_new_lines(
                                &parser_manager,
                                &log_path,
                                &log_file_repository,
                                &event_bus,
                                &character_service,
                                &server_monitoring_service,
                                &last_position,
                                &current_session,
                                &session_repository,
                                &stats_repository,
                                last_pos,
                            )
                            .await
                            {
                                error!("Failed to process new log lines: {}", e);
                            }
                        } else if current_size < last_pos {
                            // File was truncated, reset position
                            warn!("Log file was truncated, resetting position");
                            let mut pos = last_position.write().await;
                            *pos = current_size;
                        }
                    }
                    Err(e) => {
                        error!("Failed to get log file size: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Processes new lines that have been added to the log file
    async fn process_new_lines(
        parser_manager: &LogParserManager,
        log_path: &str,
        log_file_repository: &Arc<dyn LogFileRepository>,
        event_bus: &Arc<EventBus>,
        character_service: &Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: &Arc<dyn ServerMonitoringServiceTrait>,
        last_position: &Arc<RwLock<u64>>,
        current_session: &Arc<RwLock<Option<LogAnalysisSession>>>,
        session_repository: &Arc<dyn LogAnalysisSessionRepository>,
        stats_repository: &Arc<dyn LogAnalysisStatsRepository>,
        start_position: u64,
    ) -> AppResult<()> {
        // Read new lines from the last known position
        let new_lines = log_file_repository
            .read_from_position(log_path, start_position)
            .await?;

        if new_lines.is_empty() {
            return Ok(());
        }

        // Process each new line
        for line in &new_lines {
            if let Err(e) = Self::process_single_line(
                parser_manager,
                line,
                event_bus,
                character_service,
                server_monitoring_service,
                current_session,
                stats_repository,
            )
            .await
            {
                error!("Failed to process log line: {}", e);
            }
        }

        // Update the last position to the current file size
        let current_size = log_file_repository.get_file_size(log_path).await?;
        {
            let mut pos = last_position.write().await;
            *pos = current_size;
        }

        // Update the current session with the number of lines processed
        if let Some(mut session) = current_session.read().await.clone() {
            session.events_processed += new_lines.len() as u64;
            session.last_position = current_size;
            session_repository.update_session(&session).await?;

            let mut current_session_guard = current_session.write().await;
            *current_session_guard = Some(session);
        }

        Ok(())
    }

    /// Processes a single log line and handles any detected events
    async fn process_single_line(
        parser_manager: &LogParserManager,
        line: &str,
        event_bus: &Arc<EventBus>,
        character_service: &Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: &Arc<dyn ServerMonitoringServiceTrait>,
        _current_session: &Arc<RwLock<Option<LogAnalysisSession>>>,
        stats_repository: &Arc<dyn LogAnalysisStatsRepository>,
    ) -> AppResult<()> {
        // Try to parse the line for known events
        if let Ok(Some(result)) = parser_manager.parse_line(line) {
            match result {
                crate::infrastructure::parsing::ParserResult::SceneChange(content) => {
                    // Handle zone change events
                    let event = LogEvent::SceneChange(
                        crate::domain::log_analysis::models::SceneChangeEvent::Zone(
                            crate::domain::log_analysis::models::ZoneChangeEvent {
                                zone_name: content,
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            },
                        ),
                    );

                    if let Err(e) = event_bus.publish(AppEvent::LogParsed(event)).await {
                        warn!("Failed to publish log event: {}", e);
                    }
                    stats_repository
                        .increment_event_count("scene_change")
                        .await?;
                }
                crate::infrastructure::parsing::ParserResult::ServerConnection(event) => {
                    // Handle server connection events
                    let server_status = crate::domain::server_monitoring::models::ServerStatus::from_connection_event(&event);
                    server_monitoring_service
                        .update_status(server_status)
                        .await?;

                    if let Err(e) = event_bus
                        .publish(AppEvent::LogParsed(LogEvent::ServerConnection(event)))
                        .await
                    {
                        warn!("Failed to publish server connection event: {}", e);
                    }
                    stats_repository
                        .increment_event_count("server_connection")
                        .await?;
                }
                crate::infrastructure::parsing::ParserResult::CharacterLevel((
                    character_name,
                    character_class,
                    new_level,
                )) => {
                    // Handle character level up events
                    if let Some(active_character) = character_service.get_active_character().await {
                        if active_character.name == character_name
                            && active_character.class == character_class
                        {
                            character_service
                                .update_character_level(&active_character.id, new_level)
                                .await?;

                            let level_up_event =
                                crate::domain::log_analysis::models::CharacterLevelUpEvent {
                                    character_name: character_name.clone(),
                                    character_class: character_class.to_string(),
                                    new_level,
                                    timestamp: chrono::Utc::now().to_rfc3339(),
                                };

                            if let Err(e) = event_bus
                                .publish(AppEvent::LogParsed(LogEvent::CharacterLevelUp(
                                    level_up_event,
                                )))
                                .await
                            {
                                warn!("Failed to publish character level up event: {}", e);
                            }
                            stats_repository
                                .increment_event_count("character_level_up")
                                .await?;
                        }
                    }
                }
                crate::infrastructure::parsing::ParserResult::CharacterDeath(character_name) => {
                    // Handle character death events
                    if let Some(active_character) = character_service.get_active_character().await {
                        if active_character.name == character_name {
                            character_service
                                .increment_character_deaths(&active_character.id)
                                .await?;

                            let death_event =
                                crate::domain::log_analysis::models::CharacterDeathEvent {
                                    character_name: character_name.clone(),
                                    timestamp: chrono::Utc::now().to_rfc3339(),
                                };

                            if let Err(e) = event_bus
                                .publish(AppEvent::LogParsed(LogEvent::CharacterDeath(death_event)))
                                .await
                            {
                                warn!("Failed to publish character death event: {}", e);
                            }
                            stats_repository
                                .increment_event_count("character_death")
                                .await?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[async_trait]
impl LogAnalysisService for LogAnalysisServiceImpl {
    /// Starts monitoring the configured log file for new events
    async fn start_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Log monitoring is already running");
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        self.start_monitoring_loop().await
    }

    /// Stops the current log monitoring session
    async fn stop_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            warn!("Log monitoring is not running");
            return Ok(());
        }

        *is_running = false;

        // End the current session if it exists
        if let Some(mut session) = self.current_session.read().await.clone() {
            session.end_session();
            self.session_repository.update_session(&session).await?;
        }

        self.session_repository.end_current_session().await?;

        info!("Log monitoring stopped");
        Ok(())
    }

    /// Returns whether log monitoring is currently active
    async fn is_monitoring(&self) -> bool {
        *self.is_running.read().await
    }

    /// Gets information about the currently monitored log file
    async fn get_log_file_info(&self) -> AppResult<LogFileInfo> {
        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        drop(config);

        if log_path.is_empty() {
            return Err(AppError::config_error(
                "get_log_file_info",
                "Log file path not configured",
            ));
        }

        self.log_file_repository.get_file_info(&log_path).await
    }

    /// Reads a specified number of lines from the log file starting at a given line
    async fn read_log_lines(&self, start_line: usize, count: usize) -> AppResult<Vec<String>> {
        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        drop(config);

        if log_path.is_empty() {
            return Err(AppError::config_error(
                "get_log_file_info",
                "Log file path not configured",
            ));
        }

        self.log_file_repository
            .read_lines(&log_path, start_line, count)
            .await
    }

    /// Gets current statistics about log analysis activity
    async fn get_analysis_stats(&self) -> AppResult<LogAnalysisStats> {
        self.stats_repository.load_stats().await
    }

    /// Subscribes to log events published by the service
    async fn subscribe_to_events(&self) -> AppResult<broadcast::Receiver<AppEvent>> {
        self.event_bus.get_receiver(EventType::LogAnalysis).await
    }

    /// Updates the path to the log file being monitored
    async fn update_log_path(&self, new_path: String) -> AppResult<()> {
        let mut config = self.config.write().await;
        config.log_file_path = new_path;
        drop(config);
        Ok(())
    }

    /// Gets the current log analysis configuration
    async fn get_config(&self) -> LogAnalysisConfig {
        self.config.read().await.clone()
    }

    /// Updates the log analysis configuration
    async fn update_config(&self, new_config: LogAnalysisConfig) -> AppResult<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }
}
