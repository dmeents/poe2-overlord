use crate::domain::character::traits::CharacterService;
use crate::domain::log_analysis::models::LogAnalysisConfig;
use crate::domain::log_analysis::repository::LogFileRepositoryImpl;
use crate::domain::log_analysis::traits::{LogAnalysisService, LogFileRepository};
use crate::domain::server_monitoring::ServerMonitoringService;
use crate::domain::walkthrough::traits::WalkthroughService;
use crate::errors::{AppError, AppResult};
use crate::infrastructure::expand_tilde;
use crate::infrastructure::parsing::LogParserManager;
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time;

pub struct LogAnalysisServiceImpl {
    config: Arc<RwLock<LogAnalysisConfig>>,
    log_file_repository: Arc<dyn LogFileRepository>,
    character_service: Arc<dyn CharacterService>,
    server_monitoring_service: Arc<dyn ServerMonitoringService>,
    walkthrough_service: Arc<dyn WalkthroughService>,
    zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    wiki_service: Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
    config_service: Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
    event_bus: Arc<crate::infrastructure::events::EventBus>,
    parser_manager: LogParserManager,
    is_running: Arc<RwLock<bool>>,
    last_position: Arc<RwLock<u64>>,
    zone_level_cache: Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
}

impl LogAnalysisServiceImpl {
    pub fn new(
        config: LogAnalysisConfig,
        character_service: Arc<dyn CharacterService>,
        server_monitoring_service: Arc<dyn ServerMonitoringService>,
        walkthrough_service: Arc<dyn WalkthroughService>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
        event_bus: Arc<crate::infrastructure::events::EventBus>,
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
            zone_config,
            wiki_service,
            config_service,
            event_bus,
            parser_manager,
            is_running: Arc::new(RwLock::new(false)),
            last_position: Arc::new(RwLock::new(0)),
            zone_level_cache: Arc::new(RwLock::new(None)),
        })
    }

    pub fn with_repositories(
        config: LogAnalysisConfig,
        log_file_repository: Arc<dyn LogFileRepository>,
        character_service: Arc<dyn CharacterService>,
        server_monitoring_service: Arc<dyn ServerMonitoringService>,
        walkthrough_service: Arc<dyn WalkthroughService>,
        zone_config: Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
        event_bus: Arc<crate::infrastructure::events::EventBus>,
    ) -> Self {
        let config = Arc::new(RwLock::new(config));
        let parser_manager = LogParserManager::new();
        Self {
            config,
            log_file_repository,
            character_service,
            server_monitoring_service,
            walkthrough_service,
            zone_config,
            wiki_service,
            config_service,
            event_bus,
            parser_manager,
            is_running: Arc::new(RwLock::new(false)),
            last_position: Arc::new(RwLock::new(0)),
            zone_level_cache: Arc::new(RwLock::new(None)),
        }
    }

    async fn start_monitoring_loop(&self) -> AppResult<()> {
        info!("LOG ANALYSIS: start_monitoring_loop() entered");

        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        drop(config);

        info!("LOG ANALYSIS: Log path from config: '{}'", log_path);

        if log_path.is_empty() {
            error!("LOG ANALYSIS: Log file path is empty!");
            return Err(AppError::internal_error(
                "start_monitoring_loop",
                "Log file path not configured",
            ));
        }

        info!("LOG ANALYSIS: Checking if log file exists...");
        let file_exists = self.log_file_repository.file_exists(&log_path).await;
        info!("LOG ANALYSIS: File exists check result: {}", file_exists);

        if !file_exists {
            warn!("Log file does not exist at path: {}", log_path);
            return Err(AppError::file_system_error(
                "start_monitoring_loop",
                &format!("Log file does not exist: {}", log_path),
            ));
        }

        let file_size = self.log_file_repository.get_file_size(&log_path).await?;
        {
            let mut last_pos = self.last_position.write().await;
            *last_pos = file_size;
        }

        info!("Starting log file monitoring for: {}", log_path);
        info!(
            "LOG ANALYSIS: Monitoring initialized - starting from position: {}",
            file_size
        );

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
        let character_service = Arc::clone(&self.character_service);
        let server_monitoring_service = Arc::clone(&self.server_monitoring_service);
        let walkthrough_service = Arc::clone(&self.walkthrough_service);
        let zone_config = Arc::clone(&self.zone_config);
        let wiki_service = Arc::clone(&self.wiki_service);
        let config_service = Arc::clone(&self.config_service);
        let event_bus = Arc::clone(&self.event_bus);
        let is_running = Arc::clone(&self.is_running);
        let last_position = Arc::clone(&self.last_position);
        let zone_level_cache = Arc::clone(&self.zone_level_cache);
        let parser_manager = self.parser_manager.clone();

        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_millis(interval_ms));

            loop {
                interval.tick().await;

                if !*is_running.read().await {
                    info!("LOG ANALYSIS: Monitoring stopped - is_running is false");
                    break;
                }

                match log_file_repository.get_file_size(&log_path).await {
                    Ok(current_size) => {
                        let last_pos = *last_position.read().await;
                        if current_size > last_pos {
                            info!(
                                "LOG ANALYSIS: New content detected - file grew from {} to {} bytes",
                                last_pos, current_size
                            );
                            if let Err(e) = Self::process_new_lines(
                                &log_path,
                                &log_file_repository,
                                &parser_manager,
                                &character_service,
                                &server_monitoring_service,
                                &walkthrough_service,
                                &zone_config,
                                &wiki_service,
                                &config_service,
                                &event_bus,
                                &last_position,
                                &zone_level_cache,
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
        parser_manager: &LogParserManager,
        character_service: &Arc<dyn CharacterService>,
        server_monitoring_service: &Arc<dyn ServerMonitoringService>,
        walkthrough_service: &Arc<dyn WalkthroughService>,
        zone_config: &Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: &Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: &Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
        event_bus: &Arc<crate::infrastructure::events::EventBus>,
        last_position: &Arc<RwLock<u64>>,
        zone_level_cache: &Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
        start_position: u64,
    ) -> AppResult<()> {
        let new_lines = log_file_repository
            .read_from_position(log_path, start_position)
            .await?;

        if new_lines.is_empty() {
            return Ok(());
        }

        info!(
            "LOG ANALYSIS: Processing {} new lines from log file",
            new_lines.len()
        );

        for line in &new_lines {
            if let Err(e) = Self::process_single_line(
                parser_manager,
                line,
                character_service,
                server_monitoring_service,
                walkthrough_service,
                zone_config,
                wiki_service,
                config_service,
                event_bus,
                zone_level_cache,
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

        Ok(())
    }

    async fn trigger_wiki_fetch(
        zone_name: &str,
        wiki_service: &Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        zone_config: &Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
    ) {
        info!("Triggering wiki fetch for zone: {}", zone_name);
        let wiki_service = wiki_service.clone();
        let zone_config = zone_config.clone();
        let zone_name = zone_name.to_string();

        tokio::spawn(async move {
            info!("Starting wiki fetch for zone: {}", zone_name);
            match wiki_service.fetch_zone_data(&zone_name).await {
                Ok(wiki_data) => {
                    info!(
                        "Successfully fetched wiki data for zone '{}': act={}, level={:?}, town={}",
                        zone_name, wiki_data.act, wiki_data.area_level, wiki_data.is_town
                    );

                    let _area_id = wiki_data
                        .area_id
                        .as_ref()
                        .map(|id| id.clone())
                        .unwrap_or_else(|| {
                            zone_name
                                .to_lowercase()
                                .replace(' ', "_")
                                .replace('-', "_")
                                .chars()
                                .filter(|c| c.is_alphanumeric() || *c == '_')
                                .collect::<String>()
                                .trim_matches('_')
                                .to_string()
                        });

                    info!("Reloading zone configuration before lookup...");
                    if let Err(e) = zone_config.reload_configuration().await {
                        error!("Failed to reload zone configuration: {}", e);
                    }

                    info!("Looking up zone '{}' in configuration...", zone_name);
                    if let Some(zone_metadata) = zone_config.get_zone_metadata(&zone_name).await {
                        info!(
                            "Found zone '{}' in configuration, updating with wiki data",
                            zone_name
                        );
                        info!(
                            "BEFORE UPDATE: area_id={:?}, act={}, is_town={}",
                            zone_metadata.area_id, zone_metadata.act, zone_metadata.is_town
                        );

                        let mut updated_metadata = zone_metadata;
                        updated_metadata.update_from_wiki_data(&wiki_data);

                        info!(
                            "AFTER UPDATE: area_id={:?}, act={}, is_town={}",
                            updated_metadata.area_id,
                            updated_metadata.act,
                            updated_metadata.is_town
                        );

                        if let Err(e) = zone_config.update_zone(updated_metadata).await {
                            error!("Failed to update zone metadata: {}", e);
                        } else {
                            info!("Successfully updated zone metadata for '{}'", zone_name);
                        }
                    } else {
                        warn!(
                            "Zone '{}' not found in configuration after reload",
                            zone_name
                        );
                    }
                }
                Err(e) => {
                    error!("Failed to fetch wiki data for zone '{}': {}", zone_name, e);
                }
            }
        });
    }

    fn is_act_name(scene_name: &str) -> bool {
        let lower_name = scene_name.to_lowercase();

        let act_names = ["act 1", "act 2", "act 3", "act 4", "interlude", "endgame"];

        if act_names.iter().any(|act| lower_name == *act) {
            return true;
        }

        let act_keywords = ["act", "endgame", "interlude", "atlas"];
        act_keywords
            .iter()
            .any(|keyword| lower_name.contains(keyword))
    }

    async fn process_scene_change(
        character_service: &Arc<dyn CharacterService>,
        zone_config: &Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: &Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: &Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
        event_bus: &Arc<crate::infrastructure::events::EventBus>,
        content: &str,
        character_id: &str,
        _zone_level: Option<u32>,
    ) -> Result<Option<crate::domain::log_analysis::models::SceneChangeEvent>, AppError> {
        let zone_name = content.trim();

        if zone_name.is_empty() {
            return Ok(None);
        }

        if Self::is_act_name(zone_name) {
            debug!(
                "SCENE FILTER: Filtering out act name '{}' - not tracking as zone",
                zone_name
            );
            return Ok(None);
        }

        let zone_metadata = if let Some(metadata) = zone_config.get_zone_metadata(zone_name).await {
            metadata
        } else {
            let mut placeholder =
                crate::domain::zone_configuration::models::ZoneMetadata::placeholder(
                    zone_name.to_string(),
                );

            placeholder.act = 0;

            if let Err(e) = zone_config.add_zone(placeholder.clone()).await {
                debug!("Failed to add placeholder zone '{}': {}", zone_name, e);
            }

            Self::trigger_wiki_fetch(zone_name, wiki_service, zone_config).await;

            placeholder
        };

        let refresh_interval = config_service
            .get_zone_refresh_interval()
            .await
            .unwrap_or_default()
            .to_seconds();

        if zone_metadata.needs_refresh(refresh_interval) {
            Self::trigger_wiki_fetch(zone_name, wiki_service, zone_config).await;
        }

        if let Err(e) = character_service.enter_zone(character_id, zone_name).await {
            error!("Failed to enter zone '{}': {}", zone_name, e);
            return Err(e);
        }

        let character_data = match character_service.load_character_data(character_id).await {
            Ok(data) => data,
            Err(e) => {
                error!("Failed to load character data: {}", e);
                return Err(e);
            }
        };

        let mut updated_character_data = character_data.clone();
        updated_character_data.timestamps.last_played = Some(chrono::Utc::now());
        updated_character_data.touch();

        if let Err(e) = character_service
            .save_character_data(&updated_character_data)
            .await
        {
            error!("Failed to save character data: {}", e);
            return Err(e);
        }

        let scene_change_event = crate::domain::log_analysis::models::SceneChangeEvent::Zone(
            crate::domain::log_analysis::models::ZoneChangeEvent {
                zone_name: zone_name.to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        );

        let event = crate::infrastructure::events::AppEvent::character_tracking_data_updated(
            character_id.to_string(),
            updated_character_data,
        );
        if let Err(e) = event_bus.publish(event).await {
            warn!(
                "SCENE CHANGE: Failed to publish character tracking data updated event: {}",
                e
            );
        }

        let zone_metadata = zone_config.get_zone_metadata(zone_name).await;
        let act_info = zone_metadata
            .as_ref()
            .map(|z| z.act.to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        let is_town_info = zone_metadata.map(|z| z.is_town).unwrap_or(false);

        info!(
            "Scene change: character {} entered '{}' (Act: {}, Town: {})",
            character_id, zone_name, act_info, is_town_info
        );

        Ok(Some(scene_change_event))
    }

    async fn process_scene_change_with_error_handling(
        character_service: &Arc<dyn CharacterService>,
        walkthrough_service: &Arc<dyn WalkthroughService>,
        zone_config: &Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: &Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: &Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
        event_bus: &Arc<crate::infrastructure::events::EventBus>,
        content: &str,
        character_id: &str,
        zone_level: Option<u32>,
    ) {
        let result = Self::process_scene_change(
            character_service,
            zone_config,
            wiki_service,
            config_service,
            event_bus,
            content,
            character_id,
            zone_level,
        )
        .await;

        if let Err(e) = result {
            error!("SCENE CHANGE: Failed to process scene change: {}", e);
            return;
        }

        if let Err(e) = walkthrough_service
            .handle_scene_change(character_id, content)
            .await
        {
            error!(
                "WALKTHROUGH: Failed to handle walkthrough scene change: {}",
                e
            );
        }
    }

    async fn process_character_death_with_error_handling(
        character_service: &Arc<dyn CharacterService>,
        character_name: &str,
        character_id: &str,
    ) {
        match character_service.get_character(character_id).await {
            Ok(character_data) => {
                if let Some(current_location) = &character_data.current_location {
                    if let Err(e) = character_service.record_death(character_id).await {
                        error!("DEATH PROCESSING: Failed to record death in zone: {}", e);
                    } else {
                        info!(
                            "Character death: {} in zone '{}'",
                            character_name, current_location.zone_name
                        );
                    }
                }
            }
            Err(e) => {
                error!(
                    "DEATH PROCESSING: Failed to load character data for death recording: '{}' - {}",
                    character_name, e
                );
            }
        }
    }

    async fn process_single_line(
        parser_manager: &LogParserManager,
        line: &str,
        character_service: &Arc<dyn CharacterService>,
        server_monitoring_service: &Arc<dyn ServerMonitoringService>,
        walkthrough_service: &Arc<dyn WalkthroughService>,
        zone_config: &Arc<dyn crate::domain::zone_configuration::traits::ZoneConfigurationService>,
        wiki_service: &Arc<dyn crate::domain::wiki_scraping::traits::WikiScrapingService>,
        config_service: &Arc<dyn crate::domain::configuration::traits::ConfigurationService>,
        event_bus: &Arc<crate::infrastructure::events::EventBus>,
        zone_level_cache: &Arc<RwLock<Option<(u32, chrono::DateTime<chrono::Utc>)>>>,
    ) -> AppResult<()> {
        if line.contains("[SCENE]") {
            info!("LOG ANALYSIS: Processing line with [SCENE]: {}", line);
        }

        if line.contains("[INFO") && line.contains("[SCENE]") {
            info!("LOG ANALYSIS: Found [INFO] + [SCENE] line: {}", line);
        }

        let parse_result = parser_manager.parse_line(line);
        if let Err(e) = &parse_result {
            if line.contains("[SCENE]") {
                warn!(
                    "LOG ANALYSIS: Failed to parse SCENE line: {} - Error: {:?}",
                    line, e
                );
            }
        }

        if let Ok(Some(result)) = parse_result {
            match result {
                crate::infrastructure::parsing::ParserResult::SceneChange(content) => {
                    info!(
                        "LOG ANALYSIS: Scene change detected - content: '{}'",
                        content
                    );

                    let active_character_result = character_service.get_active_character().await;

                    if let Err(e) = &active_character_result {
                        warn!(
                            "LOG ANALYSIS: Failed to get active character for scene change: {}",
                            e
                        );
                    }

                    if let Ok(Some(active_character)) = active_character_result {
                        info!(
                            "LOG ANALYSIS: Processing scene change for active character: {}",
                            active_character.id
                        );
                        let cached_level = {
                            let cache = zone_level_cache.read().await;
                            cache.clone()
                        };

                        let zone_level = if let Some((level, _timestamp)) = cached_level {
                            // Clear the cache after use
                            {
                                let mut cache = zone_level_cache.write().await;
                                *cache = None;
                            }
                            Some(level)
                        } else {
                            None
                        };

                        Self::process_scene_change_with_error_handling(
                            character_service,
                            walkthrough_service,
                            zone_config,
                            wiki_service,
                            config_service,
                            event_bus,
                            &content,
                            &active_character.id,
                            zone_level,
                        )
                        .await;
                    } else {
                        warn!("LOG ANALYSIS: No active character found for scene change");
                    }
                }
                crate::infrastructure::parsing::ParserResult::ServerConnection(event) => {
                    server_monitoring_service
                        .update_server_from_log(event.ip_address.clone(), event.port)
                        .await?;
                }
                crate::infrastructure::parsing::ParserResult::CharacterLevel((
                    character_name,
                    new_level,
                )) => {
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        if active_character.name == character_name {
                            character_service
                                .update_character_level(&active_character.id, new_level)
                                .await?;

                            info!("Character level up: {} -> {}", character_name, new_level);
                        }
                    }
                }
                crate::infrastructure::parsing::ParserResult::CharacterDeath(character_name) => {
                    if let Ok(Some(active_character)) =
                        character_service.get_active_character().await
                    {
                        if active_character.name == character_name {
                            Self::process_character_death_with_error_handling(
                                character_service,
                                &character_name,
                                &active_character.id,
                            )
                            .await;
                        }
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
        info!("LOG ANALYSIS: start_monitoring() called");

        let config = self.config.read().await;
        let log_path = config.log_file_path.clone();
        drop(config);

        info!("LOG ANALYSIS: Configured log path: '{}'", log_path);

        let mut is_running = self.is_running.write().await;
        if *is_running {
            warn!("Log monitoring is already running");
            return Ok(());
        }

        *is_running = true;
        drop(is_running);

        info!("LOG ANALYSIS: Calling start_monitoring_loop()");
        let result = self.start_monitoring_loop().await;

        if let Err(e) = &result {
            error!("LOG ANALYSIS: start_monitoring_loop() failed: {}", e);
        } else {
            info!("LOG ANALYSIS: start_monitoring_loop() completed successfully");
        }

        result
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
        // Expand tilde (~) in the path to handle home directory references
        let expanded_path = expand_tilde(&new_path);
        let expanded_path_str = expanded_path.to_string_lossy().to_string();

        let mut config = self.config.write().await;
        config.log_file_path = expanded_path_str;
        drop(config);
        Ok(())
    }
}
