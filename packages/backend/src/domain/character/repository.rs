use async_trait::async_trait;
use chrono::Utc;
use log::{debug, warn};
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::domain::walkthrough::models::WalkthroughProgress;
use crate::domain::zone_tracking::{TrackingSummary, ZoneStats};
use crate::errors::AppResult;
use crate::infrastructure::database::get_or_create_zone_id_tx;

use super::models::{
    CharacterData, CharacterProfile, CharacterTimestamps, LocationState,
};
use super::traits::CharacterRepository;

/// Character repository implementation using SQLite.
///
/// Normalizes CharacterData into 3 tables:
/// - `characters`: profile + timestamps + current_location
/// - `zone_stats`: zone tracking data (references zone_metadata by ID)
/// - `walkthrough_progress`: campaign progress
///
/// TrackingSummary is computed on demand from zone_stats using TrackingSummary::from_zones().
pub struct CharacterRepositoryImpl {
    pool: SqlitePool,
}

impl CharacterRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CharacterRepository for CharacterRepositoryImpl {

    async fn load_character_data(&self, character_id: &str) -> AppResult<CharacterData> {
        debug!("Loading character data for {}", character_id);

        // Load character profile + timestamps
        let character_row: Option<(
            String, // id
            String, // name
            String, // class
            String, // ascendency
            String, // league
            i64,    // hardcore
            i64,    // solo_self_found
            i64,    // level
            String, // created_at
            Option<String>, // last_played
            String, // last_updated
            Option<i64>, // current_zone_id
            Option<String>, // current_zone_updated_at
        )> = sqlx::query_as(
            "SELECT id, name, class, ascendency, league, hardcore, solo_self_found, level,
                    created_at, last_played, last_updated, current_zone_id, current_zone_updated_at
             FROM characters
             WHERE id = ?"
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        let (
            id,
            name,
            class_str,
            ascendency_str,
            league_str,
            hardcore,
            solo_self_found,
            level,
            created_at_str,
            last_played_str,
            last_updated_str,
            current_zone_id,
            current_zone_updated_at_str,
        ) = character_row.ok_or_else(|| {
            crate::errors::AppError::validation_error(
                "load_character_data",
                &format!("Character with ID '{}' not found", character_id),
            )
        })?;

        // Parse character profile
        let class = serde_json::from_str(&format!("\"{}\"", class_str))?;
        let ascendency = serde_json::from_str(&format!("\"{}\"", ascendency_str))?;
        let league = serde_json::from_str(&format!("\"{}\"", league_str))?;

        let profile = CharacterProfile {
            name,
            class,
            ascendency,
            league,
            hardcore: hardcore != 0,
            solo_self_found: solo_self_found != 0,
            level: level as u32,
        };

        let timestamps = CharacterTimestamps {
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)?
                .with_timezone(&Utc),
            last_played: last_played_str
                .as_ref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            last_updated: chrono::DateTime::parse_from_rfc3339(&last_updated_str)?
                .with_timezone(&Utc),
        };

        // Load current_location if exists
        let current_location: Option<LocationState> = if let Some(zone_id) = current_zone_id {
            // Look up zone name by ID
            let zone_name: Option<String> =
                sqlx::query_scalar("SELECT zone_name FROM zone_metadata WHERE id = ?")
                    .bind(zone_id)
                    .fetch_optional(&self.pool)
                    .await?;

            if let Some(zone_name) = zone_name {
                let last_updated = if let Some(timestamp_str) = current_zone_updated_at_str {
                    chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                        .ok()
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(Utc::now)
                } else {
                    Utc::now()
                };

                Some(LocationState {
                    zone_name,
                    last_updated,
                })
            } else {
                None
            }
        } else {
            None
        };

        // Load zone_stats with JOIN to zone_metadata
        let zone_rows: Vec<(
            i64,    // duration
            i64,    // deaths
            i64,    // visits
            String, // first_visited
            String, // last_visited
            i64,    // is_active
            Option<String>, // entry_timestamp
            String, // zone_name (from zone_metadata)
            i64,    // act (from zone_metadata)
            i64,    // is_town (from zone_metadata)
        )> = sqlx::query_as(
            "SELECT zs.duration, zs.deaths, zs.visits, zs.first_visited, zs.last_visited,
                    zs.is_active, zs.entry_timestamp,
                    zm.zone_name, zm.act, zm.is_town
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             WHERE zs.character_id = ?
             ORDER BY zs.last_visited DESC"
        )
        .bind(character_id)
        .fetch_all(&self.pool)
        .await?;

        let mut zones = Vec::new();
        for (
            duration,
            deaths,
            visits,
            first_visited_str,
            last_visited_str,
            is_active,
            entry_timestamp_str,
            zone_name,
            act,
            is_town,
        ) in zone_rows
        {
            let first_visited = chrono::DateTime::parse_from_rfc3339(&first_visited_str)?
                .with_timezone(&Utc);
            let last_visited = chrono::DateTime::parse_from_rfc3339(&last_visited_str)?
                .with_timezone(&Utc);
            let entry_timestamp = entry_timestamp_str
                .as_ref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            zones.push(ZoneStats {
                zone_name,
                duration: duration as u64,
                deaths: deaths as u32,
                visits: visits as u32,
                first_visited,
                last_visited,
                is_active: is_active != 0,
                entry_timestamp,
                act: Some(act as u32),
                is_town: is_town != 0,
            });
        }

        // Load walkthrough_progress
        let progress_row: Option<(Option<String>, i64, String)> = sqlx::query_as(
            "SELECT current_step_id, is_completed, last_updated
             FROM walkthrough_progress
             WHERE character_id = ?"
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        let walkthrough_progress = if let Some((current_step_id, is_completed, last_updated_str)) =
            progress_row
        {
            let last_updated = chrono::DateTime::parse_from_rfc3339(&last_updated_str)?
                .with_timezone(&Utc);
            WalkthroughProgress {
                current_step_id,
                is_completed: is_completed != 0,
                last_updated,
            }
        } else {
            WalkthroughProgress::new()
        };

        // Compute TrackingSummary on demand
        let summary = TrackingSummary::from_zones(&id, &zones);

        Ok(CharacterData {
            id,
            profile,
            timestamps,
            current_location,
            summary,
            zones,
            walkthrough_progress,
        })
    }

    async fn save_character_data(&self, character_data: &CharacterData) -> AppResult<()> {
        debug!("Saving character data for {}", character_data.id);

        // Use transaction for atomicity
        let mut tx = self.pool.begin().await?;

        // Resolve current_zone_id
        let current_zone_id: Option<i64> = if let Some(location) = &character_data.current_location
        {
            Some(get_or_create_zone_id_tx(&mut tx, &location.zone_name).await?)
        } else {
            None
        };

        let current_zone_updated_at = character_data
            .current_location
            .as_ref()
            .map(|loc| loc.last_updated.to_rfc3339());

        // UPDATE or INSERT character
        sqlx::query(
            "INSERT INTO characters
             (id, name, class, ascendency, league, hardcore, solo_self_found, level,
              created_at, last_played, last_updated, current_zone_id, current_zone_updated_at, is_active)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0)
             ON CONFLICT(id) DO UPDATE SET
               name = excluded.name,
               class = excluded.class,
               ascendency = excluded.ascendency,
               league = excluded.league,
               hardcore = excluded.hardcore,
               solo_self_found = excluded.solo_self_found,
               level = excluded.level,
               last_played = excluded.last_played,
               last_updated = excluded.last_updated,
               current_zone_id = excluded.current_zone_id,
               current_zone_updated_at = excluded.current_zone_updated_at"
        )
        .bind(&character_data.id)
        .bind(&character_data.profile.name)
        .bind(format!("{}", character_data.profile.class))
        .bind(serde_json::to_string(&character_data.profile.ascendency)?.trim_matches('"'))
        .bind(serde_json::to_string(&character_data.profile.league)?.trim_matches('"'))
        .bind(if character_data.profile.hardcore { 1 } else { 0 })
        .bind(if character_data.profile.solo_self_found { 1 } else { 0 })
        .bind(character_data.profile.level as i64)
        .bind(character_data.timestamps.created_at.to_rfc3339())
        .bind(character_data.timestamps.last_played.as_ref().map(|dt| dt.to_rfc3339()))
        .bind(character_data.timestamps.last_updated.to_rfc3339())
        .bind(current_zone_id)
        .bind(current_zone_updated_at)
        .execute(&mut *tx)
        .await?;

        // UPSERT zone_stats
        for zone in &character_data.zones {
            let zone_id = get_or_create_zone_id_tx(&mut tx, &zone.zone_name).await?;

            sqlx::query(
                "INSERT INTO zone_stats
                 (character_id, zone_id, duration, deaths, visits, first_visited, last_visited,
                  is_active, entry_timestamp)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                 ON CONFLICT(character_id, zone_id) DO UPDATE SET
                   duration = excluded.duration,
                   deaths = excluded.deaths,
                   visits = excluded.visits,
                   last_visited = excluded.last_visited,
                   is_active = excluded.is_active,
                   entry_timestamp = excluded.entry_timestamp"
            )
            .bind(&character_data.id)
            .bind(zone_id)
            .bind(zone.duration as i64)
            .bind(zone.deaths as i64)
            .bind(zone.visits as i64)
            .bind(zone.first_visited.to_rfc3339())
            .bind(zone.last_visited.to_rfc3339())
            .bind(if zone.is_active { 1 } else { 0 })
            .bind(zone.entry_timestamp.as_ref().map(|dt| dt.to_rfc3339()))
            .execute(&mut *tx)
            .await?;
        }

        // UPSERT walkthrough_progress
        sqlx::query(
            "INSERT INTO walkthrough_progress
             (character_id, current_step_id, is_completed, last_updated)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(character_id) DO UPDATE SET
               current_step_id = excluded.current_step_id,
               is_completed = excluded.is_completed,
               last_updated = excluded.last_updated"
        )
        .bind(&character_data.id)
        .bind(&character_data.walkthrough_progress.current_step_id)
        .bind(if character_data.walkthrough_progress.is_completed { 1 } else { 0 })
        .bind(character_data.walkthrough_progress.last_updated.to_rfc3339())
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        debug!("Character data saved successfully");
        Ok(())
    }

    async fn delete_character_data(&self, character_id: &str) -> AppResult<()> {
        debug!("Deleting character data for {}", character_id);

        // DELETE cascades to zone_stats and walkthrough_progress
        let result = sqlx::query("DELETE FROM characters WHERE id = ?")
            .bind(character_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            warn!("No character found with ID {}", character_id);
        } else {
            debug!("Character deleted successfully (cascaded)");
        }

        Ok(())
    }

    async fn load_all_characters(&self) -> AppResult<Vec<CharacterData>> {
        debug!("Loading all characters from SQLite");

        // Batch load to avoid N+1 queries
        // Query 1: All characters with LEFT JOIN for current_zone_name
        let character_rows: Vec<(
            String, // id
            String, // name
            String, // class
            String, // ascendency
            String, // league
            i64,    // hardcore
            i64,    // solo_self_found
            i64,    // level
            String, // created_at
            Option<String>, // last_played
            String, // last_updated
            Option<String>, // current_zone_updated_at
            Option<String>, // current_zone_name (from zone_metadata)
        )> = sqlx::query_as(
            "SELECT c.id, c.name, c.class, c.ascendency, c.league, c.hardcore, c.solo_self_found, c.level,
                    c.created_at, c.last_played, c.last_updated, c.current_zone_updated_at,
                    zm.zone_name as current_zone_name
             FROM characters c
             LEFT JOIN zone_metadata zm ON c.current_zone_id = zm.id
             ORDER BY c.last_played DESC NULLS LAST"
        )
        .fetch_all(&self.pool)
        .await?;

        if character_rows.is_empty() {
            return Ok(Vec::new());
        }

        // Query 2: All zone_stats with metadata joined
        let zone_rows: Vec<(
            String, // character_id
            i64,    // duration
            i64,    // deaths
            i64,    // visits
            String, // first_visited
            String, // last_visited
            i64,    // is_active
            Option<String>, // entry_timestamp
            String, // zone_name
            i64,    // act
            i64,    // is_town
        )> = sqlx::query_as(
            "SELECT zs.character_id, zs.duration, zs.deaths, zs.visits,
                    zs.first_visited, zs.last_visited, zs.is_active, zs.entry_timestamp,
                    zm.zone_name, zm.act, zm.is_town
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             ORDER BY zs.character_id, zs.last_visited DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        // Query 3: All walkthrough progress
        let progress_rows: Vec<(String, Option<String>, i64, String)> = sqlx::query_as(
            "SELECT character_id, current_step_id, is_completed, last_updated
             FROM walkthrough_progress"
        )
        .fetch_all(&self.pool)
        .await?;

        // Group zone_stats by character_id
        let mut zones_by_character: HashMap<String, Vec<ZoneStats>> = HashMap::new();
        for (
            character_id,
            duration,
            deaths,
            visits,
            first_visited_str,
            last_visited_str,
            is_active,
            entry_timestamp_str,
            zone_name,
            act,
            is_town,
        ) in zone_rows
        {
            let first_visited = chrono::DateTime::parse_from_rfc3339(&first_visited_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            let last_visited = chrono::DateTime::parse_from_rfc3339(&last_visited_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            let entry_timestamp = entry_timestamp_str
                .as_ref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc));

            zones_by_character
                .entry(character_id)
                .or_default()
                .push(ZoneStats {
                    zone_name,
                    duration: duration as u64,
                    deaths: deaths as u32,
                    visits: visits as u32,
                    first_visited,
                    last_visited,
                    is_active: is_active != 0,
                    entry_timestamp,
                    act: Some(act as u32),
                    is_town: is_town != 0,
                });
        }

        // Group walkthrough_progress by character_id
        let mut progress_by_character: HashMap<String, WalkthroughProgress> = HashMap::new();
        for (character_id, current_step_id, is_completed, last_updated_str) in progress_rows {
            let last_updated = chrono::DateTime::parse_from_rfc3339(&last_updated_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            progress_by_character.insert(
                character_id,
                WalkthroughProgress {
                    current_step_id,
                    is_completed: is_completed != 0,
                    last_updated,
                },
            );
        }

        // Assemble CharacterData
        let mut characters = Vec::new();
        for (
            id,
            name,
            class_str,
            ascendency_str,
            league_str,
            hardcore,
            solo_self_found,
            level,
            created_at_str,
            last_played_str,
            last_updated_str,
            current_zone_updated_at_str,
            current_zone_name,
        ) in character_rows
        {
            let class = serde_json::from_str(&format!("\"{}\"", class_str))
                .unwrap_or_default();
            let ascendency = serde_json::from_str(&format!("\"{}\"", ascendency_str))
                .unwrap_or_default();
            let league = serde_json::from_str(&format!("\"{}\"", league_str))
                .unwrap_or_default();

            let profile = CharacterProfile {
                name,
                class,
                ascendency,
                league,
                hardcore: hardcore != 0,
                solo_self_found: solo_self_found != 0,
                level: level as u32,
            };

            let timestamps = CharacterTimestamps {
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
                last_played: last_played_str
                    .as_ref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                last_updated: chrono::DateTime::parse_from_rfc3339(&last_updated_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now),
            };

            // Build current_location from joined zone_name
            let current_location: Option<LocationState> = current_zone_name.map(|zone_name| {
                let last_updated = current_zone_updated_at_str
                    .as_ref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);

                LocationState {
                    zone_name,
                    last_updated,
                }
            });

            let zones = zones_by_character.remove(&id).unwrap_or_default();
            let summary = TrackingSummary::from_zones(&id, &zones);
            let walkthrough_progress = progress_by_character
                .remove(&id)
                .unwrap_or_else(WalkthroughProgress::new);

            characters.push(CharacterData {
                id,
                profile,
                timestamps,
                current_location,
                summary,
                zones,
                walkthrough_progress,
            });
        }

        debug!("Loaded {} characters from SQLite", characters.len());
        Ok(characters)
    }

    async fn character_exists(&self, character_id: &str) -> AppResult<bool> {
        let exists: Option<i64> = sqlx::query_scalar("SELECT 1 FROM characters WHERE id = ? LIMIT 1")
            .bind(character_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(exists.is_some())
    }

    async fn set_active_character(&self, character_id: Option<&str>) -> AppResult<()> {
        debug!("Setting active character to {:?}", character_id);

        let mut tx = self.pool.begin().await?;

        // Reset all is_active flags
        sqlx::query("UPDATE characters SET is_active = 0")
            .execute(&mut *tx)
            .await?;

        // Set active character if specified
        if let Some(id) = character_id {
            let rows_affected = sqlx::query("UPDATE characters SET is_active = 1 WHERE id = ?")
                .bind(id)
                .execute(&mut *tx)
                .await?
                .rows_affected();

            if rows_affected == 0 {
                return Err(crate::errors::AppError::validation_error(
                    "set_active_character",
                    &format!("Character with ID '{}' not found", id),
                ));
            }
        }

        tx.commit().await?;
        debug!("Active character set successfully");
        Ok(())
    }

    async fn get_active_character_id(&self) -> AppResult<Option<String>> {
        let active_id: Option<String> =
            sqlx::query_scalar("SELECT id FROM characters WHERE is_active = 1")
                .fetch_optional(&self.pool)
                .await?;
        Ok(active_id)
    }

    async fn is_name_taken(&self, name: &str, exclude_id: Option<&str>) -> AppResult<bool> {
        let exists: Option<i64> = if let Some(exclude) = exclude_id {
            sqlx::query_scalar("SELECT 1 FROM characters WHERE name = ? AND id != ? LIMIT 1")
                .bind(name)
                .bind(exclude)
                .fetch_optional(&self.pool)
                .await?
        } else {
            sqlx::query_scalar("SELECT 1 FROM characters WHERE name = ? LIMIT 1")
                .bind(name)
                .fetch_optional(&self.pool)
                .await?
        };
        Ok(exists.is_some())
    }

    async fn get_character_ids(&self) -> AppResult<Vec<String>> {
        let character_ids: Vec<String> =
            sqlx::query_scalar("SELECT id FROM characters ORDER BY last_played DESC NULLS LAST")
                .fetch_all(&self.pool)
                .await?;
        Ok(character_ids)
    }
}
