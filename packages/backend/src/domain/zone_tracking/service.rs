use log::info;

use super::models::{TrackingSummary, ZoneStats};
use super::traits::ZoneTrackingService;
use crate::domain::character::CharacterData;
use crate::errors::AppError;

/// Pure business logic implementation for zone tracking.
/// All methods are synchronous and operate on CharacterData without performing I/O.
/// CharacterService is responsible for loading/saving data and publishing events.
pub struct ZoneTrackingServiceImpl;

impl ZoneTrackingServiceImpl {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ZoneTrackingServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoneTrackingService for ZoneTrackingServiceImpl {
    fn enter_zone(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
        act: Option<u32>,
        is_town: bool,
    ) -> Result<(), AppError> {
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
            // Update act and is_town info in case it was updated from wiki data
            zone.act = act;
            zone.is_town = is_town;
            zone.activate();
            zone.start_timer();
        } else {
            let mut new_zone = ZoneStats::new(zone_name.to_string(), act, is_town);
            new_zone.activate();
            new_zone.start_timer();
            character_data.zones.push(new_zone);
        }

        // Update summary
        character_data.summary =
            TrackingSummary::from_zones(&character_data.id, &character_data.zones);

        info!(
            "Character {} entered zone '{}'",
            character_data.id, zone_name
        );

        Ok(())
    }

    fn leave_zone(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
    ) -> Result<(), AppError> {
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
                    TrackingSummary::from_zones(&character_data.id, &character_data.zones);

                info!("Character {} left zone '{}'", character_data.id, zone_name);
            }
        }

        Ok(())
    }

    fn record_death(&self, character_data: &mut CharacterData) -> Result<(), AppError> {
        if let Some(active_zone) = character_data.zones.iter_mut().find(|z| z.is_active) {
            let zone_name = active_zone.zone_name.clone();
            active_zone.record_death();

            // Update summary
            character_data.summary =
                TrackingSummary::from_zones(&character_data.id, &character_data.zones);

            info!(
                "Character {} died in zone '{}' (total deaths: {})",
                character_data.id, zone_name, character_data.summary.total_deaths
            );
        } else {
            info!(
                "Attempted to record death for character {} but no active zone found",
                character_data.id
            );
        }

        Ok(())
    }

    fn add_zone_time(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
        seconds: u64,
    ) -> Result<(), AppError> {
        if let Some(zone) = character_data
            .zones
            .iter_mut()
            .find(|z| z.zone_name == zone_name)
        {
            zone.add_time(seconds);

            // Update summary
            character_data.summary =
                TrackingSummary::from_zones(&character_data.id, &character_data.zones);
        }

        Ok(())
    }

    fn finalize_active_zones(&self, character_data: &mut CharacterData) -> Result<(), AppError> {
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
                TrackingSummary::from_zones(&character_data.id, &character_data.zones);
        }

        Ok(())
    }

    fn update_summary(&self, character_data: &mut CharacterData) {
        character_data.summary =
            TrackingSummary::from_zones(&character_data.id, &character_data.zones);
    }

    fn update_zone_metadata(
        &self,
        character_data: &mut CharacterData,
        zone_name: &str,
        act: Option<u32>,
        is_town: bool,
    ) {
        if let Some(zone) = character_data
            .zones
            .iter_mut()
            .find(|z| z.zone_name == zone_name)
        {
            zone.act = act;
            zone.is_town = is_town;

            // Update summary to reflect the changes
            character_data.summary =
                TrackingSummary::from_zones(&character_data.id, &character_data.zones);
        }
    }
}
