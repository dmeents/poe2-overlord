// CharacterServiceTrait is no longer needed - using CharacterService directly
use crate::domain::character::traits::CharacterService;
use crate::domain::log_analysis::models::LogAnalysisConfig;
use crate::domain::log_analysis::repository::LogFileRepositoryImpl;
use crate::domain::log_analysis::traits::{LogAnalysisService, LogFileRepository};
use crate::domain::server_monitoring::ServerMonitoringService;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::parsing::LogParserManager;
use async_trait::async_trait;
use log::{debug, error, info, warn};
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
    ) -> AppResult<Self> {
        let config = Arc::new(RwLock::new(config));
        let log_file_repository = Arc::new(LogFileRepositoryImpl::new());
        let parser_manager = LogParserManager::new();
        Ok(Self {
            config,
            log_file_repository,
            character_service,
            server_monitoring_service,
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
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let parser_manager = LogParserManager::new();
        Self {
            config,
            log_file_repository,
            character_service,
            server_monitoring_service,
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
                                &character_service,
                                &server_monitoring_service,
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
        zone_level_cache: &Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
    ) -> AppResult<()> {
        // Try to parse the line for known events
        if let Ok(Some(result)) = parser_manager.parse_line(line) {
            match result {
                crate::infrastructure::parsing::ParserResult::SceneChange(content) => {
                    // Process scene changes through character tracking service
                    debug!("🔍 SCENE CHANGE: Scene change detected: '{}'", content);
                    
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        debug!("🔍 SCENE CHANGE: Active character found: '{}' (ID: {})", 
                               active_character.name, active_character.id);
                        
                        // Check for cached zone level
                        let cached_level = {
                            let cache = zone_level_cache.read().await;
                            cache.clone()
                        };

                        if let Some((level, _timestamp)) = cached_level {
                            debug!("🔍 SCENE CHANGE: Using cached zone level {} for scene change", level);
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
                                debug!(
                                    "✅ SCENE CHANGE: Scene change with zone level {} processed for character {}: '{}'",
                                    level, active_character.id, content
                                );
                            }
                        } else {
                            debug!("🔍 SCENE CHANGE: No cached zone level, processing without level");
                            // Process scene change without zone level
                            if let Err(e) = character_service
                                .process_scene_content(&content, &active_character.id)
                                .await
                            {
                                error!("❌ SCENE CHANGE: Failed to process scene change: {}", e);
                            } else {
                                debug!(
                                    "✅ SCENE CHANGE: Scene change processed for character {}: '{}'",
                                    active_character.id, content
                                );
                            }
                        }
                        debug!("✅ SCENE CHANGE: Scene change processing completed for character '{}'", active_character.name);
                    } else {
                        debug!("❌ SCENE CHANGE: Scene change detected but no active character: '{}'", content);
                    }
                }
                crate::infrastructure::parsing::ParserResult::ServerConnection(event) => {
                    // Handle server connection events
                    server_monitoring_service
                        .update_server_from_log(event.ip_address.clone(), event.port)
                        .await?;

                    // Server monitoring service will publish its own events
                    debug!(
                        "Server connection detected in log: {}:{}",
                        event.ip_address, event.port
                    );
                }
                crate::infrastructure::parsing::ParserResult::CharacterLevel((
                    character_name,
                    character_class,
                    new_level,
                )) => {
                    // Handle character level up events
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        if active_character.name == character_name
                            && active_character.class == character_class
                        {
                            character_service
                                .update_character_level(&active_character.id, new_level)
                                .await?;

                            debug!(
                                "Character level up detected in log: {} ({} -> {})",
                                character_name, character_class, new_level
                            );
                        }
                    }
                }
                crate::infrastructure::parsing::ParserResult::CharacterDeath(character_name) => {
                    // Handle character death events - track deaths only in character_data.json
                    debug!("🔍 DEATH PROCESSING: Starting death processing for character: '{}'", character_name);
                    
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        debug!("🔍 DEATH PROCESSING: Active character found: '{}' (ID: {})", 
                               active_character.name, active_character.id);
                        
                        if active_character.name == character_name {
                            debug!(
                                "✅ DEATH PROCESSING: Character name matches! Processing death for: {} (ID: {})",
                                character_name, active_character.id
                            );

                            // Record death in the current zone via character service
                            match character_service.get_character(&active_character.id).await {
                                Ok(character_data) => {
                                    debug!(
                                        "✅ DEATH PROCESSING: Character data loaded successfully for '{}'",
                                        character_name
                                    );
                                    debug!("🔍 DEATH PROCESSING: Character has {} zones total", character_data.zones.len());

                                    if let Some(active_zone) = character_data.get_active_zone() {
                                        debug!(
                                            "✅ DEATH PROCESSING: Active zone found: '{}' (ID: '{}')",
                                            active_zone.location_name, active_zone.location_id
                                        );

                                        if let Err(e) = character_service
                                            .record_death(
                                                &active_character.id,
                                                &active_zone.location_id,
                                            )
                                            .await
                                        {
                                            error!("❌ DEATH PROCESSING: Failed to record death in zone: {}", e);
                                        } else {
                                            debug!(
                                                "✅ DEATH PROCESSING: Death successfully recorded in zone '{}' for character '{}'",
                                                active_zone.location_name, character_name
                                            );
                                        }
                                    } else {
                                        debug!(
                                            "❌ DEATH PROCESSING: Character death detected but no active zone found for: '{}'",
                                            character_name
                                        );
                                        debug!(
                                            "🔍 DEATH PROCESSING: Available zones: {:?}",
                                            character_data
                                                .zones
                                                .iter()
                                                .map(|z| (z.location_name.clone(), z.is_active, z.location_id.clone()))
                                                .collect::<Vec<_>>()
                                        );
                                        debug!("🔍 DEATH PROCESSING: Current location: {:?}", character_data.current_location);
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "❌ DEATH PROCESSING: Failed to load character data for death recording: '{}' - {}",
                                        character_name, e
                                    );
                                }
                            }

                            debug!("✅ DEATH PROCESSING: Death processing completed for character: '{}'", character_name);
                        } else {
                            debug!("❌ DEATH PROCESSING: Character name mismatch! Active: '{}', Death: '{}'", 
                                   active_character.name, character_name);
                        }
                    } else {
                        debug!("❌ DEATH PROCESSING: No active character found when processing death for: '{}'", character_name);
                    }
                }
                crate::infrastructure::parsing::ParserResult::ZoneLevel(level) => {
                    // Handle zone level events - cache the level for association with scene changes
                    debug!("Zone level detected: {}", level);
                    let mut cache = zone_level_cache.write().await;
                    *cache = Some((level, chrono::Utc::now()));
                    debug!("Zone level cached: {}", level);
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
