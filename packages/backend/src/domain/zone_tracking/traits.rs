use crate::domain::character::CharacterData;
use crate::errors::AppError;

/// Zone tracking service that provides pure business logic for zone operations.
/// All methods are synchronous and operate on CharacterData without performing I/O.
/// CharacterService is responsible for loading/saving data and publishing events.
pub trait ZoneTrackingService: Send + Sync {
    /// Enters a zone for a character (modifies character_data in place)
    fn enter_zone(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
        act: Option<u32>,
        is_town: bool,
    ) -> Result<(), AppError>;

    /// Leaves a zone for a character (modifies character_data in place)
    fn leave_zone(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
    ) -> Result<(), AppError>;

    /// Records a death in the current active zone (modifies character_data in place)
    fn record_death(&self, character_data: &mut CharacterData) -> Result<(), AppError>;

    /// Adds time to a specific zone (modifies character_data in place)
    fn add_zone_time(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
        seconds: u64,
    ) -> Result<(), AppError>;

    /// Finalizes all active zones for a character - stops timers (modifies character_data in place)
    fn finalize_active_zones(&self, character_data: &mut CharacterData) -> Result<(), AppError>;

    /// Updates the summary statistics from zone data (modifies character_data in place)
    fn update_summary(&self, character_data: &mut CharacterData);

    /// Updates zone metadata (act and is_town) for a specific zone in character data
    fn update_zone_metadata(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
        act: Option<u32>,
        is_town: bool,
    );
}
