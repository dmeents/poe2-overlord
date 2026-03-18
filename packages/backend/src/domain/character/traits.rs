use async_trait::async_trait;

use crate::errors::AppResult;

use super::models::{
    Ascendency, CharacterClass, CharacterData, CharacterDataResponse, CharacterProfile,
    CharacterSummaryResponse, CharacterUpdateParams, EnrichedZoneStats, League, LocationState,
};
use crate::domain::walkthrough::models::WalkthroughProgress;
use crate::domain::zone_tracking::ZoneStats;

#[async_trait]
pub trait CharacterRepository {
    async fn load_character_data(&self, character_id: &str) -> AppResult<CharacterData>;

    async fn save_character_data(&self, character_data: &CharacterData) -> AppResult<()>;

    async fn delete_character_data(&self, character_id: &str) -> AppResult<()>;

    async fn load_all_characters(&self) -> AppResult<Vec<CharacterData>>;

    /// Sets the active character by ID. Pass None to deactivate all characters.
    /// Returns an error if the character doesn't exist.
    async fn set_active_character(&self, character_id: Option<&str>) -> AppResult<()>;

    /// Gets the currently active character ID, if any.
    async fn get_active_character_id(&self) -> AppResult<Option<String>>;

    /// Checks if a character name is already taken, optionally excluding a specific character ID.
    async fn is_name_taken(&self, name: &str, exclude_id: Option<&str>) -> AppResult<bool>;

    /// Gets all character IDs, ordered by last_played DESC.
    async fn get_character_ids(&self) -> AppResult<Vec<String>>;

    // --- Granular targeted mutations ---

    /// Records a death in the currently active zone (single SQL UPDATE).
    async fn record_death_in_active_zone(&self, character_id: &str) -> AppResult<()>;

    /// Updates only the character level and last_updated timestamp (single SQL UPDATE).
    async fn update_character_level(&self, character_id: &str, new_level: u32) -> AppResult<()>;

    /// Updates only the character profile fields (name, class, ascendency, league, flags, level).
    async fn update_character_profile(
        &self,
        character_id: &str,
        profile: &CharacterProfile,
    ) -> AppResult<()>;

    /// Atomically leaves the current active zone and enters the new zone.
    /// Computes elapsed time for the active zone, deactivates it, then activates the new zone.
    async fn transition_zone(&self, character_id: &str, zone_name: &str) -> AppResult<()>;

    /// Upserts walkthrough progress for a character (single SQL UPSERT).
    async fn update_walkthrough_progress(
        &self,
        character_id: &str,
        progress: &WalkthroughProgress,
    ) -> AppResult<()>;

    /// Stops timers for all active zones for a character and clears current_zone_id.
    async fn finalize_character_active_zones(&self, character_id: &str) -> AppResult<()>;

    /// Loads all characters without zone stats — profile + SQL-aggregated summary.
    async fn load_all_characters_summary(&self) -> AppResult<Vec<CharacterData>>;

    /// Gets the name of the currently active zone for a character, if any.
    async fn get_active_zone_name(&self, character_id: &str) -> AppResult<Option<String>>;

    /// Loads all zone stats for a character (raw, unenriched).
    async fn get_character_zones(&self, character_id: &str) -> AppResult<Vec<ZoneStats>>;

    /// Returns true if the character has visited the given zone at least once.
    async fn has_character_visited_zone(
        &self,
        character_id: &str,
        zone_name: &str,
    ) -> AppResult<bool>;
}

#[async_trait]
pub trait CharacterService: Send + Sync {
    async fn create_character(
        &self,
        name: String,
        class: CharacterClass,
        ascendency: Ascendency,
        league: League,
        hardcore: bool,
        solo_self_found: bool,
    ) -> AppResult<CharacterDataResponse>;

    async fn get_character(&self, character_id: &str) -> AppResult<CharacterDataResponse>;

    async fn get_all_characters(&self) -> AppResult<Vec<CharacterDataResponse>>;

    /// Returns lean character summaries without zones for list views.
    async fn get_all_characters_summary(&self) -> AppResult<Vec<CharacterSummaryResponse>>;

    async fn update_character(
        &self,
        character_id: &str,
        update_params: CharacterUpdateParams,
    ) -> AppResult<CharacterDataResponse>;

    async fn delete_character(&self, character_id: &str) -> AppResult<()>;

    async fn set_active_character(&self, character_id: Option<&str>) -> AppResult<()>;

    async fn get_active_character(&self) -> AppResult<Option<CharacterDataResponse>>;

    async fn is_name_unique(&self, name: &str, exclude_id: Option<&str>) -> AppResult<bool>;

    async fn update_character_level(&self, character_id: &str, new_level: u32) -> AppResult<()>;

    async fn get_current_location(
        &self,
        character_id: &str,
    ) -> AppResult<Option<LocationState>>;

    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> AppResult<()>;

    async fn record_death(&self, character_id: &str) -> AppResult<()>;

    async fn finalize_all_active_zones(&self) -> AppResult<()>;

    /// Reloads and re-publishes enriched character data (zone metadata sync).
    async fn sync_zone_metadata(&self, character_id: &str) -> AppResult<()>;

    /// Updates walkthrough progress for a character. Returns the enriched character data.
    async fn update_walkthrough_progress(
        &self,
        character_id: &str,
        progress: &WalkthroughProgress,
    ) -> AppResult<CharacterDataResponse>;

    /// Returns enriched zone stats for a character (loaded + enriched with zone config).
    async fn get_character_zones(&self, character_id: &str) -> AppResult<Vec<EnrichedZoneStats>>;

    /// Returns true if the character has visited the given zone at least once.
    async fn has_character_visited_zone(
        &self,
        character_id: &str,
        zone_name: &str,
    ) -> AppResult<bool>;
}
