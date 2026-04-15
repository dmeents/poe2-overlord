use crate::domain::configuration::models::{
    AppConfig, ConfigurationValidationResult, ZoneRefreshInterval,
};
use crate::domain::configuration::traits::ConfigurationRepository;
use crate::errors::AppResult;
use crate::infrastructure::PathValidator;
use async_trait::async_trait;
use chrono::Utc;
use log::debug;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration repository implementation using `SQLite`.
///
/// Stores configuration in a single-row `app_config` table. Unlike the JSON-based
/// implementation, this eliminates the debounce mechanism since `SQLite` writes are
/// fast enough (< 1ms for single-row updates).
///
/// The in-memory cache is kept for fast reads without database queries.
#[derive(Clone)]
pub struct ConfigurationRepositoryImpl {
    pool: SqlitePool,
    /// In-memory cache for fast reads
    config: Arc<RwLock<AppConfig>>,
}

impl ConfigurationRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            config: Arc::new(RwLock::new(AppConfig::default())),
        }
    }
}

#[async_trait]
impl ConfigurationRepository for ConfigurationRepositoryImpl {
    async fn save(&self, config: &AppConfig) -> AppResult<()> {
        debug!("Saving configuration to SQLite");

        // Update in-memory cache first
        {
            let mut cache = self.config.write().await;
            *cache = config.clone();
        }

        // Convert enum to TEXT for storage
        let zone_refresh_interval = format!("{:?}", config.zone_refresh_interval);
        let updated_at = Utc::now().to_rfc3339();

        // INSERT OR REPLACE into single-row table
        sqlx::query(
            "INSERT OR REPLACE INTO app_config
             (id, config_version, poe_client_log_path, log_level, zone_refresh_interval, updated_at,
              hide_optional_objectives, hide_league_start_objectives, hide_flavor_text,
              hide_objective_descriptions, ui_zoom_level)
             VALUES (1, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(i64::from(config.config_version))
        .bind(&config.poe_client_log_path)
        .bind(&config.log_level)
        .bind(&zone_refresh_interval)
        .bind(&updated_at)
        .bind(i64::from(config.hide_optional_objectives))
        .bind(i64::from(config.hide_league_start_objectives))
        .bind(i64::from(config.hide_flavor_text))
        .bind(i64::from(config.hide_objective_descriptions))
        .bind(config.ui_zoom_level)
        .execute(&self.pool)
        .await?;

        debug!("Configuration saved to SQLite");
        Ok(())
    }

    async fn load(&self) -> AppResult<AppConfig> {
        debug!("Loading configuration from SQLite");

        // Query the single-row config table
        let row: Option<(i64, String, String, String, i64, i64, i64, i64, f64)> = sqlx::query_as(
            "SELECT config_version, poe_client_log_path, log_level, zone_refresh_interval,
                    hide_optional_objectives, hide_league_start_objectives,
                    hide_flavor_text, hide_objective_descriptions, ui_zoom_level
             FROM app_config
             WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?;

        let config = if let Some((
            config_version,
            poe_client_log_path,
            log_level,
            zone_refresh_interval_str,
            hide_optional_objectives,
            hide_league_start_objectives,
            hide_flavor_text,
            hide_objective_descriptions,
            ui_zoom_level,
        )) = row
        {
            // Parse enum from TEXT
            let zone_refresh_interval = match zone_refresh_interval_str.as_str() {
                "FiveMinutes" => ZoneRefreshInterval::FiveMinutes,
                "OneHour" => ZoneRefreshInterval::OneHour,
                "TwelveHours" => ZoneRefreshInterval::TwelveHours,
                "TwentyFourHours" => ZoneRefreshInterval::TwentyFourHours,
                "ThreeDays" => ZoneRefreshInterval::ThreeDays,
                "SevenDays" => ZoneRefreshInterval::SevenDays,
                _ => {
                    log::warn!(
                        "Unknown zone_refresh_interval value: {zone_refresh_interval_str}, defaulting to SevenDays"
                    );
                    ZoneRefreshInterval::SevenDays
                }
            };

            AppConfig {
                config_version: config_version as u32,
                poe_client_log_path,
                log_level,
                zone_refresh_interval,
                hide_optional_objectives: hide_optional_objectives != 0,
                hide_league_start_objectives: hide_league_start_objectives != 0,
                hide_flavor_text: hide_flavor_text != 0,
                hide_objective_descriptions: hide_objective_descriptions != 0,
                ui_zoom_level,
            }
        } else {
            // No config exists, return default and save it
            debug!("No configuration found, creating default");
            let default_config = AppConfig::default();
            self.save(&default_config).await?;
            default_config
        };

        // Update in-memory cache
        {
            let mut cache = self.config.write().await;
            *cache = config.clone();
        }

        Ok(config)
    }

    async fn get_in_memory_config(&self) -> AppResult<AppConfig> {
        // Fast read from in-memory cache
        let config = self.config.read().await.clone();
        Ok(config)
    }

    async fn validate_config(
        &self,
        config: &AppConfig,
    ) -> AppResult<ConfigurationValidationResult> {
        let mut errors = Vec::new();

        // Validate POE client log path
        let validator = PathValidator::new_for_poe_logs();
        match validator.validate_path(&config.poe_client_log_path) {
            Ok(_) => {}
            Err(e) => {
                errors.push(format!("Invalid POE client log path: {e}"));
            }
        }

        // Validate log level
        if !AppConfig::VALID_LOG_LEVELS.contains(&config.log_level.as_str()) {
            errors.push(format!(
                "Invalid log level '{}'. Must be one of: {:?}",
                config.log_level,
                AppConfig::VALID_LOG_LEVELS
            ));
        }

        // Validate config version
        if config.config_version > AppConfig::CURRENT_VERSION {
            errors.push(format!(
                "Configuration version {} is newer than supported version {}",
                config.config_version,
                AppConfig::CURRENT_VERSION
            ));
        }

        Ok(ConfigurationValidationResult {
            is_valid: errors.is_empty(),
            errors,
        })
    }
}
