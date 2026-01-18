#[cfg(test)]
mod tests {
    use crate::domain::character::models::CharacterData;
    use crate::domain::zone_tracking::service::ZoneTrackingServiceImpl;
    use crate::domain::zone_tracking::traits::ZoneTrackingService;

    fn create_test_character() -> CharacterData {
        CharacterData::default()
    }

    fn create_service() -> ZoneTrackingServiceImpl {
        ZoneTrackingServiceImpl::new()
    }

    #[test]
    fn test_service_creation() {
        let _service = create_service();
        // Service is a zero-sized type, just verify it can be created
    }

    #[test]
    fn test_enter_zone_creates_new_zone() {
        let service = create_service();
        let mut character = create_test_character();

        let result = service.enter_zone(&mut character, "The Coast", Some(1), false);
        assert!(result.is_ok());

        assert_eq!(character.zones.len(), 1);
        assert_eq!(character.zones[0].zone_name, "The Coast");
        assert_eq!(character.zones[0].act, Some(1));
        assert!(!character.zones[0].is_town);
        assert!(character.zones[0].is_active);
        assert!(character.zones[0].entry_timestamp.is_some());
        assert_eq!(character.zones[0].visits, 1);
    }

    #[test]
    fn test_enter_zone_updates_existing_zone() {
        let service = create_service();
        let mut character = create_test_character();

        // Enter zone first time
        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();

        // Leave zone
        service.leave_zone(&mut character, "The Coast").unwrap();

        // Enter same zone again
        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();

        assert_eq!(character.zones.len(), 1);
        assert_eq!(character.zones[0].visits, 2);
        assert!(character.zones[0].is_active);
    }

    #[test]
    fn test_enter_zone_deactivates_previous_zone() {
        let service = create_service();
        let mut character = create_test_character();

        // Enter first zone
        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();

        // Enter second zone (should deactivate first)
        service
            .enter_zone(&mut character, "The Mud Flats", Some(1), false)
            .unwrap();

        assert_eq!(character.zones.len(), 2);
        assert!(!character.zones[0].is_active);
        assert!(character.zones[1].is_active);
    }

    #[test]
    fn test_enter_zone_updates_metadata() {
        let service = create_service();
        let mut character = create_test_character();

        // Enter zone with no act
        service
            .enter_zone(&mut character, "The Coast", None, false)
            .unwrap();
        service.leave_zone(&mut character, "The Coast").unwrap();

        // Re-enter with act data
        service
            .enter_zone(&mut character, "The Coast", Some(1), true)
            .unwrap();

        assert_eq!(character.zones[0].act, Some(1));
        assert!(character.zones[0].is_town);
    }

    #[test]
    fn test_leave_zone_deactivates_zone() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();
        service.leave_zone(&mut character, "The Coast").unwrap();

        assert!(!character.zones[0].is_active);
        assert!(character.zones[0].entry_timestamp.is_none());
    }

    #[test]
    fn test_leave_zone_nonexistent_zone_is_ok() {
        let service = create_service();
        let mut character = create_test_character();

        let result = service.leave_zone(&mut character, "Nonexistent Zone");
        assert!(result.is_ok());
    }

    #[test]
    fn test_leave_zone_inactive_zone_is_noop() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();
        service.leave_zone(&mut character, "The Coast").unwrap();

        // Leave again - should be noop
        let result = service.leave_zone(&mut character, "The Coast");
        assert!(result.is_ok());
    }

    #[test]
    fn test_record_death_increments_death_count() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();
        service.record_death(&mut character).unwrap();

        assert_eq!(character.zones[0].deaths, 1);
        assert_eq!(character.summary.total_deaths, 1);
    }

    #[test]
    fn test_record_death_no_active_zone_is_ok() {
        let service = create_service();
        let mut character = create_test_character();

        let result = service.record_death(&mut character);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_zone_time() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();
        service
            .add_zone_time(&mut character, "The Coast", 3600)
            .unwrap();

        assert_eq!(character.zones[0].duration, 3600);
    }

    #[test]
    fn test_add_zone_time_nonexistent_zone_is_ok() {
        let service = create_service();
        let mut character = create_test_character();

        let result = service.add_zone_time(&mut character, "Nonexistent Zone", 100);
        assert!(result.is_ok());
    }

    #[test]
    fn test_finalize_active_zones() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();
        assert!(character.zones[0].is_active);

        service.finalize_active_zones(&mut character).unwrap();

        assert!(!character.zones[0].is_active);
        assert!(character.zones[0].entry_timestamp.is_none());
    }

    #[test]
    fn test_finalize_active_zones_no_active_is_ok() {
        let service = create_service();
        let mut character = create_test_character();

        let result = service.finalize_active_zones(&mut character);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_summary() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();

        // Manually modify zone
        character.zones[0].deaths = 5;

        service.update_summary(&mut character);

        assert_eq!(character.summary.total_deaths, 5);
    }

    #[test]
    fn test_update_zone_metadata() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", None, false)
            .unwrap();

        service.update_zone_metadata(&mut character, "The Coast", Some(1), true);

        assert_eq!(character.zones[0].act, Some(1));
        assert!(character.zones[0].is_town);
    }

    #[test]
    fn test_update_zone_metadata_nonexistent_zone_is_noop() {
        let service = create_service();
        let mut character = create_test_character();

        // Should not panic
        service.update_zone_metadata(&mut character, "Nonexistent", Some(1), true);

        assert!(character.zones.is_empty());
    }

    #[test]
    fn test_summary_updates_after_enter() {
        let service = create_service();
        let mut character = create_test_character();

        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();

        assert_eq!(character.summary.total_zones_visited, 1);
    }

    #[test]
    fn test_town_time_tracking() {
        let service = create_service();
        let mut character = create_test_character();

        // Enter town
        service
            .enter_zone(&mut character, "Clearfell Encampment", Some(1), true)
            .unwrap();

        // Add time
        service
            .add_zone_time(&mut character, "Clearfell Encampment", 600)
            .unwrap();

        assert!(character.zones[0].is_town);
        assert_eq!(character.summary.total_town_time, 600);
    }

    #[test]
    fn test_hideout_time_tracking() {
        let service = create_service();
        let mut character = create_test_character();

        // Enter hideout
        service
            .enter_zone(&mut character, "Coastal Hideout", Some(10), false)
            .unwrap();

        // Add time
        service
            .add_zone_time(&mut character, "Coastal Hideout", 600)
            .unwrap();

        assert!(character.zones[0].is_hideout());
        assert_eq!(character.summary.total_hideout_time, 600);
    }

    #[test]
    fn test_act_time_tracking() {
        let service = create_service();
        let mut character = create_test_character();

        // Enter Act 1 zone
        service
            .enter_zone(&mut character, "The Coast", Some(1), false)
            .unwrap();
        service
            .add_zone_time(&mut character, "The Coast", 1000)
            .unwrap();

        // Enter Act 2 zone
        service
            .enter_zone(&mut character, "Vastiri Outskirts", Some(2), false)
            .unwrap();
        service
            .add_zone_time(&mut character, "Vastiri Outskirts", 2000)
            .unwrap();

        assert_eq!(character.summary.play_time_act1, 1000);
        assert_eq!(character.summary.play_time_act2, 2000);
    }

    #[test]
    fn test_multiple_active_zones_warning_case() {
        let service = create_service();
        let mut character = create_test_character();

        // Manually create a character with multiple active zones (edge case)
        use crate::domain::zone_tracking::models::ZoneStats;

        let mut zone1 = ZoneStats::new("Zone 1".to_string(), Some(1), false);
        zone1.activate();
        zone1.start_timer();

        let mut zone2 = ZoneStats::new("Zone 2".to_string(), Some(1), false);
        zone2.activate();
        zone2.start_timer();

        character.zones.push(zone1);
        character.zones.push(zone2);

        // Enter new zone - should deactivate both
        service
            .enter_zone(&mut character, "Zone 3", Some(1), false)
            .unwrap();

        let active_count = character.zones.iter().filter(|z| z.is_active).count();
        assert_eq!(active_count, 1);
        assert!(character.zones[2].is_active);
    }
}
