use crate::domain::zone_configuration::{
    models::{ZoneConfiguration, ZoneMetadata},
    traits::ZoneConfigurationRepository,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use chrono::Utc;
use log::{debug, info};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::path::PathBuf;

/// SQLite-based zone configuration repository.
///
/// Stores zone metadata in the `zone_metadata` table with integer surrogate keys.
/// The shared helper `get_or_create_zone_id` (in infrastructure/database/helpers.rs)
/// is used by both this repository and CharacterRepository when they need to
/// reference zones by ID.
///
/// Vec<String> fields (bosses, monsters, npcs, etc.) are stored as JSON TEXT
/// columns for simplicity since they're never queried individually.
pub struct ZoneConfigurationSqliteRepository {
    pool: SqlitePool,
}

impl ZoneConfigurationSqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationSqliteRepository {
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        debug!("Loading zone configuration from SQLite");

        // Query all zone metadata rows
        let rows: Vec<(
            String,       // zone_name
            Option<String>, // area_id
            i64,          // act
            Option<i64>,  // area_level
            i64,          // is_town
            i64,          // has_waypoint
            String,       // bosses (JSON)
            String,       // monsters (JSON)
            String,       // npcs (JSON)
            String,       // connected_zones (JSON)
            Option<String>, // description
            String,       // points_of_interest (JSON)
            Option<String>, // image_url
            Option<String>, // wiki_url
            String,       // first_discovered
            String,       // last_updated
        )> = sqlx::query_as(
            "SELECT zone_name, area_id, act, area_level, is_town, has_waypoint,
                    bosses, monsters, npcs, connected_zones, description,
                    points_of_interest, image_url, wiki_url, first_discovered, last_updated
             FROM zone_metadata
             ORDER BY zone_name"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut zones = HashMap::new();

        for (
            zone_name,
            area_id,
            act,
            area_level,
            is_town,
            has_waypoint,
            bosses_json,
            monsters_json,
            npcs_json,
            connected_zones_json,
            description,
            points_of_interest_json,
            image_url,
            wiki_url,
            first_discovered,
            last_updated,
        ) in rows
        {
            // Deserialize JSON TEXT columns
            let bosses: Vec<String> = serde_json::from_str(&bosses_json).unwrap_or_default();
            let monsters: Vec<String> = serde_json::from_str(&monsters_json).unwrap_or_default();
            let npcs: Vec<String> = serde_json::from_str(&npcs_json).unwrap_or_default();
            let connected_zones: Vec<String> =
                serde_json::from_str(&connected_zones_json).unwrap_or_default();
            let points_of_interest: Vec<String> =
                serde_json::from_str(&points_of_interest_json).unwrap_or_default();

            let metadata = ZoneMetadata {
                zone_name: zone_name.clone(),
                area_id,
                act: act as u32,
                area_level: area_level.map(|v| v as u32),
                is_town: is_town != 0,
                has_waypoint: has_waypoint != 0,
                bosses,
                monsters,
                npcs,
                connected_zones,
                description,
                points_of_interest,
                image_url,
                wiki_url,
                first_discovered: chrono::DateTime::parse_from_rfc3339(&first_discovered)
                    .ok()
                    .and_then(|dt| Some(dt.with_timezone(&Utc)))
                    .unwrap_or_else(Utc::now),
                last_updated: chrono::DateTime::parse_from_rfc3339(&last_updated)
                    .ok()
                    .and_then(|dt| Some(dt.with_timezone(&Utc)))
                    .unwrap_or_else(Utc::now),
            };

            zones.insert(zone_name, metadata);
        }

        info!("Loaded {} zones from SQLite", zones.len());

        Ok(ZoneConfiguration { zones })
    }

    async fn save_configuration(&self, config: &ZoneConfiguration) -> AppResult<()> {
        debug!("Saving zone configuration to SQLite");

        // Use a transaction for atomic DELETE + INSERT
        let mut tx = self.pool.begin().await?;

        // Delete all existing zones
        sqlx::query("DELETE FROM zone_metadata")
            .execute(&mut *tx)
            .await?;

        // Insert all zones from the configuration
        for (zone_name, metadata) in &config.zones {
            // Serialize Vec<String> to JSON for TEXT columns
            let bosses_json = serde_json::to_string(&metadata.bosses)?;
            let monsters_json = serde_json::to_string(&metadata.monsters)?;
            let npcs_json = serde_json::to_string(&metadata.npcs)?;
            let connected_zones_json = serde_json::to_string(&metadata.connected_zones)?;
            let points_of_interest_json = serde_json::to_string(&metadata.points_of_interest)?;

            sqlx::query(
                "INSERT INTO zone_metadata
                 (zone_name, area_id, act, area_level, is_town, has_waypoint,
                  bosses, monsters, npcs, connected_zones, description,
                  points_of_interest, image_url, wiki_url, first_discovered, last_updated)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(zone_name)
            .bind(&metadata.area_id)
            .bind(metadata.act as i64)
            .bind(metadata.area_level.map(|v| v as i64))
            .bind(if metadata.is_town { 1 } else { 0 })
            .bind(if metadata.has_waypoint { 1 } else { 0 })
            .bind(&bosses_json)
            .bind(&monsters_json)
            .bind(&npcs_json)
            .bind(&connected_zones_json)
            .bind(&metadata.description)
            .bind(&points_of_interest_json)
            .bind(&metadata.image_url)
            .bind(&metadata.wiki_url)
            .bind(&metadata.first_discovered.to_rfc3339())
            .bind(&metadata.last_updated.to_rfc3339())
            .execute(&mut *tx)
            .await?;
        }

        // Commit transaction
        tx.commit().await?;

        info!("Saved {} zones to SQLite", config.zones.len());
        Ok(())
    }

    async fn get_configuration_path(&self) -> PathBuf {
        // Return a placeholder path since we're using SQLite now
        // This method is deprecated but kept for trait compatibility
        PathBuf::from("<sqlite:zone_metadata>")
    }
}
