use async_trait::async_trait;
use chrono::Utc;
use log::{debug, warn};
use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::domain::walkthrough::models::WalkthroughProgress;
use crate::domain::zone_tracking::TrackingSummary;
use crate::errors::AppResult;
use crate::infrastructure::database::get_or_create_zone_id_tx;

use super::models::{
    CharacterData, CharacterProfile, CharacterTimestamps, EnrichedZoneStats, LocationState,
};
use super::traits::CharacterRepository;

/// Character repository implementation using SQLite.
///
/// Normalizes CharacterData into 3 tables:
/// - `characters`: profile + timestamps + current_location
/// - `zone_stats`: zone tracking data (references zone_metadata by ID)
/// - `walkthrough_progress`: campaign progress
///
/// TrackingSummary is computed on demand via SQL aggregation in load_character_data / load_all_characters_summary.
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
            String,         // id
            String,         // name
            String,         // class
            String,         // ascendency
            String,         // league
            i64,            // hardcore
            i64,            // solo_self_found
            i64,            // level
            String,         // created_at
            Option<String>, // last_played
            String,         // last_updated
            Option<i64>,    // current_zone_id
            Option<String>, // current_zone_updated_at
        )> = sqlx::query_as(
            "SELECT id, name, class, ascendency, league, hardcore, solo_self_found, level,
                    created_at, last_played, last_updated, current_zone_id, current_zone_updated_at
             FROM characters
             WHERE id = ?",
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

        // Parse character profile using FromStr (replaces fragile serde_json hack)
        let class = class_str
            .parse()
            .map_err(|e: String| crate::errors::AppError::validation_error("parse_class", &e))?;
        let ascendency = ascendency_str.parse().map_err(|e: String| {
            crate::errors::AppError::validation_error("parse_ascendency", &e)
        })?;
        let league = league_str
            .parse()
            .map_err(|e: String| crate::errors::AppError::validation_error("parse_league", &e))?;

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
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc),
            last_played: last_played_str
                .as_ref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            last_updated: chrono::DateTime::parse_from_rfc3339(&last_updated_str)?
                .with_timezone(&Utc),
        };

        // Load current_location if exists
        let current_location: Option<LocationState> = if let Some(zone_id) = current_zone_id {
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

        // Compute TrackingSummary via SQL aggregation
        let summary_row: Option<(i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64, i64)> =
            sqlx::query_as(
                "SELECT
                    COALESCE(SUM(zs.duration), 0),
                    COALESCE(SUM(CASE WHEN zm.zone_type = 'Hideout' OR LOWER(zm.zone_name) LIKE '%hideout%' THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.is_town = 1 AND zm.zone_type != 'Hideout' AND LOWER(zm.zone_name) NOT LIKE '%hideout%' THEN zs.duration ELSE 0 END), 0),
                    COALESCE(COUNT(DISTINCT zs.zone_id), 0),
                    COALESCE(SUM(zs.deaths), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 1 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 2 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 3 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 4 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 5 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 6 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.zone_type = 'Map' OR zm.act = 10 THEN zs.duration ELSE 0 END), 0)
                 FROM zone_stats zs
                 JOIN zone_metadata zm ON zs.zone_id = zm.id
                 WHERE zs.character_id = ?",
            )
            .bind(character_id)
            .fetch_optional(&self.pool)
            .await?;

        let summary = if let Some((
            total_play_time,
            total_hideout_time,
            total_town_time,
            total_zones_visited,
            total_deaths,
            play_time_act1,
            play_time_act2,
            play_time_act3,
            play_time_act4,
            play_time_act5,
            play_time_interlude,
            play_time_endgame,
        )) = summary_row
        {
            TrackingSummary {
                character_id: id.clone(),
                total_play_time: total_play_time as u64,
                total_hideout_time: total_hideout_time as u64,
                total_town_time: total_town_time as u64,
                total_zones_visited: total_zones_visited as u32,
                total_deaths: total_deaths as u32,
                play_time_act1: play_time_act1 as u64,
                play_time_act2: play_time_act2 as u64,
                play_time_act3: play_time_act3 as u64,
                play_time_act4: play_time_act4 as u64,
                play_time_act5: play_time_act5 as u64,
                play_time_interlude: play_time_interlude as u64,
                play_time_endgame: play_time_endgame as u64,
            }
        } else {
            TrackingSummary::new(id.clone())
        };

        // Load walkthrough_progress
        let progress_row: Option<(Option<String>, i64, String)> = sqlx::query_as(
            "SELECT current_step_id, is_completed, last_updated
             FROM walkthrough_progress
             WHERE character_id = ?",
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        let walkthrough_progress =
            if let Some((current_step_id, is_completed, last_updated_str)) = progress_row {
                let last_updated =
                    chrono::DateTime::parse_from_rfc3339(&last_updated_str)?.with_timezone(&Utc);
                WalkthroughProgress {
                    current_step_id,
                    is_completed: is_completed != 0,
                    last_updated,
                }
            } else {
                WalkthroughProgress::new()
            };

        Ok(CharacterData {
            id,
            profile,
            timestamps,
            current_location,
            summary,
            walkthrough_progress,
        })
    }

    async fn save_character_data(&self, character_data: &CharacterData) -> AppResult<()> {
        debug!("Saving character data for {}", character_data.id);

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
        .bind(format!("{}", character_data.profile.ascendency))
        .bind(format!("{}", character_data.profile.league))
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

        // UPSERT walkthrough_progress
        sqlx::query(
            "INSERT INTO walkthrough_progress
             (character_id, current_step_id, is_completed, last_updated)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(character_id) DO UPDATE SET
               current_step_id = excluded.current_step_id,
               is_completed = excluded.is_completed,
               last_updated = excluded.last_updated",
        )
        .bind(&character_data.id)
        .bind(&character_data.walkthrough_progress.current_step_id)
        .bind(if character_data.walkthrough_progress.is_completed {
            1
        } else {
            0
        })
        .bind(
            character_data
                .walkthrough_progress
                .last_updated
                .to_rfc3339(),
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        debug!("Character data saved successfully");
        Ok(())
    }

    async fn delete_character_data(&self, character_id: &str) -> AppResult<()> {
        debug!("Deleting character data for {}", character_id);

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

        let character_rows: Vec<(
            String,         // id
            String,         // name
            String,         // class
            String,         // ascendency
            String,         // league
            i64,            // hardcore
            i64,            // solo_self_found
            i64,            // level
            String,         // created_at
            Option<String>, // last_played
            String,         // last_updated
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

        // SQL aggregation for TrackingSummary (no zone loading needed)
        let summary_rows: Vec<(
            String, // character_id
            i64,    // total_play_time
            i64,    // total_hideout_time
            i64,    // total_town_time
            i64,    // total_zones_visited
            i64,    // total_deaths
            i64,    // play_time_act1
            i64,    // play_time_act2
            i64,    // play_time_act3
            i64,    // play_time_act4
            i64,    // play_time_act5
            i64,    // play_time_interlude
            i64,    // play_time_endgame
        )> = sqlx::query_as(
            "SELECT zs.character_id,
                    COALESCE(SUM(zs.duration), 0),
                    COALESCE(SUM(CASE WHEN zm.zone_type = 'Hideout' OR LOWER(zm.zone_name) LIKE '%hideout%' THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.is_town = 1 AND zm.zone_type != 'Hideout' AND LOWER(zm.zone_name) NOT LIKE '%hideout%' THEN zs.duration ELSE 0 END), 0),
                    COALESCE(COUNT(DISTINCT zs.zone_id), 0),
                    COALESCE(SUM(zs.deaths), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 1 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 2 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 3 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 4 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 5 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 6 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.zone_type = 'Map' OR zm.act = 10 THEN zs.duration ELSE 0 END), 0)
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             GROUP BY zs.character_id",
        )
        .fetch_all(&self.pool)
        .await?;

        let progress_rows: Vec<(String, Option<String>, i64, String)> = sqlx::query_as(
            "SELECT character_id, current_step_id, is_completed, last_updated
             FROM walkthrough_progress",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut summary_by_character: HashMap<String, TrackingSummary> = HashMap::new();
        for (
            character_id,
            total_play_time,
            total_hideout_time,
            total_town_time,
            total_zones_visited,
            total_deaths,
            play_time_act1,
            play_time_act2,
            play_time_act3,
            play_time_act4,
            play_time_act5,
            play_time_interlude,
            play_time_endgame,
        ) in summary_rows
        {
            summary_by_character.insert(
                character_id.clone(),
                TrackingSummary {
                    character_id,
                    total_play_time: total_play_time as u64,
                    total_hideout_time: total_hideout_time as u64,
                    total_town_time: total_town_time as u64,
                    total_zones_visited: total_zones_visited as u32,
                    total_deaths: total_deaths as u32,
                    play_time_act1: play_time_act1 as u64,
                    play_time_act2: play_time_act2 as u64,
                    play_time_act3: play_time_act3 as u64,
                    play_time_act4: play_time_act4 as u64,
                    play_time_act5: play_time_act5 as u64,
                    play_time_interlude: play_time_interlude as u64,
                    play_time_endgame: play_time_endgame as u64,
                },
            );
        }

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
            let class = class_str.parse().unwrap_or_default();
            let ascendency = ascendency_str.parse().unwrap_or_default();
            let league = league_str.parse().unwrap_or_default();

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

            let summary = summary_by_character
                .remove(&id)
                .unwrap_or_else(|| TrackingSummary::new(id.clone()));
            let walkthrough_progress = progress_by_character.remove(&id).unwrap_or_default();

            characters.push(CharacterData {
                id,
                profile,
                timestamps,
                current_location,
                summary,
                walkthrough_progress,
            });
        }

        debug!("Loaded {} characters from SQLite", characters.len());
        Ok(characters)
    }

    async fn set_active_character(&self, character_id: Option<&str>) -> AppResult<()> {
        debug!("Setting active character to {:?}", character_id);

        let mut tx = self.pool.begin().await?;

        sqlx::query("UPDATE characters SET is_active = 0")
            .execute(&mut *tx)
            .await?;

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

    // --- Granular targeted mutations ---

    async fn record_death_in_active_zone(&self, character_id: &str) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        let result = sqlx::query(
            "UPDATE zone_stats SET deaths = deaths + 1, last_visited = ?
             WHERE character_id = ? AND is_active = 1",
        )
        .bind(&now)
        .bind(character_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            log::warn!(
                "record_death_in_active_zone: no active zone found for character {}",
                character_id
            );
        }

        Ok(())
    }

    async fn update_character_level(&self, character_id: &str, new_level: u32) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE characters SET level = ?, last_updated = ? WHERE id = ?")
            .bind(new_level as i64)
            .bind(&now)
            .bind(character_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn update_character_profile(
        &self,
        character_id: &str,
        profile: &CharacterProfile,
    ) -> AppResult<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE characters SET name = ?, class = ?, ascendency = ?, league = ?,
             hardcore = ?, solo_self_found = ?, level = ?, last_updated = ?
             WHERE id = ?",
        )
        .bind(&profile.name)
        .bind(format!("{}", profile.class))
        .bind(format!("{}", profile.ascendency))
        .bind(format!("{}", profile.league))
        .bind(if profile.hardcore { 1i64 } else { 0 })
        .bind(if profile.solo_self_found { 1i64 } else { 0 })
        .bind(profile.level as i64)
        .bind(&now)
        .bind(character_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn transition_zone(&self, character_id: &str, zone_name: &str) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();
        let now_rfc3339 = now.to_rfc3339();

        // 1. Compute elapsed time for active zone and deactivate it
        let active: Option<(i64, Option<String>)> = sqlx::query_as(
            "SELECT zone_id, entry_timestamp FROM zone_stats WHERE character_id = ? AND is_active = 1",
        )
        .bind(character_id)
        .fetch_optional(&mut *tx)
        .await?;

        if let Some((_, entry_ts_opt)) = active {
            if let Some(entry_ts_str) = entry_ts_opt {
                let elapsed = chrono::DateTime::parse_from_rfc3339(&entry_ts_str)
                    .map(|entry_dt| (now - entry_dt.with_timezone(&Utc)).num_seconds().max(0))
                    .unwrap_or(0);

                sqlx::query(
                    "UPDATE zone_stats SET is_active = 0, duration = duration + ?,
                     entry_timestamp = NULL, last_visited = ?
                     WHERE character_id = ? AND is_active = 1",
                )
                .bind(elapsed)
                .bind(&now_rfc3339)
                .bind(character_id)
                .execute(&mut *tx)
                .await?;
            } else {
                sqlx::query(
                    "UPDATE zone_stats SET is_active = 0, last_visited = ?
                     WHERE character_id = ? AND is_active = 1",
                )
                .bind(&now_rfc3339)
                .bind(character_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        // 2. Get or create zone_id for the new zone
        let zone_id = get_or_create_zone_id_tx(&mut tx, zone_name).await?;

        // 3. Upsert zone_stats for the new zone (visits++, is_active=1, entry_timestamp=now)
        sqlx::query(
            "INSERT INTO zone_stats
             (character_id, zone_id, duration, deaths, visits, first_visited, last_visited,
              is_active, entry_timestamp)
             VALUES (?, ?, 0, 0, 1, ?, ?, 1, ?)
             ON CONFLICT(character_id, zone_id) DO UPDATE SET
               visits = zone_stats.visits + 1,
               is_active = 1,
               entry_timestamp = excluded.entry_timestamp,
               last_visited = excluded.last_visited",
        )
        .bind(character_id)
        .bind(zone_id)
        .bind(&now_rfc3339)
        .bind(&now_rfc3339)
        .bind(&now_rfc3339)
        .execute(&mut *tx)
        .await?;

        // 4. Update character's current location and timestamps
        sqlx::query(
            "UPDATE characters SET current_zone_id = ?, current_zone_updated_at = ?,
             last_played = ?, last_updated = ?
             WHERE id = ?",
        )
        .bind(zone_id)
        .bind(&now_rfc3339)
        .bind(&now_rfc3339)
        .bind(&now_rfc3339)
        .bind(character_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn update_walkthrough_progress(
        &self,
        character_id: &str,
        progress: &WalkthroughProgress,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO walkthrough_progress
             (character_id, current_step_id, is_completed, last_updated)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(character_id) DO UPDATE SET
               current_step_id = excluded.current_step_id,
               is_completed = excluded.is_completed,
               last_updated = excluded.last_updated",
        )
        .bind(character_id)
        .bind(&progress.current_step_id)
        .bind(if progress.is_completed { 1i64 } else { 0 })
        .bind(progress.last_updated.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn finalize_character_active_zones(&self, character_id: &str) -> AppResult<()> {
        let mut tx = self.pool.begin().await?;
        let now = Utc::now();
        let now_rfc3339 = now.to_rfc3339();

        // Get all active zones for this character
        let active_zones: Vec<(i64, Option<String>)> = sqlx::query_as(
            "SELECT zone_id, entry_timestamp FROM zone_stats
             WHERE character_id = ? AND is_active = 1",
        )
        .bind(character_id)
        .fetch_all(&mut *tx)
        .await?;

        for (zone_id, entry_ts_opt) in active_zones {
            if let Some(entry_ts_str) = entry_ts_opt {
                let elapsed = chrono::DateTime::parse_from_rfc3339(&entry_ts_str)
                    .map(|entry_dt| (now - entry_dt.with_timezone(&Utc)).num_seconds().max(0))
                    .unwrap_or(0);

                sqlx::query(
                    "UPDATE zone_stats SET is_active = 0, duration = duration + ?,
                     entry_timestamp = NULL, last_visited = ?
                     WHERE character_id = ? AND zone_id = ?",
                )
                .bind(elapsed)
                .bind(&now_rfc3339)
                .bind(character_id)
                .bind(zone_id)
                .execute(&mut *tx)
                .await?;
            } else {
                sqlx::query(
                    "UPDATE zone_stats SET is_active = 0, last_visited = ?
                     WHERE character_id = ? AND zone_id = ?",
                )
                .bind(&now_rfc3339)
                .bind(character_id)
                .bind(zone_id)
                .execute(&mut *tx)
                .await?;
            }
        }

        // Clear current_zone_id
        sqlx::query(
            "UPDATE characters SET current_zone_id = NULL, current_zone_updated_at = NULL,
             last_updated = ? WHERE id = ?",
        )
        .bind(&now_rfc3339)
        .bind(character_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn load_all_characters_summary(&self) -> AppResult<Vec<CharacterData>> {
        debug!("Loading character summaries from SQLite");

        // Query 1: Characters with current zone name
        let character_rows: Vec<(
            String,         // id
            String,         // name
            String,         // class
            String,         // ascendency
            String,         // league
            i64,            // hardcore
            i64,            // solo_self_found
            i64,            // level
            String,         // created_at
            Option<String>, // last_played
            String,         // last_updated
            Option<String>, // current_zone_updated_at
            Option<String>, // current_zone_name
        )> = sqlx::query_as(
            "SELECT c.id, c.name, c.class, c.ascendency, c.league, c.hardcore, c.solo_self_found,
                    c.level, c.created_at, c.last_played, c.last_updated, c.current_zone_updated_at,
                    zm.zone_name as current_zone_name
             FROM characters c
             LEFT JOIN zone_metadata zm ON c.current_zone_id = zm.id
             ORDER BY c.last_played DESC NULLS LAST",
        )
        .fetch_all(&self.pool)
        .await?;

        if character_rows.is_empty() {
            return Ok(Vec::new());
        }

        // Query 2: Per-character aggregate summary from zone_stats
        let summary_rows: Vec<(
            String, // character_id
            i64,    // total_play_time
            i64,    // total_hideout_time
            i64,    // total_town_time
            i64,    // total_zones_visited
            i64,    // total_deaths
            i64,    // play_time_act1
            i64,    // play_time_act2
            i64,    // play_time_act3
            i64,    // play_time_act4
            i64,    // play_time_act5
            i64,    // play_time_interlude
            i64,    // play_time_endgame
        )> = sqlx::query_as(
            "SELECT zs.character_id,
                    COALESCE(SUM(zs.duration), 0),
                    COALESCE(SUM(CASE WHEN zm.zone_type = 'Hideout' OR LOWER(zm.zone_name) LIKE '%hideout%' THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.is_town = 1 AND zm.zone_type != 'Hideout' AND LOWER(zm.zone_name) NOT LIKE '%hideout%' THEN zs.duration ELSE 0 END), 0),
                    COALESCE(COUNT(DISTINCT zs.zone_id), 0),
                    COALESCE(SUM(zs.deaths), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 1 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 2 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 3 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 4 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 5 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.act = 6 AND zm.zone_type NOT IN ('Hideout', 'Map') THEN zs.duration ELSE 0 END), 0),
                    COALESCE(SUM(CASE WHEN zm.zone_type = 'Map' OR zm.act = 10 THEN zs.duration ELSE 0 END), 0)
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             GROUP BY zs.character_id",
        )
        .fetch_all(&self.pool)
        .await?;

        // Query 3: Walkthrough progress
        let progress_rows: Vec<(String, Option<String>, i64, String)> = sqlx::query_as(
            "SELECT character_id, current_step_id, is_completed, last_updated
             FROM walkthrough_progress",
        )
        .fetch_all(&self.pool)
        .await?;

        // Build summary lookup
        let mut summary_by_character: HashMap<String, TrackingSummary> = HashMap::new();
        for (
            character_id,
            total_play_time,
            total_hideout_time,
            total_town_time,
            total_zones_visited,
            total_deaths,
            play_time_act1,
            play_time_act2,
            play_time_act3,
            play_time_act4,
            play_time_act5,
            play_time_interlude,
            play_time_endgame,
        ) in summary_rows
        {
            summary_by_character.insert(
                character_id.clone(),
                TrackingSummary {
                    character_id,
                    total_play_time: total_play_time as u64,
                    total_hideout_time: total_hideout_time as u64,
                    total_town_time: total_town_time as u64,
                    total_zones_visited: total_zones_visited as u32,
                    total_deaths: total_deaths as u32,
                    play_time_act1: play_time_act1 as u64,
                    play_time_act2: play_time_act2 as u64,
                    play_time_act3: play_time_act3 as u64,
                    play_time_act4: play_time_act4 as u64,
                    play_time_act5: play_time_act5 as u64,
                    play_time_interlude: play_time_interlude as u64,
                    play_time_endgame: play_time_endgame as u64,
                },
            );
        }

        // Build walkthrough progress lookup
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

        // Assemble CharacterData with empty zones (summary only)
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
            let class = class_str.parse().unwrap_or_default();
            let ascendency = ascendency_str.parse().unwrap_or_default();
            let league = league_str.parse().unwrap_or_default();

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

            let summary = summary_by_character
                .remove(&id)
                .unwrap_or_else(|| TrackingSummary::new(id.clone()));
            let walkthrough_progress = progress_by_character.remove(&id).unwrap_or_default();

            characters.push(CharacterData {
                id,
                profile,
                timestamps,
                current_location,
                summary,
                walkthrough_progress,
            });
        }

        debug!(
            "Loaded {} character summaries from SQLite",
            characters.len()
        );
        Ok(characters)
    }

    async fn get_active_zone_name(&self, character_id: &str) -> AppResult<Option<String>> {
        let zone_name: Option<String> = sqlx::query_scalar(
            "SELECT zm.zone_name
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             WHERE zs.character_id = ? AND zs.is_active = 1",
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(zone_name)
    }

    async fn get_character_zones(&self, character_id: &str) -> AppResult<Vec<EnrichedZoneStats>> {
        use sqlx::Row;

        let rows = sqlx::query(
            "SELECT zs.duration, zs.deaths, zs.visits, zs.first_visited, zs.last_visited,
                    zs.is_active, zs.entry_timestamp,
                    zm.zone_name, zm.act, zm.area_level, zm.is_town, zm.has_waypoint,
                    zm.zone_type, zm.bosses, zm.npcs, zm.connected_zones, zm.description,
                    zm.points_of_interest, zm.image_url, zm.wiki_url, zm.last_updated
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             WHERE zs.character_id = ?
             ORDER BY zs.last_visited DESC",
        )
        .bind(character_id)
        .fetch_all(&self.pool)
        .await?;

        let mut zones = Vec::new();
        for row in rows {
            let first_visited_str: String = row.get("first_visited");
            let last_visited_str: String = row.get("last_visited");
            let entry_timestamp_str: Option<String> = row.get("entry_timestamp");
            let zm_last_updated_str: String = row.get("last_updated");
            let bosses_json: String = row.get("bosses");
            let npcs_json: String = row.get("npcs");
            let connected_zones_json: String = row.get("connected_zones");
            let points_of_interest_json: String = row.get("points_of_interest");

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
            let zm_last_updated = chrono::DateTime::parse_from_rfc3339(&zm_last_updated_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc));

            let duration: i64 = row.get("duration");
            let deaths: i64 = row.get("deaths");
            let visits: i64 = row.get("visits");
            let is_active: i64 = row.get("is_active");
            let act: i64 = row.get("act");
            let area_level: Option<i64> = row.get("area_level");
            let is_town: i64 = row.get("is_town");
            let has_waypoint: i64 = row.get("has_waypoint");

            zones.push(EnrichedZoneStats {
                zone_name: row.get("zone_name"),
                duration: duration as u64,
                deaths: deaths as u32,
                visits: visits as u32,
                first_visited,
                last_visited,
                is_active: is_active != 0,
                entry_timestamp,
                act: Some(act as u32),
                area_level: area_level.map(|v| v as u32),
                is_town: is_town != 0,
                has_waypoint: has_waypoint != 0,
                zone_type: row.get::<String, _>("zone_type"),
                bosses: serde_json::from_str(&bosses_json).unwrap_or_default(),
                npcs: serde_json::from_str(&npcs_json).unwrap_or_default(),
                connected_zones: serde_json::from_str(&connected_zones_json).unwrap_or_default(),
                description: row.get("description"),
                points_of_interest: serde_json::from_str(&points_of_interest_json)
                    .unwrap_or_default(),
                image_url: row.get("image_url"),
                wiki_url: row.get("wiki_url"),
                last_updated: zm_last_updated,
            });
        }

        Ok(zones)
    }

    async fn has_character_visited_zone(
        &self,
        character_id: &str,
        zone_name: &str,
    ) -> AppResult<bool> {
        let exists: Option<i64> = sqlx::query_scalar(
            "SELECT 1 FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             WHERE zs.character_id = ? AND zm.zone_name = ?
             LIMIT 1",
        )
        .bind(character_id)
        .bind(zone_name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(exists.is_some())
    }
}
