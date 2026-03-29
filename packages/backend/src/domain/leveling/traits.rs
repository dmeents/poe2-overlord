use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::errors::AppResult;

use super::models::{ActiveZoneInfo, LevelEvent, LevelingStats};

#[async_trait]
pub trait LevelingRepository: Send + Sync {
    /// Upsert a level-up event. The UNIQUE(character_id, level) constraint prevents duplicates.
    async fn insert_level_event(
        &self,
        character_id: &str,
        level: u32,
        reached_at: DateTime<Utc>,
        deaths_at_level: u32,
        active_seconds: u64,
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

    /// Returns the accumulated active grinding seconds for the current level.
    async fn get_active_seconds_at_level(&self, character_id: &str) -> AppResult<u64>;

    /// Adds `seconds` to the persisted active grinding counter.
    async fn increment_active_seconds_at_level(
        &self,
        character_id: &str,
        seconds: u64,
    ) -> AppResult<()>;

    /// Resets the active grinding counter to 0 (called on level-up).
    async fn reset_active_seconds_at_level(&self, character_id: &str) -> AppResult<()>;

    /// Returns the currently active (non-finalized) zone for a character.
    async fn get_active_zone_info(
        &self,
        character_id: &str,
    ) -> AppResult<Option<ActiveZoneInfo>>;

    /// Returns the timestamp of the most recent level-up event.
    async fn get_last_level_reached_at(
        &self,
        character_id: &str,
    ) -> AppResult<Option<DateTime<Utc>>>;

    /// Returns all character IDs that currently have an active zone (for bulk finalization).
    async fn get_all_active_zone_character_ids(&self) -> AppResult<Vec<String>>;
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

    /// Accumulates the time spent in the current active grinding zone into the DB.
    /// Called before a zone transition so the departing zone's time is captured.
    async fn record_active_zone_exit(&self, character_id: &str) -> AppResult<()>;

    /// Accumulates active zone time for ALL characters with active zones.
    /// Called on game stop, app shutdown, and session gaps.
    async fn finalize_active_zone_times(&self) -> AppResult<()>;

    /// Computes and publishes a `LevelingStatsUpdated` event for the given character.
    /// Called after zone transitions so the frontend gets fresh `is_actively_grinding`.
    async fn emit_stats_update(&self, character_id: &str) -> AppResult<()>;

    /// Returns character IDs that currently have an active zone.
    /// Used to emit stats updates after bulk finalization on game stop.
    async fn get_active_zone_character_ids(&self) -> AppResult<Vec<String>>;
}
