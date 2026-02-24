use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

use crate::errors::AppResult;

use super::models::LevelEvent;
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
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO level_events (character_id, level, reached_at, deaths_at_level)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(character_id, level) DO UPDATE SET
                 reached_at = excluded.reached_at,
                 deaths_at_level = excluded.deaths_at_level",
        )
        .bind(character_id)
        .bind(level as i64)
        .bind(reached_at.to_rfc3339())
        .bind(deaths_at_level as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_recent_level_events(
        &self,
        character_id: &str,
        limit: u32,
    ) -> AppResult<Vec<LevelEvent>> {
        let rows: Vec<(i64, String, i64, String, i64)> = sqlx::query_as(
            "SELECT id, character_id, level, reached_at, deaths_at_level
             FROM level_events
             WHERE character_id = ?
             ORDER BY level DESC
             LIMIT ?",
        )
        .bind(character_id)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        let events = rows
            .into_iter()
            .map(|(id, char_id, level, reached_at_str, deaths)| {
                let reached_at = chrono::DateTime::parse_from_rfc3339(&reached_at_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                LevelEvent {
                    id,
                    character_id: char_id,
                    level: level as u32,
                    reached_at,
                    deaths_at_level: deaths as u32,
                }
            })
            .collect();

        Ok(events)
    }

    async fn get_deaths_at_current_level(&self, character_id: &str) -> AppResult<u32> {
        let row: Option<(i64,)> = sqlx::query_as(
            "SELECT deaths_at_current_level FROM characters WHERE id = ?",
        )
        .bind(character_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|(v,)| v as u32).unwrap_or(0))
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
        sqlx::query(
            "UPDATE characters SET deaths_at_current_level = 0 WHERE id = ?",
        )
        .bind(character_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_character_level(&self, character_id: &str) -> AppResult<u32> {
        let row: Option<(i64,)> =
            sqlx::query_as("SELECT level FROM characters WHERE id = ?")
                .bind(character_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(row.map(|(v,)| v as u32).unwrap_or(1))
    }

    async fn count_levels_in_last_minutes(
        &self,
        character_id: &str,
        minutes: u32,
    ) -> AppResult<u32> {
        let cutoff = (Utc::now()
            - chrono::Duration::minutes(minutes as i64))
        .to_rfc3339();

        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM level_events WHERE character_id = ? AND reached_at >= ?",
        )
        .bind(character_id)
        .bind(cutoff)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0 as u32)
    }
}
