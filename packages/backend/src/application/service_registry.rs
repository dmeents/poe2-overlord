use crate::domain::character::traits::CharacterService;
use crate::domain::configuration::{
    service::ConfigurationServiceImpl, traits::ConfigurationService,
};
use crate::domain::economy::EconomyService;
use crate::domain::game_monitoring::{
    traits::GameMonitoringService, GameMonitoringServiceImpl, ProcessDetectorImpl,
};
use crate::domain::log_analysis::{
    models::LogAnalysisConfig, service::LogAnalysisServiceImpl, traits::LogAnalysisService,
};
use crate::domain::server_monitoring::{
    ServerMonitoringService, ServerMonitoringServiceImpl, ServerStatusRepositoryImpl,
    SystemPingProvider,
};
use crate::domain::walkthrough::{
    repository::WalkthroughRepositoryImpl, service::WalkthroughServiceImpl,
    traits::WalkthroughService,
};
use crate::domain::zone_configuration::{
    repository::ZoneConfigurationRepositoryImpl, service::ZoneConfigurationServiceImpl,
};
use crate::infrastructure::events::EventBus;
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

        let config_service = Arc::new(
            tauri::async_runtime::block_on(ConfigurationServiceImpl::new())
                .expect("Failed to create configuration service"),
        );
        app.manage(config_service.clone());

        let event_bus = Arc::new(EventBus::new());
        app.manage(event_bus.clone());

        let economy_service = EconomyService::new();
        app.manage(economy_service);

        let zone_config_repo = Arc::new(tauri::async_runtime::block_on(
            ZoneConfigurationRepositoryImpl::new(),
        )?);
        let zone_config_service = Arc::new(ZoneConfigurationServiceImpl::new(zone_config_repo));
        app.manage(zone_config_service.clone());

        let wiki_service =
            Arc::new(crate::domain::wiki_scraping::service::WikiScrapingServiceImpl::new()?);
        app.manage(wiki_service.clone());

        let character_service = tauri::async_runtime::block_on(
            crate::domain::character::service::CharacterServiceImpl::with_default_repository(
                event_bus.clone(),
                zone_config_service.clone(),
            ),
        )
        .map_err(|e| {
            error!("Failed to initialize CharacterService: {}", e);
            e
        })?;

        let character_arc = Arc::new(character_service) as Arc<dyn CharacterService + Send + Sync>;

        let character_box = Box::new(
            tauri::async_runtime::block_on(
                crate::domain::character::service::CharacterServiceImpl::with_default_repository(
                    event_bus.clone(),
                    zone_config_service.clone(),
                ),
            )
            .map_err(|e| {
                error!(
                    "Failed to initialize CharacterService for state management: {}",
                    e
                );
                e
            })?,
        ) as Box<dyn CharacterService + Send + Sync>;

        app.manage(character_box);

        let walkthrough_repo = Arc::new(WalkthroughRepositoryImpl::new(std::path::PathBuf::from(
            "config/walkthrough_guide.json",
        )));
        let walkthrough_service = Arc::new(WalkthroughServiceImpl::new(
            walkthrough_repo,
            character_arc.clone(),
            event_bus.clone(),
        ));

        let walkthrough_box = Box::new(WalkthroughServiceImpl::new(
            Arc::new(WalkthroughRepositoryImpl::new(std::path::PathBuf::from(
                "config/walkthrough_guide.json",
            ))),
            character_arc.clone(),
            event_bus.clone(),
        )) as Box<dyn WalkthroughService + Send + Sync>;

        app.manage(walkthrough_box);

        let ping_provider = Arc::new(SystemPingProvider::new());
        let server_status_repository =
            tauri::async_runtime::block_on(ServerStatusRepositoryImpl::new()).map_err(|e| {
                error!("Failed to initialize ServerStatusRepository: {}", e);
                e
            })?;
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
            match config_service_clone.get_poe_client_log_path().await {
                Ok(log_path) => {
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
                    error!("Failed to get log path from configuration service: {}", e);
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
        })
    }
}

#[derive(Clone)]
pub struct ServiceInstances {
    pub config_service: Arc<ConfigurationServiceImpl>,
    pub event_bus: Arc<EventBus>,
    pub character_service: Arc<dyn CharacterService + Send + Sync>,
    pub walkthrough_service: Arc<dyn WalkthroughService + Send + Sync>,
    pub log_analysis_service: Arc<dyn LogAnalysisService>,
    pub server_monitoring_service: Arc<dyn ServerMonitoringService>,
    pub game_monitoring_service: Arc<dyn GameMonitoringService>,
}

impl ServiceInstances {
    pub async fn shutdown_services(&self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Starting application shutdown cleanup...");

        if let Err(e) = self.character_service.finalize_all_active_zones().await {
            log::error!("Failed to finalize character tracking data: {}", e);
        } else {
            log::info!("Character tracking data finalized successfully");
        }

        log::info!("Background services shutdown completed");

        log::info!("Application shutdown cleanup completed");
        Ok(())
    }
}
