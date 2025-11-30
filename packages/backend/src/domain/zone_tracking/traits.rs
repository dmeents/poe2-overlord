use async_trait::async_trait;

use super::models::{TrackingSummary, ZoneStats};
use crate::errors::AppError;

#[async_trait]
pub trait ZoneTrackingService: Send + Sync {
    /// Enters a zone for a character
    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError>;

    /// Leaves a zone for a character
    async fn leave_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError>;

    /// Records a death in the current active zone
    async fn record_death(&self, character_id: &str) -> Result<(), AppError>;

    /// Adds time to a specific zone
    async fn add_zone_time(
        &self,
        character_id: &str,
        zone_name: &str,
        seconds: u64,
    ) -> Result<(), AppError>;

    /// Gets all zone statistics for a character
    async fn get_zone_stats(&self, character_id: &str) -> Result<Vec<ZoneStats>, AppError>;

    /// Gets the tracking summary for a character
    async fn get_summary(&self, character_id: &str) -> Result<TrackingSummary, AppError>;

    /// Gets the currently active zone for a character
    async fn get_active_zone(&self, character_id: &str) -> Result<Option<ZoneStats>, AppError>;

    /// Finalizes all active zones for a character (stops timers)
    async fn finalize_active_zones(&self, character_id: &str) -> Result<(), AppError>;

    /// Updates the summary statistics from zone data
    async fn update_summary(&self, character_id: &str) -> Result<(), AppError>;
}
