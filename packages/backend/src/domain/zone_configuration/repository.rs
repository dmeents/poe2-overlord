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

/// Zone configuration repository implementation using SQLite.
///
/// Stores zone metadata in the `zone_metadata` table with integer surrogate keys.
/// The shared helpers in `infrastructure/database/helpers.rs` (`get_or_create_zone_id_tx`,
/// `get_or_create_zone_id_pool`) are used by CharacterRepository when it needs to
/// reference zones by ID and auto-create stubs for unknown zones.
///
/// Vec<String> fields (bosses, monsters, npcs, etc.) are stored as JSON TEXT
/// columns for simplicity since they're never queried individually.
pub struct ZoneConfigurationRepositoryImpl {
    pool: SqlitePool,
}

impl ZoneConfigurationRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ZoneConfigurationRepository for ZoneConfigurationRepositoryImpl {
    async fn load_configuration(&self) -> AppResult<ZoneConfiguration> {
        debug!("Loading zone configuration from SQLite");

        // Query all zone metadata rows
        let rows: Vec<(
            String,         // zone_name
            Option<String>, // area_id
            i64,            // act
            Option<i64>,    // area_level
            i64,            // is_town
            i64,            // has_waypoint
            String,         // bosses (JSON)
            String,         // monsters (JSON)
            String,         // npcs (JSON)
            String,         // connected_zones (JSON)
            Option<String>, // description
            String,         // points_of_interest (JSON)
            Option<String>, // image_url
            Option<String>, // wiki_url
            String,         // first_discovered
            String,         // last_updated
        )> = sqlx::query_as(
            "SELECT zone_name, area_id, act, area_level, is_town, has_waypoint,
                    bosses, monsters, npcs, connected_zones, description,
                    points_of_interest, image_url, wiki_url, first_discovered, last_updated
             FROM zone_metadata
             ORDER BY zone_name",
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
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                last_updated: chrono::DateTime::parse_from_rfc3339(&last_updated)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
            };

            zones.insert(zone_name, metadata);
        }

        info!("Loaded {} zones from SQLite", zones.len());

        Ok(ZoneConfiguration { zones })
    }

    async fn upsert_zone(&self, metadata: &ZoneMetadata) -> AppResult<()> {
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
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(zone_name) DO UPDATE SET
                 area_id = excluded.area_id,
                 act = excluded.act,
                 area_level = excluded.area_level,
                 is_town = excluded.is_town,
                 has_waypoint = excluded.has_waypoint,
                 bosses = excluded.bosses,
                 monsters = excluded.monsters,
                 npcs = excluded.npcs,
                 connected_zones = excluded.connected_zones,
                 description = excluded.description,
                 points_of_interest = excluded.points_of_interest,
                 image_url = excluded.image_url,
                 wiki_url = excluded.wiki_url,
                 last_updated = excluded.last_updated",
        )
        .bind(&metadata.zone_name)
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
        .bind(metadata.first_discovered.to_rfc3339())
        .bind(metadata.last_updated.to_rfc3339())
        .execute(&self.pool)
        .await?;

        debug!("Upserted zone: {}", metadata.zone_name);
        Ok(())
    }

    async fn get_zone_by_name(&self, zone_name: &str) -> AppResult<Option<ZoneMetadata>> {
        let row: Option<(
            String,         // zone_name
            Option<String>, // area_id
            i64,            // act
            Option<i64>,    // area_level
            i64,            // is_town
            i64,            // has_waypoint
            String,         // bosses (JSON)
            String,         // monsters (JSON)
            String,         // npcs (JSON)
            String,         // connected_zones (JSON)
            Option<String>, // description
            String,         // points_of_interest (JSON)
            Option<String>, // image_url
            Option<String>, // wiki_url
            String,         // first_discovered
            String,         // last_updated
        )> = sqlx::query_as(
            "SELECT zone_name, area_id, act, area_level, is_town, has_waypoint,
                    bosses, monsters, npcs, connected_zones, description,
                    points_of_interest, image_url, wiki_url, first_discovered, last_updated
             FROM zone_metadata WHERE zone_name = ?",
        )
        .bind(zone_name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::row_to_metadata))
    }

    async fn get_act_and_town(&self, zone_name: &str) -> AppResult<Option<(u32, bool)>> {
        let row: Option<(i64, i64)> =
            sqlx::query_as("SELECT act, is_town FROM zone_metadata WHERE zone_name = ?")
                .bind(zone_name)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(|(act, is_town)| (act as u32, is_town != 0)))
    }

    async fn get_zones_by_act(&self, act: u32) -> AppResult<Vec<ZoneMetadata>> {
        let rows: Vec<(
            String,
            Option<String>,
            i64,
            Option<i64>,
            i64,
            i64,
            String,
            String,
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            String,
            String,
        )> = sqlx::query_as(
            "SELECT zone_name, area_id, act, area_level, is_town, has_waypoint,
                    bosses, monsters, npcs, connected_zones, description,
                    points_of_interest, image_url, wiki_url, first_discovered, last_updated
             FROM zone_metadata WHERE act = ? ORDER BY zone_name",
        )
        .bind(act as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::row_to_metadata).collect())
    }
}

impl ZoneConfigurationRepositoryImpl {
    fn row_to_metadata(
        row: (
            String,
            Option<String>,
            i64,
            Option<i64>,
            i64,
            i64,
            String,
            String,
            String,
            String,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            String,
            String,
        ),
    ) -> ZoneMetadata {
        let (
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
        ) = row;

        ZoneMetadata {
            zone_name,
            area_id,
            act: act as u32,
            area_level: area_level.map(|v| v as u32),
            is_town: is_town != 0,
            has_waypoint: has_waypoint != 0,
            bosses: serde_json::from_str(&bosses_json).unwrap_or_default(),
            monsters: serde_json::from_str(&monsters_json).unwrap_or_default(),
            npcs: serde_json::from_str(&npcs_json).unwrap_or_default(),
            connected_zones: serde_json::from_str(&connected_zones_json).unwrap_or_default(),
            description,
            points_of_interest: serde_json::from_str(&points_of_interest_json).unwrap_or_default(),
            image_url,
            wiki_url,
            first_discovered: chrono::DateTime::parse_from_rfc3339(&first_discovered)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
            last_updated: chrono::DateTime::parse_from_rfc3339(&last_updated)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now),
        }
    }
}
