use async_trait::async_trait;
use log::{info, warn};

use super::models::{TrackingSummary, ZoneStats};
use super::traits::ZoneTrackingService;
use crate::domain::character::traits::CharacterRepository;
use crate::errors::AppError;
use crate::infrastructure::events::EventBus;
use std::sync::Arc;

pub struct ZoneTrackingServiceImpl {
    character_repository: Arc<dyn CharacterRepository + Send + Sync>,
    event_bus: Arc<EventBus>,
}

impl ZoneTrackingServiceImpl {
    pub fn new(
        character_repository: Arc<dyn CharacterRepository + Send + Sync>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            character_repository,
            event_bus,
        }
    }
}

#[async_trait]
impl ZoneTrackingService for ZoneTrackingServiceImpl {
    async fn enter_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError> {
        let mut character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;

        // Deactivate any currently active zone
        if let Some(active_zone) = character_data.zones.iter_mut().find(|z| z.is_active) {
            active_zone.stop_timer_and_add_time();
            active_zone.deactivate();
        }

        // Find or create zone
        if let Some(zone) = character_data
            .zones
            .iter_mut()
            .find(|z| z.zone_name == zone_name)
        {
            zone.activate();
            zone.start_timer();
        } else {
            let mut new_zone = ZoneStats::new(zone_name.to_string());
            new_zone.activate();
            new_zone.start_timer();
            character_data.zones.push(new_zone);
        }

        // Update summary
        character_data.summary = TrackingSummary::from_zones(&character_id, &character_data.zones);
        character_data.touch();

        self.character_repository
            .save_character_data(&character_data)
            .await?;

        info!("Character {} entered zone '{}'", character_id, zone_name);

        // Emit tracking data updated event
        let event = crate::infrastructure::events::AppEvent::character_tracking_data_updated(
            character_id.to_string(),
            character_data,
        );
        if let Err(e) = self.event_bus.publish(event).await {
            warn!("Failed to publish tracking data updated event: {}", e);
        }

        Ok(())
    }

    async fn leave_zone(&self, character_id: &str, zone_name: &str) -> Result<(), AppError> {
        let mut character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;

        if let Some(zone) = character_data
            .zones
            .iter_mut()
            .find(|z| z.zone_name == zone_name)
        {
            if zone.is_active {
                zone.stop_timer_and_add_time();
                zone.deactivate();

                // Update summary
                character_data.summary =
                    TrackingSummary::from_zones(&character_id, &character_data.zones);
                character_data.touch();

                self.character_repository
                    .save_character_data(&character_data)
                    .await?;

                info!("Character {} left zone '{}'", character_id, zone_name);
            }
        }

        Ok(())
    }

    async fn record_death(&self, character_id: &str) -> Result<(), AppError> {
        let mut character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;

        let mut zone_name = None;
        if let Some(active_zone) = character_data.zones.iter_mut().find(|z| z.is_active) {
            active_zone.record_death();
            zone_name = Some(active_zone.zone_name.clone());
        }

        if let Some(name) = zone_name {
            // Update summary
            character_data.summary =
                TrackingSummary::from_zones(&character_id, &character_data.zones);
            character_data.touch();

            self.character_repository
                .save_character_data(&character_data)
                .await?;

            info!(
                "Character {} died in zone '{}' (total deaths: {})",
                character_id, name, character_data.summary.total_deaths
            );
        } else {
            warn!(
                "Attempted to record death for character {} but no active zone found",
                character_id
            );
        }

        Ok(())
    }

    async fn add_zone_time(
        &self,
        character_id: &str,
        zone_name: &str,
        seconds: u64,
    ) -> Result<(), AppError> {
        let mut character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;

        if let Some(zone) = character_data
            .zones
            .iter_mut()
            .find(|z| z.zone_name == zone_name)
        {
            zone.add_time(seconds);

            // Update summary
            character_data.summary =
                TrackingSummary::from_zones(&character_id, &character_data.zones);
            character_data.touch();

            self.character_repository
                .save_character_data(&character_data)
                .await?;
        }

        Ok(())
    }

    async fn get_zone_stats(&self, character_id: &str) -> Result<Vec<ZoneStats>, AppError> {
        let character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;
        Ok(character_data.zones.clone())
    }

    async fn get_summary(&self, character_id: &str) -> Result<TrackingSummary, AppError> {
        let character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;
        Ok(character_data.summary.clone())
    }

    async fn get_active_zone(&self, character_id: &str) -> Result<Option<ZoneStats>, AppError> {
        let character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;
        Ok(character_data.zones.iter().find(|z| z.is_active).cloned())
    }

    async fn finalize_active_zones(&self, character_id: &str) -> Result<(), AppError> {
        let mut character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;

        let mut has_changes = false;
        for zone in &mut character_data.zones {
            if zone.is_active {
                zone.stop_timer_and_add_time();
                zone.deactivate();
                has_changes = true;
            }
        }

        if has_changes {
            character_data.summary =
                TrackingSummary::from_zones(&character_id, &character_data.zones);
            character_data.touch();

            self.character_repository
                .save_character_data(&character_data)
                .await?;
        }

        Ok(())
    }

    async fn update_summary(&self, character_id: &str) -> Result<(), AppError> {
        let mut character_data = self
            .character_repository
            .load_character_data(character_id)
            .await?;

        character_data.summary = TrackingSummary::from_zones(&character_id, &character_data.zones);
        character_data.touch();

        self.character_repository
            .save_character_data(&character_data)
            .await?;

        Ok(())
    }
}
