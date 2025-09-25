use crate::domain::character::traits::CharacterService as CharacterServiceTrait;
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
use crate::infrastructure::tauri::EventPublisher;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, RwLock};
use tokio::time;

pub struct LogAnalysisServiceImpl {
    config: Arc<RwLock<LogAnalysisConfig>>,
    log_file_repository: Arc<dyn LogFileRepository>,
    session_repository: Arc<dyn LogAnalysisSessionRepository>,
    stats_repository: Arc<dyn LogAnalysisStatsRepository>,
    event_publisher: Arc<EventPublisher>,
    parser_manager: LogParserManager,
    character_service: Arc<dyn CharacterServiceTrait>,
    server_monitoring_service: Arc<dyn ServerMonitoringServiceTrait>,
    is_running: Arc<RwLock<bool>>,
    current_session: Arc<RwLock<Option<LogAnalysisSession>>>,
    last_position: Arc<RwLock<u64>>,
}

impl LogAnalysisServiceImpl {
    pub fn new(
        config: LogAnalysisConfig,
        character_service: Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: Arc<dyn ServerMonitoringServiceTrait>,
        event_publisher: Arc<EventPublisher>,
    ) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(config));
        let log_file_repository = Arc::new(LogFileRepositoryImpl::new(String::new()));

        let sessions_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
            .join("sessions")
            .to_string_lossy()
            .to_string();

        let stats_file_path = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("poe2-overlord")
            .join("log_analysis_stats.json")
            .to_string_lossy()
            .to_string();

        let session_repository = Arc::new(LogAnalysisSessionRepositoryImpl::new(sessions_dir));
        let stats_repository = Arc::new(LogAnalysisStatsRepositoryImpl::new(stats_file_path));
        let parser_manager = LogParserManager::new();

        Ok(Self {
            config,
            log_file_repository,
            session_repository,
            stats_repository,
            event_publisher,
            parser_manager,
            character_service,
            server_monitoring_service,
            is_running: Arc::new(RwLock::new(false)),
            current_session: Arc::new(RwLock::new(None)),
            last_position: Arc::new(RwLock::new(0)),
        })
    }

    pub fn with_repositories(
        config: LogAnalysisConfig,
        log_file_repository: Arc<dyn LogFileRepository>,
        session_repository: Arc<dyn LogAnalysisSessionRepository>,
        stats_repository: Arc<dyn LogAnalysisStatsRepository>,
        character_service: Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: Arc<dyn ServerMonitoringServiceTrait>,
        event_publisher: Arc<EventPublisher>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let parser_manager = LogParserManager::new();

        Self {
            config,
            log_file_repository,
            session_repository,
            stats_repository,
            event_publisher,
            parser_manager,
            character_service,
            server_monitoring_service,
            is_running: Arc::new(RwLock::new(false)),
            current_session: Arc::new(RwLock::new(None)),
            last_position: Arc::new(RwLock::new(0)),
        }
    }

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

        let file_size = self.log_file_repository.get_file_size(&log_path).await?;
        {
            let mut last_pos = self.last_position.write().await;
            *last_pos = file_size;
        }

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

    async fn create_monitoring_task(&self) -> AppResult<()> {
        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        let interval_ms = config.monitoring_interval_ms;
        drop(config);

        let log_file_repository = Arc::clone(&self.log_file_repository);
        let event_publisher = Arc::clone(&self.event_publisher);
        let parser_manager = self.parser_manager.clone();
        let character_service = Arc::clone(&self.character_service);
        let server_monitoring_service = Arc::clone(&self.server_monitoring_service);
        let is_running = Arc::clone(&self.is_running);
        let last_position = Arc::clone(&self.last_position);
        let current_session = Arc::clone(&self.current_session);
        let session_repository = Arc::clone(&self.session_repository);
        let stats_repository = Arc::clone(&self.stats_repository);

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(interval_ms));

            loop {
                interval.tick().await;

                if !*is_running.read().await {
                    debug!("Log monitoring stopped, exiting monitor loop");
                    break;
                }

                match log_file_repository.get_file_size(&log_path).await {
                    Ok(current_size) => {
                        let last_pos = *last_position.read().await;
                        if current_size > last_pos {
                            if let Err(e) = Self::process_new_lines(
                                &log_path,
                                &log_file_repository,
                                &event_publisher,
                                &parser_manager,
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

    async fn process_new_lines(
        log_path: &str,
        log_file_repository: &Arc<dyn LogFileRepository>,
        event_publisher: &Arc<EventPublisher>,
        parser_manager: &LogParserManager,
        character_service: &Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: &Arc<dyn ServerMonitoringServiceTrait>,
        last_position: &Arc<RwLock<u64>>,
        current_session: &Arc<RwLock<Option<LogAnalysisSession>>>,
        session_repository: &Arc<dyn LogAnalysisSessionRepository>,
        stats_repository: &Arc<dyn LogAnalysisStatsRepository>,
        start_position: u64,
    ) -> AppResult<()> {
        let new_lines = log_file_repository
            .read_from_position(log_path, start_position)
            .await?;

        if new_lines.is_empty() {
            return Ok(());
        }

        for line in &new_lines {
            if let Err(e) = Self::process_single_line(
                &line,
                parser_manager,
                event_publisher,
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

        let current_size = log_file_repository.get_file_size(log_path).await?;
        {
            let mut pos = last_position.write().await;
            *pos = current_size;
        }

        if let Some(mut session) = current_session.read().await.clone() {
            session.events_processed += new_lines.len() as u64;
            session.last_position = current_size;
            session_repository.update_session(&session).await?;

            let mut current_session_guard = current_session.write().await;
            *current_session_guard = Some(session);
        }

        Ok(())
    }

    async fn process_single_line(
        line: &str,
        parser_manager: &LogParserManager,
        event_publisher: &Arc<EventPublisher>,
        character_service: &Arc<dyn CharacterServiceTrait>,
        server_monitoring_service: &Arc<dyn ServerMonitoringServiceTrait>,
        _current_session: &Arc<RwLock<Option<LogAnalysisSession>>>,
        stats_repository: &Arc<dyn LogAnalysisStatsRepository>,
    ) -> AppResult<()> {
        if let Ok(Some(result)) = parser_manager.parse_line(line) {
            match result {
                crate::infrastructure::parsing::ParserResult::SceneChange(content) => {
                    let event = LogEvent::SceneChange(
                        crate::domain::log_analysis::models::SceneChangeEvent::Zone(
                            crate::domain::log_analysis::models::ZoneChangeEvent {
                                zone_name: content,
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            },
                        ),
                    );

                    if let Err(e) = event_publisher.broadcast_log_event(event) {
                        warn!("Failed to broadcast log event: {}", e);
                    }
                    stats_repository
                        .increment_event_count("scene_change")
                        .await?;
                }
                crate::infrastructure::parsing::ParserResult::ServerConnection(event) => {
                    let server_status = crate::domain::server_monitoring::models::ServerStatus::from_connection_event(&event);
                    server_monitoring_service
                        .update_status(server_status)
                        .await?;

                    if let Err(e) =
                        event_publisher.broadcast_log_event(LogEvent::ServerConnection(event))
                    {
                        warn!("Failed to broadcast server connection event: {}", e);
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

                            if let Err(e) = event_publisher
                                .broadcast_log_event(LogEvent::CharacterLevelUp(level_up_event))
                            {
                                warn!("Failed to broadcast character level up event: {}", e);
                            }
                            stats_repository
                                .increment_event_count("character_level_up")
                                .await?;
                        }
                    }
                }
                crate::infrastructure::parsing::ParserResult::CharacterDeath(character_name) => {
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

                            if let Err(e) = event_publisher
                                .broadcast_log_event(LogEvent::CharacterDeath(death_event))
                            {
                                warn!("Failed to broadcast character death event: {}", e);
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

    async fn stop_monitoring(&self) -> AppResult<()> {
        let mut is_running = self.is_running.write().await;
        if !*is_running {
            warn!("Log monitoring is not running");
            return Ok(());
        }

        *is_running = false;

        if let Some(mut session) = self.current_session.read().await.clone() {
            session.end_session();
            self.session_repository.update_session(&session).await?;
        }

        self.session_repository.end_current_session().await?;

        info!("Log monitoring stopped");
        Ok(())
    }

    async fn is_monitoring(&self) -> bool {
        *self.is_running.read().await
    }

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

    async fn get_analysis_stats(&self) -> AppResult<LogAnalysisStats> {
        self.stats_repository.load_stats().await
    }

    fn subscribe_to_events(&self) -> broadcast::Receiver<LogEvent> {
        self.event_publisher.subscribe_to_log_events()
    }

    async fn update_log_path(&self, new_path: String) -> AppResult<()> {
        let mut config = self.config.write().await;
        config.log_file_path = new_path;
        drop(config);
        Ok(())
    }

    async fn get_config(&self) -> LogAnalysisConfig {
        self.config.read().await.clone()
    }

    async fn update_config(&self, new_config: LogAnalysisConfig) -> AppResult<()> {
        let mut config = self.config.write().await;
        *config = new_config;
        Ok(())
    }
}
