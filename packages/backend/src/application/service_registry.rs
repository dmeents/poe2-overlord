use crate::domain::character::traits::CharacterService;
use crate::domain::configuration::{
    service::ConfigurationServiceImpl, sqlite_repository::ConfigurationSqliteRepository,
    traits::ConfigurationService,
};
use crate::domain::economy::EconomyService;
use crate::domain::game_monitoring::{
    traits::GameMonitoringService, GameMonitoringServiceImpl, ProcessDetectorImpl,
};
use crate::domain::log_analysis::{
    models::LogAnalysisConfig, service::LogAnalysisServiceImpl, traits::LogAnalysisService,
};
use crate::domain::server_monitoring::{
    ServerMonitoringService, ServerMonitoringServiceImpl, ServerStatusSqliteRepository,
    SystemPingProvider,
};
use crate::domain::walkthrough::{
    repository::WalkthroughRepositoryImpl, service::WalkthroughServiceImpl,
    traits::WalkthroughService,
};
use crate::domain::zone_configuration::{
    service::ZoneConfigurationServiceImpl, sqlite_repository::ZoneConfigurationSqliteRepository,
};
use crate::infrastructure::database::DatabasePool;
use crate::infrastructure::events::{EventBus, TauriEventBridge};
use crate::infrastructure::file_management::paths::AppPaths;
use log::{error, info};
use std::sync::Arc;
use tauri::Manager;

pub struct ServiceInitializer;

impl ServiceInitializer {
    /// Initializes services in dependency order and registers them with Tauri state
    pub fn initialize_services(
        app: &mut tauri::App,
    ) -> Result<ServiceInstances, Box<dyn std::error::Error>> {
        info!("Starting service initialization...");

        // Initialize database pool before any repositories
        let db_path = AppPaths::data_dir().join("poe2-overlord.db");
        tauri::async_runtime::block_on(AppPaths::ensure_data_dir())?;

        let database_pool = tauri::async_runtime::block_on(DatabasePool::new(&db_path))
            .map_err(|e| {
                error!("Failed to initialize database pool: {}", e);
                e
            })?;
        let pool = database_pool.pool().clone();
        info!("Database pool initialized at {:?}", db_path);

        let event_bus = Arc::new(EventBus::new());
        app.manage(event_bus.clone());

        // Create configuration repository (SQLite-based)
        let config_repository =
            Arc::new(ConfigurationSqliteRepository::new(pool.clone()))
                as Arc<dyn crate::domain::configuration::traits::ConfigurationRepository + Send + Sync>;

        // Create configuration service with DI
        let config_service_impl = tauri::async_runtime::block_on(async {
            let service =
                ConfigurationServiceImpl::new(config_repository.clone(), event_bus.clone());

            // Load config or use defaults
            match config_repository.load().await {
                Ok(_) => {
                    log::info!("Configuration loaded successfully");
                }
                Err(e) => {
                    log::warn!("Failed to load config, using defaults: {}", e);
                    let default_config = crate::domain::configuration::models::AppConfig::default();
                    if let Err(save_err) = config_repository.save(&default_config).await {
                        log::warn!("Failed to save default config: {}", save_err);
                    }
                }
            }

            service
        });

        let config_service =
            Arc::new(config_service_impl) as Arc<dyn ConfigurationService + Send + Sync>;
        app.manage(config_service.clone());

        let economy_service = EconomyService::new().map_err(|e| {
            error!("Failed to initialize EconomyService: {}", e);
            e
        })?;
        app.manage(economy_service);

        let zone_config_repo = Arc::new(ZoneConfigurationSqliteRepository::new(pool.clone()));
        let zone_config_service = Arc::new(ZoneConfigurationServiceImpl::new(zone_config_repo));
        app.manage(zone_config_service.clone());

        let wiki_service =
            Arc::new(crate::domain::wiki_scraping::service::WikiScrapingServiceImpl::new()?);
        app.manage(wiki_service.clone());

        let character_repo = Arc::new(crate::domain::character::CharacterSqliteRepository::new(
            pool.clone(),
        )) as Arc<dyn crate::domain::character::traits::CharacterRepository + Send + Sync>;

        let zone_tracking = Arc::new(crate::domain::zone_tracking::ZoneTrackingServiceImpl::new());

        let character_service = crate::domain::character::service::CharacterServiceImpl::new(
            character_repo,
            event_bus.clone(),
            zone_tracking,
            zone_config_service.clone(),
        );

        let character_arc = Arc::new(character_service) as Arc<dyn CharacterService + Send + Sync>;
        app.manage(character_arc.clone());

        let walkthrough_repo = Arc::new(WalkthroughRepositoryImpl::new(std::path::PathBuf::from(
            "config/walkthrough_guide.json",
        )));
        let walkthrough_service = Arc::new(WalkthroughServiceImpl::new(
            walkthrough_repo,
            character_arc.clone(),
            event_bus.clone(),
        )) as Arc<dyn WalkthroughService + Send + Sync>;
        app.manage(walkthrough_service.clone());

        let ping_provider = Arc::new(SystemPingProvider::new());
        let server_status_repository = ServerStatusSqliteRepository::new(pool.clone());
        let server_monitoring_service =
            tauri::async_runtime::block_on(ServerMonitoringServiceImpl::new(
                event_bus.clone(),
                ping_provider,
                Arc::new(server_status_repository),
            ))
            .map_err(|e| {
                error!("Failed to initialize ServerMonitoringService: {}", e);
                e
            })?;
        let server_monitoring_arc =
            Arc::new(server_monitoring_service) as Arc<dyn ServerMonitoringService>;
        app.manage(server_monitoring_arc.clone());

        let log_analysis_config = LogAnalysisConfig {
            log_file_path: String::new(),
            monitoring_interval_ms: 500,
            max_file_size_mb: 100,
            buffer_size: 1000,
            session_gap_threshold_minutes: 30,
        };

        let log_analysis_service = LogAnalysisServiceImpl::new(
            log_analysis_config,
            character_arc.clone(),
            server_monitoring_arc.clone(),
            walkthrough_service.clone(),
            zone_config_service.clone(),
            wiki_service.clone(),
            config_service.clone(),
            event_bus.clone(),
        )
        .map_err(|e| {
            error!("Failed to initialize LogAnalysisService: {}", e);
            e
        })?;

        let log_analysis_arc = Arc::new(log_analysis_service) as Arc<dyn LogAnalysisService>;

        let config_service_clone = config_service.clone();
        let log_analysis_clone = log_analysis_arc.clone();
        tauri::async_runtime::block_on(async move {
            match config_service_clone.get_config().await {
                Ok(config) => {
                    let log_path = config.poe_client_log_path;
                    if !log_path.is_empty() {
                        if let Err(e) = log_analysis_clone.update_log_path(log_path.clone()).await {
                            error!("Failed to update log path in LogAnalysisService: {}", e);
                        } else {
                            info!("Log analysis service configured with path: {}", log_path);
                        }
                    } else {
                        info!("No POE client log path configured, log monitoring will use default");
                    }
                }
                Err(e) => {
                    error!("Failed to get config from configuration service: {}", e);
                }
            }
        });

        app.manage(log_analysis_arc.clone());

        let process_detector = Arc::new(ProcessDetectorImpl::new());

        let game_monitoring_service = Arc::new(GameMonitoringServiceImpl::new(
            event_bus.clone(),
            process_detector.clone(),
            character_arc.clone(),
        )) as Arc<dyn GameMonitoringService>;

        app.manage(game_monitoring_service.clone());

        info!("Service initialization completed successfully");

        Ok(ServiceInstances {
            config_service,
            event_bus,
            character_service: character_arc,
            walkthrough_service,
            log_analysis_service: log_analysis_arc,
            server_monitoring_service: server_monitoring_arc,
            game_monitoring_service,
            event_bridge: None,
        })
    }
}

#[derive(Clone)]
pub struct ServiceInstances {
    pub config_service: Arc<dyn ConfigurationService + Send + Sync>,
    pub event_bus: Arc<EventBus>,
    pub character_service: Arc<dyn CharacterService + Send + Sync>,
    pub walkthrough_service: Arc<dyn WalkthroughService + Send + Sync>,
    pub log_analysis_service: Arc<dyn LogAnalysisService>,
    pub server_monitoring_service: Arc<dyn ServerMonitoringService>,
    pub game_monitoring_service: Arc<dyn GameMonitoringService>,
    pub event_bridge: Option<Arc<TauriEventBridge>>,
}

impl ServiceInstances {
    pub fn set_event_bridge(&mut self, bridge: Arc<TauriEventBridge>) {
        self.event_bridge = Some(bridge);
    }

    pub async fn shutdown_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting application shutdown cleanup...");

        // Stop event bridge forwarding
        if let Some(bridge) = &self.event_bridge {
            if let Err(e) = bridge.stop_forwarding().await {
                log::error!("Failed to stop event bridge: {}", e);
            } else {
                log::info!("Event bridge stopped successfully");
            }
        }

        // Stop monitoring services
        if let Err(e) = self.game_monitoring_service.stop_monitoring().await {
            log::error!("Failed to stop game monitoring: {}", e);
        } else {
            log::info!("Game monitoring stopped successfully");
        }

        if let Err(e) = self.log_analysis_service.stop_monitoring().await {
            log::error!("Failed to stop log monitoring: {}", e);
        } else {
            log::info!("Log monitoring stopped successfully");
        }

        if let Err(e) = self.server_monitoring_service.stop_ping_monitoring().await {
            log::error!("Failed to stop server monitoring: {}", e);
        } else {
            log::info!("Server monitoring stopped successfully");
        }

        // Finalize character data
        if let Err(e) = self.character_service.finalize_all_active_zones().await {
            log::error!("Failed to finalize character tracking data: {}", e);
        } else {
            log::info!("Character tracking data finalized successfully");
        }

        // Emit system shutdown event
        if let Err(e) = self
            .event_bus
            .publish(crate::infrastructure::events::AppEvent::system_shutdown())
            .await
        {
            log::error!("Failed to publish system shutdown event: {}", e);
        }

        log::info!("Background services shutdown completed");
        log::info!("Application shutdown cleanup completed");
        Ok(())
    }
}
