use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::errors::AppResult;

use super::models::{LevelEvent, LevelingStats};

#[async_trait]
pub trait LevelingRepository: Send + Sync {
    /// Upsert a level-up event. The UNIQUE(character_id, level) constraint prevents duplicates.
    async fn insert_level_event(
        &self,
        character_id: &str,
        level: u32,
        reached_at: DateTime<Utc>,
        deaths_at_level: u32,
    ) -> AppResult<()>;

    /// Returns recent level events for a character, ordered by level DESC (newest first).
    async fn get_recent_level_events(
        &self,
        character_id: &str,
        limit: u32,
    ) -> AppResult<Vec<LevelEvent>>;

    /// Returns the current deaths_at_current_level counter for a character.
    async fn get_deaths_at_current_level(&self, character_id: &str) -> AppResult<u32>;

    /// Increments deaths_at_current_level by 1.
    async fn increment_deaths_at_current_level(&self, character_id: &str) -> AppResult<()>;

    /// Resets deaths_at_current_level to 0 (called on level-up).
    async fn reset_deaths_at_current_level(&self, character_id: &str) -> AppResult<()>;

    /// Returns the current level of the character from the characters table.
    async fn get_character_level(&self, character_id: &str) -> AppResult<u32>;

    /// Counts the number of level events in the last `minutes` minutes.
    async fn count_levels_in_last_minutes(
        &self,
        character_id: &str,
        minutes: u32,
    ) -> AppResult<u32>;
}

#[async_trait]
pub trait LevelingService: Send + Sync {
    /// Records a level-up event. Called by log analysis when a CharacterLevel line is parsed.
    async fn record_level_up(
        &self,
        character_id: &str,
        old_level: u32,
        new_level: u32,
    ) -> AppResult<()>;

    /// Records a death for the current level. Called by log analysis on CharacterDeath.
    async fn record_death(&self, character_id: &str) -> AppResult<()>;

    /// Computes and returns all leveling statistics for a character.
    async fn get_leveling_stats(&self, character_id: &str) -> AppResult<LevelingStats>;
}
