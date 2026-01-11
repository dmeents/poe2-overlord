#[cfg(test)]
mod tests {
    use crate::domain::zone_configuration::models::*;
    use chrono::{Duration, Utc};

    // Helper to create a minimal ZoneMetadata
    fn create_zone_metadata(zone_name: &str, act: u32, is_town: bool) -> ZoneMetadata {
        let mut zone = ZoneMetadata::new(zone_name.to_string());
        zone.act = act;
        zone.is_town = is_town;
        zone
    }

    // ============= ZoneMetadata Tests =============

    #[test]
    fn test_zone_metadata_new() {
        let zone = ZoneMetadata::new("The Coast".to_string());

        assert_eq!(zone.zone_name, "The Coast");
        assert!(zone.area_id.is_none());
        assert_eq!(zone.act, 0);
        assert!(zone.area_level.is_none());
        assert!(!zone.is_town);
        assert!(!zone.has_waypoint);
        assert!(zone.bosses.is_empty());
        assert!(zone.monsters.is_empty());
        assert!(zone.npcs.is_empty());
        assert!(zone.connected_zones.is_empty());
        assert!(zone.description.is_none());
        assert!(zone.points_of_interest.is_empty());
        assert!(zone.image_url.is_none());
        assert!(zone.wiki_url.is_none());
    }

    #[test]
    fn test_zone_metadata_placeholder() {
        let zone = ZoneMetadata::placeholder("Unknown Zone".to_string());

        assert_eq!(zone.zone_name, "Unknown Zone");
        assert_eq!(zone.act, 0);
        assert!(!zone.is_town);
    }

    #[test]
    fn test_zone_metadata_needs_refresh_stale() {
        let mut zone = ZoneMetadata::new("Test Zone".to_string());
        // Set last_updated to 2 days ago
        zone.last_updated = Utc::now() - Duration::days(2);

        // Refresh interval of 1 day (86400 seconds)
        assert!(zone.needs_refresh(86400));
    }

    #[test]
    fn test_zone_metadata_needs_refresh_fresh() {
        let zone = ZoneMetadata::new("Test Zone".to_string());
        // Zone was just created, should be fresh

        // Refresh interval of 1 day (86400 seconds)
        assert!(!zone.needs_refresh(86400));
    }

    #[test]
    fn test_zone_metadata_needs_refresh_at_boundary() {
        let mut zone = ZoneMetadata::new("Test Zone".to_string());
        // Set last_updated to exactly 1 hour ago
        zone.last_updated = Utc::now() - Duration::hours(1);

        // Refresh interval of 2 hours (7200 seconds) - should not need refresh
        assert!(!zone.needs_refresh(7200));

        // Refresh interval of 30 minutes (1800 seconds) - should need refresh
        assert!(zone.needs_refresh(1800));
    }

    #[test]
    fn test_zone_metadata_serialization() {
        let zone = create_zone_metadata("Clearfell", 1, false);

        let json = serde_json::to_string(&zone).unwrap();
        assert!(json.contains("\"zone_name\":\"Clearfell\""));
        assert!(json.contains("\"act\":1"));
        assert!(json.contains("\"is_town\":false"));
    }

    #[test]
    fn test_zone_metadata_deserialization() {
        let json = r#"{
            "zone_name": "Ogham Village",
            "area_id": "G1_1",
            "act": 1,
            "area_level": 1,
            "is_town": true,
            "has_waypoint": true,
            "bosses": [],
            "monsters": [],
            "npcs": ["Tarkand"],
            "connected_zones": ["The Coast"],
            "description": "A village in Act 1",
            "points_of_interest": [],
            "image_url": null,
            "first_discovered": "2026-01-11T10:00:00Z",
            "last_updated": "2026-01-11T10:00:00Z",
            "wiki_url": "https://wiki.example.com/Ogham_Village"
        }"#;

        let zone: ZoneMetadata = serde_json::from_str(json).unwrap();

        assert_eq!(zone.zone_name, "Ogham Village");
        assert_eq!(zone.area_id, Some("G1_1".to_string()));
        assert_eq!(zone.act, 1);
        assert_eq!(zone.area_level, Some(1));
        assert!(zone.is_town);
        assert!(zone.has_waypoint);
        assert_eq!(zone.npcs, vec!["Tarkand"]);
        assert_eq!(zone.connected_zones, vec!["The Coast"]);
        assert_eq!(zone.wiki_url, Some("https://wiki.example.com/Ogham_Village".to_string()));
    }

    // ============= ZoneConfiguration Tests =============

    #[test]
    fn test_zone_configuration_new() {
        let config = ZoneConfiguration::new();
        assert!(config.zones.is_empty());
    }

    #[test]
    fn test_zone_configuration_default() {
        let config: ZoneConfiguration = Default::default();
        assert!(config.zones.is_empty());
    }

    #[test]
    fn test_zone_configuration_add_zone() {
        let mut config = ZoneConfiguration::new();
        let zone = create_zone_metadata("The Coast", 1, false);

        config.add_zone(zone);

        assert_eq!(config.zones.len(), 1);
        assert!(config.has_zone("The Coast"));
    }

    #[test]
    fn test_zone_configuration_add_zone_updates_existing() {
        let mut config = ZoneConfiguration::new();
        let mut zone1 = create_zone_metadata("The Coast", 1, false);
        zone1.area_level = Some(1);
        config.add_zone(zone1);

        let mut zone2 = create_zone_metadata("The Coast", 1, false);
        zone2.area_level = Some(5);
        config.add_zone(zone2);

        assert_eq!(config.zones.len(), 1);
        assert_eq!(config.get_zone("The Coast").unwrap().area_level, Some(5));
    }

    #[test]
    fn test_zone_configuration_get_zone() {
        let mut config = ZoneConfiguration::new();
        let zone = create_zone_metadata("Clearfell", 1, false);
        config.add_zone(zone);

        let retrieved = config.get_zone("Clearfell");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().zone_name, "Clearfell");
    }

    #[test]
    fn test_zone_configuration_get_zone_nonexistent() {
        let config = ZoneConfiguration::new();
        let retrieved = config.get_zone("Unknown Zone");
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_zone_configuration_get_zone_mut() {
        let mut config = ZoneConfiguration::new();
        let zone = create_zone_metadata("The Coast", 1, false);
        config.add_zone(zone);

        let zone_mut = config.get_zone_mut("The Coast").unwrap();
        zone_mut.has_waypoint = true;

        assert!(config.get_zone("The Coast").unwrap().has_waypoint);
    }

    #[test]
    fn test_zone_configuration_has_zone() {
        let mut config = ZoneConfiguration::new();
        let zone = create_zone_metadata("Clearfell", 1, false);
        config.add_zone(zone);

        assert!(config.has_zone("Clearfell"));
        assert!(!config.has_zone("Unknown Zone"));
    }

    #[test]
    fn test_zone_configuration_get_zone_by_name() {
        let mut config = ZoneConfiguration::new();
        let zone = create_zone_metadata("Clearfell", 1, false);
        config.add_zone(zone);

        let retrieved = config.get_zone_by_name("Clearfell");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().zone_name, "Clearfell");
    }

    #[test]
    fn test_zone_configuration_get_act_zones() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Zone A", 1, false));
        config.add_zone(create_zone_metadata("Zone B", 1, true));
        config.add_zone(create_zone_metadata("Zone C", 2, false));
        config.add_zone(create_zone_metadata("Zone D", 2, false));
        config.add_zone(create_zone_metadata("Zone E", 3, false));

        let act1_zones = config.get_act_zones(1);
        assert_eq!(act1_zones.len(), 2);

        let act2_zones = config.get_act_zones(2);
        assert_eq!(act2_zones.len(), 2);

        let act3_zones = config.get_act_zones(3);
        assert_eq!(act3_zones.len(), 1);

        let act4_zones = config.get_act_zones(4);
        assert!(act4_zones.is_empty());
    }

    #[test]
    fn test_zone_configuration_get_act_for_zone() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Clearfell", 1, false));
        config.add_zone(create_zone_metadata("The Hive", 2, false));

        assert_eq!(config.get_act_for_zone("Clearfell"), Some(1));
        assert_eq!(config.get_act_for_zone("The Hive"), Some(2));
        assert_eq!(config.get_act_for_zone("Unknown"), None);
    }

    #[test]
    fn test_zone_configuration_get_act_for_zone_by_name() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Clearfell", 1, false));

        assert_eq!(config.get_act_for_zone_by_name("Clearfell"), Some(1));
        assert_eq!(config.get_act_for_zone_by_name("Unknown"), None);
    }

    #[test]
    fn test_zone_configuration_is_town_zone() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Ogham Village", 1, true));
        config.add_zone(create_zone_metadata("The Coast", 1, false));

        assert!(config.is_town_zone("Ogham Village"));
        assert!(!config.is_town_zone("The Coast"));
        assert!(!config.is_town_zone("Unknown Zone"));
    }

    #[test]
    fn test_zone_configuration_is_town_zone_by_name() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Ogham Village", 1, true));

        assert!(config.is_town_zone_by_name("Ogham Village"));
        assert!(!config.is_town_zone_by_name("Unknown"));
    }

    #[test]
    fn test_zone_configuration_get_all_zone_names() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Zone A", 1, false));
        config.add_zone(create_zone_metadata("Zone B", 2, false));
        config.add_zone(create_zone_metadata("Zone C", 3, false));

        let names = config.get_all_zone_names();
        assert_eq!(names.len(), 3);
        assert!(names.contains(&"Zone A".to_string()));
        assert!(names.contains(&"Zone B".to_string()));
        assert!(names.contains(&"Zone C".to_string()));
    }

    #[test]
    fn test_zone_configuration_get_all_zone_names_as_keys() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Zone A", 1, false));
        config.add_zone(create_zone_metadata("Zone B", 2, false));

        let keys = config.get_all_zone_names_as_keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"Zone A".to_string()));
        assert!(keys.contains(&"Zone B".to_string()));
    }

    #[test]
    fn test_zone_configuration_get_zones_needing_refresh() {
        let mut config = ZoneConfiguration::new();

        // Fresh zone (just created)
        config.add_zone(create_zone_metadata("Fresh Zone", 1, false));

        // Stale zone (2 weeks old)
        let mut stale_zone = create_zone_metadata("Stale Zone", 1, false);
        stale_zone.last_updated = Utc::now() - Duration::weeks(2);
        config.add_zone(stale_zone);

        let needing_refresh = config.get_zones_needing_refresh();
        assert_eq!(needing_refresh.len(), 1);
        assert!(needing_refresh.contains(&"Stale Zone".to_string()));
        assert!(!needing_refresh.contains(&"Fresh Zone".to_string()));
    }

    #[test]
    fn test_zone_configuration_serialization_roundtrip() {
        let mut config = ZoneConfiguration::new();
        config.add_zone(create_zone_metadata("Clearfell", 1, false));
        config.add_zone(create_zone_metadata("Ogham Village", 1, true));

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: ZoneConfiguration = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.zones.len(), 2);
        assert!(deserialized.has_zone("Clearfell"));
        assert!(deserialized.has_zone("Ogham Village"));
    }
}
