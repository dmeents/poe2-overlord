use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::errors::AppResult;

use super::models::{ActiveZoneInfo, LevelEvent};
use super::traits::LevelingRepository;

pub struct LevelingRepositoryImpl {
    pool: SqlitePool,
}

impl LevelingRepositoryImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LevelingRepository for LevelingRepositoryImpl {
    async fn insert_level_event(
        &self,
        character_id: &str,
        level: u32,
        reached_at: DateTime<Utc>,
        deaths_at_level: u32,
        active_seconds: u64,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO level_events (character_id, level, reached_at, deaths_at_level, active_seconds)
             VALUES (?, ?, ?, ?, ?)
             ON CONFLICT(character_id, level) DO UPDATE SET
                 reached_at = excluded.reached_at,
                 deaths_at_level = excluded.deaths_at_level,
                 active_seconds = excluded.active_seconds",
        )
        .bind(character_id)
        .bind(i64::from(level))
        .bind(reached_at.to_rfc3339())
        .bind(i64::from(deaths_at_level))
        .bind(active_seconds as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_recent_level_events(
        &self,
        character_id: &str,
        limit: u32,
    ) -> AppResult<Vec<LevelEvent>> {
        let rows: Vec<(i64, String, i64, String, i64, i64)> = sqlx::query_as(
            "SELECT id, character_id, level, reached_at, deaths_at_level, active_seconds
             FROM level_events
             WHERE character_id = ?
             ORDER BY level DESC
             LIMIT ?",
        )
        .bind(character_id)
        .bind(i64::from(limit))
        .fetch_all(&self.pool)
        .await?;

        let events = rows
            .into_iter()
            .map(
                |(id, char_id, level, reached_at_str, deaths, active_secs)| {
                    let reached_at = chrono::DateTime::parse_from_rfc3339(&reached_at_str)
                        .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));
                    LevelEvent {
                        id,
                        character_id: char_id,
                        level: level as u32,
                        reached_at,
                        deaths_at_level: deaths as u32,
                        active_seconds: active_secs.max(0) as u64,
                    }
                },
            )
            .collect();

        Ok(events)
    }

    async fn get_deaths_at_current_level(&self, character_id: &str) -> AppResult<u32> {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT deaths_at_current_level FROM characters WHERE id = ?")
                .bind(character_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map_or(0, |(v,)| v as u32))
    }

    async fn increment_deaths_at_current_level(&self, character_id: &str) -> AppResult<()> {
        sqlx::query(
            "UPDATE characters SET deaths_at_current_level = deaths_at_current_level + 1
             WHERE id = ?",
        )
        .bind(character_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn reset_deaths_at_current_level(&self, character_id: &str) -> AppResult<()> {
        sqlx::query("UPDATE characters SET deaths_at_current_level = 0 WHERE id = ?")
            .bind(character_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_character_level(&self, character_id: &str) -> AppResult<u32> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT level FROM characters WHERE id = ?")
            .bind(character_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map_or(1, |(v,)| v as u32))
    }

    async fn count_levels_in_last_minutes(
        &self,
        character_id: &str,
        minutes: u32,
    ) -> AppResult<u32> {
        let cutoff = (Utc::now() - chrono::Duration::minutes(i64::from(minutes))).to_rfc3339();

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM level_events WHERE character_id = ? AND reached_at >= ?",
        )
        .bind(character_id)
        .bind(cutoff)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0 as u32)
    }

    async fn get_active_seconds_at_level(&self, character_id: &str) -> AppResult<u64> {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT active_seconds_at_level FROM characters WHERE id = ?")
                .bind(character_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map_or(0, |(v,)| v as u64))
    }

    async fn increment_active_seconds_at_level(
        &self,
        character_id: &str,
        seconds: u64,
    ) -> AppResult<()> {
        sqlx::query(
            "UPDATE characters SET active_seconds_at_level = active_seconds_at_level + ? WHERE id = ?",
        )
        .bind(seconds as i64)
        .bind(character_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn reset_active_seconds_at_level(&self, character_id: &str) -> AppResult<()> {
        sqlx::query("UPDATE characters SET active_seconds_at_level = 0 WHERE id = ?")
            .bind(character_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn get_active_zone_info(&self, character_id: &str) -> AppResult<Option<ActiveZoneInfo>> {
        let row: Option<(String, String, i64)> = sqlx::query_as(
            "SELECT zm.zone_name, zs.entry_timestamp, zm.is_town
             FROM zone_stats zs
             JOIN zone_metadata zm ON zs.zone_id = zm.id
             WHERE zs.character_id = ? AND zs.is_active = 1 AND zs.entry_timestamp IS NOT NULL",
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(zone_name, entry_ts_str, is_town_i64)| {
            let entry_timestamp = chrono::DateTime::parse_from_rfc3339(&entry_ts_str)
                .map_or_else(|_| Utc::now(), |dt| dt.with_timezone(&Utc));
            ActiveZoneInfo {
                zone_name,
                entry_timestamp,
                is_town: is_town_i64 != 0,
            }
        }))
    }

    async fn get_last_level_reached_at(
        &self,
        character_id: &str,
    ) -> AppResult<Option<DateTime<Utc>>> {
        let row: Option<(String,)> = sqlx::query_as(
            "SELECT reached_at FROM level_events WHERE character_id = ? ORDER BY level DESC LIMIT 1",
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(|(ts_str,)| {
            chrono::DateTime::parse_from_rfc3339(&ts_str)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        }))
    }

    async fn get_all_active_zone_character_ids(&self) -> AppResult<Vec<String>> {
        let rows: Vec<(String,)> = sqlx::query_as(
            "SELECT DISTINCT character_id FROM zone_stats
             WHERE is_active = 1 AND entry_timestamp IS NOT NULL",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|(id,)| id).collect())
    }
}
