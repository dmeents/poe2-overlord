// CharacterServiceTrait is no longer needed - using CharacterService directly
use crate::domain::character::traits::CharacterService;
use crate::domain::log_analysis::models::LogAnalysisConfig;
use crate::domain::log_analysis::repository::LogFileRepositoryImpl;
use crate::domain::log_analysis::traits::{LogAnalysisService, LogFileRepository};
use crate::domain::server_monitoring::ServerMonitoringService;
use crate::domain::walkthrough::traits::WalkthroughService;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::parsing::LogParserManager;
use async_trait::async_trait;
use log::{error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

/// Main implementation of the log analysis service
/// Handles monitoring game log files, parsing events, and coordinating with other services
pub struct LogAnalysisServiceImpl {
    /// Configuration for log analysis operations
    config: Arc<RwLock<LogAnalysisConfig>>,
    /// Repository for file system operations on log files
    log_file_repository: Arc<dyn LogFileRepository>,
    /// Service for character operations (includes tracking)
    character_service: Arc<dyn CharacterService>,
    /// Service for server monitoring operations
    server_monitoring_service: Arc<dyn ServerMonitoringService>,
    /// Service for walkthrough guide and progress tracking
    walkthrough_service: Arc<dyn WalkthroughService>,
    /// Parser manager for processing log lines
    parser_manager: LogParserManager,
    /// Flag indicating whether log monitoring is currently active
    is_running: Arc<RwLock<bool>>,
    /// Last position read in the log file (for incremental reading)
    last_position: Arc<RwLock<u64>>,
    /// Cache for zone level information (level, timestamp)
    zone_level_cache: Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
}

impl LogAnalysisServiceImpl {
    /// Creates a new LogAnalysisServiceImpl with default repositories
    pub fn new(
        config: LogAnalysisConfig,
        character_service: Arc<dyn CharacterService>,
        server_monitoring_service: Arc<dyn ServerMonitoringService>,
        walkthrough_service: Arc<dyn WalkthroughService>,
    ) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(config));
        let log_file_repository = Arc::new(LogFileRepositoryImpl::new());
        let parser_manager = LogParserManager::new();
        Ok(Self {
            config,
            log_file_repository,
            character_service,
            server_monitoring_service,
            walkthrough_service,
            parser_manager,
            is_running: Arc::new(RwLock::new(false)),
            last_position: Arc::new(RwLock::new(0)),
            zone_level_cache: Arc::new(RwLock::new(None)),
        })
    }

    /// Creates a new LogAnalysisServiceImpl with custom repositories (for testing)
    pub fn with_repositories(
        config: LogAnalysisConfig,
        log_file_repository: Arc<dyn LogFileRepository>,
        character_service: Arc<dyn CharacterService>,
        server_monitoring_service: Arc<dyn ServerMonitoringService>,
        walkthrough_service: Arc<dyn WalkthroughService>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let parser_manager = LogParserManager::new();
        Self {
            config,
            log_file_repository,
            character_service,
            server_monitoring_service,
            walkthrough_service,
            parser_manager,
            is_running: Arc::new(RwLock::new(false)),
            last_position: Arc::new(RwLock::new(0)),
            zone_level_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Starts the main monitoring loop for log file analysis
    async fn start_monitoring_loop(&self) -> AppResult<()> {
        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        drop(config);

        if log_path.is_empty() {
            return Err(AppError::internal_error(
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
        let character_service = Arc::clone(&self.character_service);
        let server_monitoring_service = Arc::clone(&self.server_monitoring_service);
        let walkthrough_service = Arc::clone(&self.walkthrough_service);
        let is_running = Arc::clone(&self.is_running);
        let last_position = Arc::clone(&self.last_position);
        let zone_level_cache = Arc::clone(&self.zone_level_cache);
        let parser_manager = self.parser_manager.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(interval_ms));

            loop {
                interval.tick().await;

                // Check if monitoring should continue
                if !*is_running.read().await {
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
                                &character_service,
                                &server_monitoring_service,
                                &walkthrough_service,
                                &last_position,
                                &zone_level_cache,
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
        character_service: &Arc<dyn CharacterService>,
        server_monitoring_service: &Arc<dyn ServerMonitoringService>,
        walkthrough_service: &Arc<dyn WalkthroughService>,
        last_position: &Arc<RwLock<u64>>,
        zone_level_cache: &Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
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
                character_service,
                server_monitoring_service,
                walkthrough_service,
                zone_level_cache,
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

        Ok(())
    }

    /// Processes a single log line and handles any detected events
    async fn process_single_line(
        parser_manager: &LogParserManager,
        line: &str,
        character_service: &Arc<dyn CharacterService>,
        server_monitoring_service: &Arc<dyn ServerMonitoringService>,
        walkthrough_service: &Arc<dyn WalkthroughService>,
        zone_level_cache: &Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
    ) -> AppResult<()> {
        // Try to parse the line for known events
        if let Ok(Some(result)) = parser_manager.parse_line(line) {
            match result {
                crate::infrastructure::parsing::ParserResult::SceneChange(content) => {
                    // Process scene changes through character tracking service
                    let walkthrough_service = walkthrough_service.clone();
                    
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        
                        // Check for cached zone level
                        let cached_level = {
                            let cache = zone_level_cache.read().await;
                            cache.clone()
                        };

                        if let Some((level, _timestamp)) = cached_level {
                            // Clear the cache after use
                            {
                                let mut cache = zone_level_cache.write().await;
                                *cache = None;
                            }

                            // Process scene change with zone level
                            if let Err(e) = character_service
                                .process_scene_content_with_zone_level(
                                    &content,
                                    &active_character.id,
                                    level,
                                )
                                .await
                            {
                                error!("❌ SCENE CHANGE: Failed to process scene change with zone level: {}", e);
                            } else {
                                // Handle walkthrough progress detection
                                if let Err(e) = walkthrough_service
                                    .handle_scene_change(&active_character.id, &content)
                                    .await
                                {
                                    error!("❌ WALKTHROUGH: Failed to handle walkthrough scene change: {}", e);
                                }
                            }
                        } else {
                            // Process scene change without zone level
                            if let Err(e) = character_service
                                .process_scene_content(&content, &active_character.id)
                                .await
                            {
                                error!("❌ SCENE CHANGE: Failed to process scene change: {}", e);
                            } else {
                                // Handle walkthrough progress detection
                                if let Err(e) = walkthrough_service
                                    .handle_scene_change(&active_character.id, &content)
                                    .await
                                {
                                    error!("❌ WALKTHROUGH: Failed to handle walkthrough scene change: {}", e);
                                }
                            }
                        }
                    } else {
                    }
                }
                crate::infrastructure::parsing::ParserResult::ServerConnection(event) => {
                    // Handle server connection events
                    server_monitoring_service
                        .update_server_from_log(event.ip_address.clone(), event.port)
                        .await?;

                    // Server monitoring service will publish its own events
                }
                crate::infrastructure::parsing::ParserResult::CharacterLevel((
                    character_name,
                    class_or_ascendency,
                    new_level,
                )) => {
                    // Handle character level up events
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        // Check if the parsed class/ascendency matches the active character
                        let matches = match &class_or_ascendency {
                            crate::infrastructure::parsing::manager::CharacterClassOrAscendency::Class(class) => {
                                active_character.class == *class
                            }
                            crate::infrastructure::parsing::manager::CharacterClassOrAscendency::Ascendency(ascendency) => {
                                active_character.ascendency == *ascendency
                            }
                        };

                        if active_character.name == character_name && matches {
                            character_service
                                .update_character_level(&active_character.id, new_level)
                                .await?;

                            info!(
                                "Character level up: {} ({:?} -> {})",
                                character_name, class_or_ascendency, new_level
                            );
                        }
                    }
                }
                crate::infrastructure::parsing::ParserResult::CharacterDeath(character_name) => {
                    // Handle character death events - track deaths only in character_data.json
                    
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        
                        if active_character.name == character_name {

                            // Record death in the current zone via character service
                            match character_service.get_character(&active_character.id).await {
                                Ok(character_data) => {

                                    if let Some(active_zone) = character_data.get_active_zone() {

                                        if let Err(e) = character_service
                                            .record_death(
                                                &active_character.id,
                                                &active_zone.location_id,
                                            )
                                            .await
                                        {
                                            error!("❌ DEATH PROCESSING: Failed to record death in zone: {}", e);
                                        } else {
                                            info!(
                                                "Character death: {} in zone '{}'",
                                                character_name, active_zone.location_name
                                            );
                                        }
                                    } else {
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "❌ DEATH PROCESSING: Failed to load character data for death recording: '{}' - {}",
                                        character_name, e
                                    );
                                }
                            }

                        } else {
                        }
                    } else {
                    }
                }
                crate::infrastructure::parsing::ParserResult::ZoneLevel(level) => {
                    // Handle zone level events - cache the level for association with scene changes
                    let mut cache = zone_level_cache.write().await;
                    *cache = Some((level, chrono::Utc::now()));
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

        info!("Log monitoring stopped");
        Ok(())
    }

    /// Returns whether log monitoring is currently active
    async fn is_monitoring(&self) -> bool {
        *self.is_running.read().await
    }

    /// Updates the path to the log file being monitored
    async fn update_log_path(&self, new_path: String) -> AppResult<()> {
        let mut config = self.config.write().await;
        config.log_file_path = new_path;
        drop(config);
        Ok(())
    }
}
