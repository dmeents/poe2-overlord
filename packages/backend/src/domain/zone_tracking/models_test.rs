#[cfg(test)]
mod tests {
    use crate::domain::zone_tracking::models::*;

    // ============= ZoneStats Tests =============

    #[test]
    fn test_zone_stats_new() {
        let zone = ZoneStats::new("The Coast".to_string(), Some(1), false);

        assert_eq!(zone.zone_name, "The Coast");
        assert_eq!(zone.duration, 0);
        assert_eq!(zone.deaths, 0);
        assert_eq!(zone.visits, 0); // Initialized to 0; activate() increments to 1
        assert!(!zone.is_active);
        assert!(zone.entry_timestamp.is_none());
        assert_eq!(zone.act, Some(1));
        assert!(!zone.is_town);
    }

    #[test]
    fn test_zone_stats_new_town() {
        let zone = ZoneStats::new("Ogham Village".to_string(), Some(1), true);

        assert!(zone.is_town);
    }

    #[test]
    fn test_zone_stats_new_no_act() {
        let zone = ZoneStats::new("Unknown Zone".to_string(), None, false);

        assert!(zone.act.is_none());
    }

    #[test]
    fn test_zone_stats_is_hideout() {
        let hideout = ZoneStats::new("My Hideout".to_string(), None, false);
        let not_hideout = ZoneStats::new("The Coast".to_string(), Some(1), false);
        let celestial_hideout = ZoneStats::new("Celestial Hideout".to_string(), None, false);

        assert!(hideout.is_hideout());
        assert!(celestial_hideout.is_hideout());
        assert!(!not_hideout.is_hideout());
    }

    #[test]
    fn test_zone_stats_is_hideout_case_insensitive() {
        let hideout_upper = ZoneStats::new("My HIDEOUT".to_string(), None, false);
        let hideout_mixed = ZoneStats::new("Hideout Area".to_string(), None, false);

        assert!(hideout_upper.is_hideout());
        assert!(hideout_mixed.is_hideout());
    }

    #[test]
    fn test_zone_stats_add_time() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        zone.add_time(100);

        assert_eq!(zone.duration, 100);

        zone.add_time(50);
        assert_eq!(zone.duration, 150);
    }

    #[test]
    fn test_zone_stats_record_death() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        assert_eq!(zone.deaths, 0);

        zone.record_death();
        assert_eq!(zone.deaths, 1);

        zone.record_death();
        zone.record_death();
        assert_eq!(zone.deaths, 3);
    }

    #[test]
    fn test_zone_stats_record_visit() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        assert_eq!(zone.visits, 0); // Now starts at 0

        zone.record_visit();
        assert_eq!(zone.visits, 1);

        zone.record_visit();
        assert_eq!(zone.visits, 2);
    }

    #[test]
    fn test_zone_stats_activate() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        assert!(!zone.is_active);
        assert_eq!(zone.visits, 0); // Starts at 0

        zone.activate();

        assert!(zone.is_active);
        // Activate records first visit, setting it to 1
        assert_eq!(zone.visits, 1);
    }

    #[test]
    fn test_zone_stats_deactivate() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        zone.activate();
        assert!(zone.is_active);

        zone.deactivate();
        assert!(!zone.is_active);
    }

    #[test]
    fn test_zone_stats_start_timer() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        assert!(zone.entry_timestamp.is_none());

        zone.start_timer();
        assert!(zone.entry_timestamp.is_some());
    }

    #[test]
    fn test_zone_stats_stop_timer_and_add_time_no_timer() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        let elapsed = zone.stop_timer_and_add_time();

        assert_eq!(elapsed, 0);
        assert_eq!(zone.duration, 0);
    }

    #[test]
    fn test_zone_stats_stop_timer_and_add_time_with_timer() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        zone.start_timer();

        // Small sleep to ensure some time passes
        std::thread::sleep(std::time::Duration::from_millis(10));

        let elapsed = zone.stop_timer_and_add_time();

        // Verify timer was stopped (elapsed is always >= 0 for u64)
        assert!(zone.entry_timestamp.is_none());
        // Verify some elapsed time was recorded
        assert!(zone.duration == elapsed);
    }

    #[test]
    fn test_zone_stats_get_current_time_spent_no_timer() {
        let mut zone = ZoneStats::new("The Coast".to_string(), Some(1), false);
        zone.add_time(100);

        let current = zone.get_current_time_spent();
        assert_eq!(current, 100);
    }

    #[test]
    fn test_zone_stats_serialization() {
        let zone = ZoneStats::new("Clearfell".to_string(), Some(1), false);

        let json = serde_json::to_string(&zone).unwrap();
        assert!(json.contains("\"zone_name\":\"Clearfell\""));
        assert!(json.contains("\"duration\":0"));
        assert!(json.contains("\"deaths\":0"));
        assert!(json.contains("\"visits\":0")); // Now starts at 0
    }

    // ============= TrackingSummary Tests =============

    #[test]
    fn test_tracking_summary_new() {
        let summary = TrackingSummary::new("char-123".to_string());

        assert_eq!(summary.character_id, "char-123");
        assert_eq!(summary.total_play_time, 0);
        assert_eq!(summary.total_hideout_time, 0);
        assert_eq!(summary.total_town_time, 0);
        assert_eq!(summary.total_zones_visited, 0);
        assert_eq!(summary.total_deaths, 0);
        assert_eq!(summary.play_time_act1, 0);
        assert_eq!(summary.play_time_act2, 0);
        assert_eq!(summary.play_time_act3, 0);
        assert_eq!(summary.play_time_act4, 0);
        assert_eq!(summary.play_time_interlude, 0);
        assert_eq!(summary.play_time_endgame, 0);
    }

    #[test]
    fn test_tracking_summary_from_zones_empty() {
        let summary = TrackingSummary::from_zones("char-123", &[]);

        assert_eq!(summary.total_zones_visited, 0);
        assert_eq!(summary.total_play_time, 0);
    }

    #[test]
    fn test_tracking_summary_from_zones_with_data() {
        let mut zones = vec![
            ZoneStats::new("Zone A".to_string(), Some(1), false),
            ZoneStats::new("Zone B".to_string(), Some(2), false),
            ZoneStats::new("Town".to_string(), Some(1), true),
        ];

        // Add some duration
        zones[0].add_time(100);
        zones[1].add_time(200);
        zones[2].add_time(50);

        // Add some deaths
        zones[0].record_death();
        zones[1].record_death();
        zones[1].record_death();

        let summary = TrackingSummary::from_zones("char-123", &zones);

        assert_eq!(summary.total_zones_visited, 3);
        assert_eq!(summary.total_play_time, 350);
        assert_eq!(summary.total_deaths, 3);
        assert_eq!(summary.total_town_time, 50);
        assert_eq!(summary.play_time_act1, 150); // Zone A (100) + Town (50)
        assert_eq!(summary.play_time_act2, 200);
    }

    #[test]
    fn test_tracking_summary_from_zones_hideout() {
        let mut zones = vec![ZoneStats::new(
            "My Hideout".to_string(),
            None,
            false,
        )];
        zones[0].add_time(300);

        let summary = TrackingSummary::from_zones("char-123", &zones);

        assert_eq!(summary.total_hideout_time, 300);
        assert_eq!(summary.total_play_time, 300);
    }

    #[test]
    fn test_tracking_summary_from_zones_all_acts() {
        let mut zones = vec![
            ZoneStats::new("Act1 Zone".to_string(), Some(1), false),
            ZoneStats::new("Act2 Zone".to_string(), Some(2), false),
            ZoneStats::new("Act3 Zone".to_string(), Some(3), false),
            ZoneStats::new("Act4 Zone".to_string(), Some(4), false),
            ZoneStats::new("Interlude Zone".to_string(), Some(6), false),
            ZoneStats::new("Endgame Zone".to_string(), Some(10), false),
        ];

        zones[0].add_time(100);
        zones[1].add_time(200);
        zones[2].add_time(300);
        zones[3].add_time(400);
        zones[4].add_time(150);
        zones[5].add_time(500);

        let summary = TrackingSummary::from_zones("char-123", &zones);

        assert_eq!(summary.play_time_act1, 100);
        assert_eq!(summary.play_time_act2, 200);
        assert_eq!(summary.play_time_act3, 300);
        assert_eq!(summary.play_time_act4, 400);
        assert_eq!(summary.play_time_interlude, 150);
        assert_eq!(summary.play_time_endgame, 500);
    }

    #[test]
    fn test_tracking_summary_get_act_time() {
        let mut summary = TrackingSummary::new("char-123".to_string());
        summary.play_time_act1 = 100;
        summary.play_time_act2 = 200;
        summary.play_time_act3 = 300;
        summary.play_time_act4 = 400;

        assert_eq!(summary.get_act_time(1), 100);
        assert_eq!(summary.get_act_time(2), 200);
        assert_eq!(summary.get_act_time(3), 300);
        assert_eq!(summary.get_act_time(4), 400);
        assert_eq!(summary.get_act_time(5), 0); // Unknown act
    }

    #[test]
    fn test_tracking_summary_get_total_story_time() {
        let mut summary = TrackingSummary::new("char-123".to_string());
        summary.play_time_act1 = 100;
        summary.play_time_act2 = 200;
        summary.play_time_act3 = 300;
        summary.play_time_act4 = 400;
        summary.play_time_interlude = 50;
        summary.play_time_endgame = 1000; // Not included in story time

        assert_eq!(summary.get_total_story_time(), 1050);
    }

    #[test]
    fn test_tracking_summary_get_act_breakdown() {
        let mut summary = TrackingSummary::new("char-123".to_string());
        summary.play_time_act1 = 100;
        summary.play_time_act2 = 200;
        summary.play_time_act3 = 300;
        summary.play_time_act4 = 400;
        summary.play_time_interlude = 50;
        summary.play_time_endgame = 1000;

        let breakdown = summary.get_act_breakdown();

        assert_eq!(breakdown.len(), 6);
        assert_eq!(breakdown[0], ("Act 1".to_string(), 100));
        assert_eq!(breakdown[1], ("Act 2".to_string(), 200));
        assert_eq!(breakdown[2], ("Act 3".to_string(), 300));
        assert_eq!(breakdown[3], ("Act 4".to_string(), 400));
        assert_eq!(breakdown[4], ("Interlude".to_string(), 50));
        assert_eq!(breakdown[5], ("Endgame".to_string(), 1000));
    }

    #[test]
    fn test_tracking_summary_get_longest_act() {
        let mut summary = TrackingSummary::new("char-123".to_string());
        summary.play_time_act1 = 100;
        summary.play_time_act2 = 500;
        summary.play_time_act3 = 300;
        summary.play_time_act4 = 200;

        let longest = summary.get_longest_act();
        assert!(longest.is_some());
        assert_eq!(longest.unwrap(), ("Act 2".to_string(), 500));
    }

    #[test]
    fn test_tracking_summary_get_longest_act_endgame() {
        let mut summary = TrackingSummary::new("char-123".to_string());
        summary.play_time_act1 = 100;
        summary.play_time_act2 = 100;
        summary.play_time_endgame = 1000;

        let longest = summary.get_longest_act();
        assert!(longest.is_some());
        assert_eq!(longest.unwrap(), ("Endgame".to_string(), 1000));
    }

    #[test]
    fn test_tracking_summary_serialization() {
        let summary = TrackingSummary::new("char-123".to_string());

        let json = serde_json::to_string(&summary).unwrap();
        assert!(json.contains("\"character_id\":\"char-123\""));
        assert!(json.contains("\"total_play_time\":0"));
    }

    #[test]
    fn test_tracking_summary_deserialization() {
        let json = r#"{
            "character_id": "test-char",
            "total_play_time": 1000,
            "total_hideout_time": 100,
            "total_town_time": 200,
            "total_zones_visited": 10,
            "total_deaths": 5,
            "play_time_act1": 200,
            "play_time_act2": 300,
            "play_time_act3": 250,
            "play_time_act4": 150,
            "play_time_interlude": 50,
            "play_time_endgame": 50
        }"#;

        let summary: TrackingSummary = serde_json::from_str(json).unwrap();

        assert_eq!(summary.character_id, "test-char");
        assert_eq!(summary.total_play_time, 1000);
        assert_eq!(summary.total_zones_visited, 10);
        assert_eq!(summary.total_deaths, 5);
    }
}
