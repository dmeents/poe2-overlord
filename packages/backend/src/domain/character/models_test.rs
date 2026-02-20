#[cfg(test)]
mod tests {
    use crate::domain::character::models::*;
    use crate::domain::walkthrough::models::WalkthroughProgress;

    // ============= CharacterClass Tests =============

    #[test]
    fn test_character_class_display() {
        assert_eq!(CharacterClass::Warrior.to_string(), "Warrior");
        assert_eq!(CharacterClass::Sorceress.to_string(), "Sorceress");
        assert_eq!(CharacterClass::Ranger.to_string(), "Ranger");
        assert_eq!(CharacterClass::Huntress.to_string(), "Huntress");
        assert_eq!(CharacterClass::Monk.to_string(), "Monk");
        assert_eq!(CharacterClass::Mercenary.to_string(), "Mercenary");
        assert_eq!(CharacterClass::Witch.to_string(), "Witch");
        assert_eq!(CharacterClass::Druid.to_string(), "Druid");
    }

    #[test]
    fn test_character_class_default() {
        let class: CharacterClass = Default::default();
        assert_eq!(class, CharacterClass::Warrior);
    }

    #[test]
    fn test_character_class_serialization() {
        let class = CharacterClass::Sorceress;
        let json = serde_json::to_string(&class).unwrap();
        assert_eq!(json, "\"Sorceress\"");
    }

    #[test]
    fn test_character_class_deserialization() {
        let class: CharacterClass = serde_json::from_str("\"Ranger\"").unwrap();
        assert_eq!(class, CharacterClass::Ranger);
    }

    // ============= Ascendency Tests =============

    #[test]
    fn test_ascendency_default() {
        let ascendency: Ascendency = Default::default();
        assert_eq!(ascendency, Ascendency::Titan);
    }

    #[test]
    fn test_ascendency_serialization() {
        let ascendency = Ascendency::SmithOfKatava;
        let json = serde_json::to_string(&ascendency).unwrap();
        assert_eq!(json, "\"Smith of Katava\"");
    }

    #[test]
    fn test_ascendency_deserialization() {
        let ascendency: Ascendency = serde_json::from_str("\"Blood Mage\"").unwrap();
        assert_eq!(ascendency, Ascendency::BloodMage);
    }

    // ============= League Tests =============

    #[test]
    fn test_league_default() {
        let league: League = Default::default();
        assert_eq!(league, League::Standard);
    }

    #[test]
    fn test_league_serialization() {
        let league = League::RiseOfTheAbyssal;
        let json = serde_json::to_string(&league).unwrap();
        assert_eq!(json, "\"Rise of the Abyssal\"");
    }

    #[test]
    fn test_league_deserialization() {
        let league: League = serde_json::from_str("\"The Fate of the Vaal\"").unwrap();
        assert_eq!(league, League::TheFateOfTheVaal);
    }

    // ============= is_valid_ascendency_for_class Tests =============

    #[test]
    fn test_valid_warrior_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Titan,
            &CharacterClass::Warrior
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Warbringer,
            &CharacterClass::Warrior
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::SmithOfKatava,
            &CharacterClass::Warrior
        ));
        // Invalid for warrior
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::Stormweaver,
            &CharacterClass::Warrior
        ));
    }

    #[test]
    fn test_valid_sorceress_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Stormweaver,
            &CharacterClass::Sorceress
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Chronomancer,
            &CharacterClass::Sorceress
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::DiscipleOfVarashta,
            &CharacterClass::Sorceress
        ));
        // Invalid for sorceress
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::Titan,
            &CharacterClass::Sorceress
        ));
    }

    #[test]
    fn test_valid_ranger_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Deadeye,
            &CharacterClass::Ranger
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Pathfinder,
            &CharacterClass::Ranger
        ));
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::Invoker,
            &CharacterClass::Ranger
        ));
    }

    #[test]
    fn test_valid_huntress_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Ritualist,
            &CharacterClass::Huntress
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Amazon,
            &CharacterClass::Huntress
        ));
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::Deadeye,
            &CharacterClass::Huntress
        ));
    }

    #[test]
    fn test_valid_monk_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Invoker,
            &CharacterClass::Monk
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::AcolyteOfChayula,
            &CharacterClass::Monk
        ));
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::GemlingLegionnaire,
            &CharacterClass::Monk
        ));
    }

    #[test]
    fn test_valid_mercenary_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::GemlingLegionnaire,
            &CharacterClass::Mercenary
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Tactitian,
            &CharacterClass::Mercenary
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Witchhunter,
            &CharacterClass::Mercenary
        ));
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::BloodMage,
            &CharacterClass::Mercenary
        ));
    }

    #[test]
    fn test_valid_witch_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::BloodMage,
            &CharacterClass::Witch
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Infernalist,
            &CharacterClass::Witch
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Lich,
            &CharacterClass::Witch
        ));
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::Shaman,
            &CharacterClass::Witch
        ));
    }

    #[test]
    fn test_valid_druid_ascendencies() {
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Shaman,
            &CharacterClass::Druid
        ));
        assert!(is_valid_ascendency_for_class(
            &Ascendency::Oracle,
            &CharacterClass::Druid
        ));
        assert!(!is_valid_ascendency_for_class(
            &Ascendency::Lich,
            &CharacterClass::Druid
        ));
    }

    // ============= LocationState Tests =============

    #[test]
    fn test_location_state_new() {
        let location = LocationState::new("The Coast".to_string());
        assert_eq!(location.zone_name, "The Coast");
    }

    #[test]
    fn test_location_state_update_zone_changes() {
        let mut location = LocationState::new("The Coast".to_string());
        let original_time = location.last_updated;

        std::thread::sleep(std::time::Duration::from_millis(1));
        let changed = location.update_zone("Clearfell".to_string());

        assert!(changed);
        assert_eq!(location.zone_name, "Clearfell");
        assert!(location.last_updated > original_time);
    }

    #[test]
    fn test_location_state_update_zone_no_change() {
        let mut location = LocationState::new("The Coast".to_string());
        let original_time = location.last_updated;

        let changed = location.update_zone("The Coast".to_string());

        assert!(!changed);
        assert_eq!(location.zone_name, "The Coast");
        assert_eq!(location.last_updated, original_time);
    }

    #[test]
    fn test_location_state_get_zone_name() {
        let location = LocationState::new("Clearfell".to_string());
        assert_eq!(location.get_zone_name(), "Clearfell");
    }

    // ============= LocationType Tests =============

    #[test]
    fn test_location_type_display() {
        assert_eq!(LocationType::Zone.to_string(), "Zone");
        assert_eq!(LocationType::Act.to_string(), "Act");
        assert_eq!(LocationType::Hideout.to_string(), "Hideout");
    }

    #[test]
    fn test_location_type_default() {
        let location_type: LocationType = Default::default();
        assert_eq!(location_type, LocationType::Zone);
    }

    // ============= CharactersIndex Tests =============

    #[test]
    fn test_characters_index_new() {
        let index = CharactersIndex::new();
        assert!(index.character_ids.is_empty());
        assert!(index.active_character_id.is_none());
    }

    #[test]
    fn test_characters_index_add_character() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        assert!(index.has_character("char1"));
        assert_eq!(index.character_ids.len(), 1);
    }

    #[test]
    fn test_characters_index_add_duplicate_character() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        index.add_character("char1".to_string());
        // Should not add duplicates
        assert_eq!(index.character_ids.len(), 1);
    }

    #[test]
    fn test_characters_index_remove_character() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        index.add_character("char2".to_string());
        index.remove_character("char1");

        assert!(!index.has_character("char1"));
        assert!(index.has_character("char2"));
        assert_eq!(index.character_ids.len(), 1);
    }

    #[test]
    fn test_characters_index_remove_active_character() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        index.set_active_character(Some("char1".to_string()));
        index.remove_character("char1");

        assert!(!index.has_character("char1"));
        assert!(index.active_character_id.is_none());
    }

    #[test]
    fn test_characters_index_set_active_character() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        index.set_active_character(Some("char1".to_string()));

        assert_eq!(index.active_character_id, Some("char1".to_string()));
    }

    #[test]
    fn test_characters_index_clear_active_character() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        index.set_active_character(Some("char1".to_string()));
        index.set_active_character(None);

        assert!(index.active_character_id.is_none());
    }

    // ============= CharacterData Tests =============

    #[test]
    fn test_character_data_new() {
        let character = CharacterData::new(
            "test-id".to_string(),
            "TestChar".to_string(),
            CharacterClass::Warrior,
            Ascendency::Titan,
            League::Standard,
            false,
            false,
        );

        assert_eq!(character.id, "test-id");
        assert_eq!(character.profile.name, "TestChar");
        assert_eq!(character.profile.class, CharacterClass::Warrior);
        assert_eq!(character.profile.ascendency, Ascendency::Titan);
        assert_eq!(character.profile.league, League::Standard);
        assert!(!character.profile.hardcore);
        assert!(!character.profile.solo_self_found);
        assert_eq!(character.profile.level, 1);
        assert!(character.timestamps.last_played.is_none());
        assert!(character.current_location.is_none());
        assert!(character.zones.is_empty());
    }

    #[test]
    fn test_character_data_default() {
        let character: CharacterData = Default::default();
        // Default now generates a UUID to prevent empty ID issues
        assert!(!character.id.is_empty());
        assert!(uuid::Uuid::parse_str(&character.id).is_ok()); // Valid UUID format
        assert!(character.profile.name.is_empty());
        assert_eq!(character.profile.class, CharacterClass::Warrior);
        assert_eq!(character.profile.level, 1);
        // TrackingSummary should use the same ID
        assert_eq!(character.summary.character_id, character.id);
    }

    #[test]
    fn test_character_data_touch() {
        let mut character = CharacterData::new(
            "test-id".to_string(),
            "TestChar".to_string(),
            CharacterClass::Warrior,
            Ascendency::Titan,
            League::Standard,
            false,
            false,
        );

        let original_time = character.timestamps.last_updated;
        std::thread::sleep(std::time::Duration::from_millis(1));
        character.touch();

        assert!(character.timestamps.last_updated > original_time);
    }

    #[test]
    fn test_character_data_update_walkthrough_progress() {
        let mut character = CharacterData::new(
            "test-id".to_string(),
            "TestChar".to_string(),
            CharacterClass::Warrior,
            Ascendency::Titan,
            League::Standard,
            false,
            false,
        );

        let mut progress = WalkthroughProgress::new();
        progress.set_current_step("act_2_step_1".to_string());
        character.update_walkthrough_progress(progress);

        assert_eq!(
            character.walkthrough_progress.current_step_id,
            Some("act_2_step_1".to_string())
        );
    }

    #[test]
    fn test_character_data_get_walkthrough_progress() {
        let character = CharacterData::new(
            "test-id".to_string(),
            "TestChar".to_string(),
            CharacterClass::Warrior,
            Ascendency::Titan,
            League::Standard,
            false,
            false,
        );

        let progress = character.get_walkthrough_progress();
        assert_eq!(progress.current_step_id, Some("act_1_step_1".to_string()));
        assert!(!progress.is_completed);
    }

    #[test]
    fn test_character_data_with_hardcore_ssf() {
        let character = CharacterData::new(
            "test-id".to_string(),
            "HardcoreSSF".to_string(),
            CharacterClass::Witch,
            Ascendency::Lich,
            League::RiseOfTheAbyssal,
            true,
            true,
        );

        assert!(character.profile.hardcore);
        assert!(character.profile.solo_self_found);
        assert_eq!(character.profile.league, League::RiseOfTheAbyssal);
    }

    // ============= CharacterDataResponse Tests =============

    #[test]
    fn test_character_data_response_from_character_data() {
        let character = CharacterData::new(
            "test-id".to_string(),
            "TestChar".to_string(),
            CharacterClass::Ranger,
            Ascendency::Pathfinder,
            League::Standard,
            false,
            true,
        );

        let response: CharacterDataResponse = character.into();

        assert_eq!(response.id, "test-id");
        assert_eq!(response.name, "TestChar");
        assert_eq!(response.class, CharacterClass::Ranger);
        assert_eq!(response.ascendency, Ascendency::Pathfinder);
        assert!(!response.hardcore);
        assert!(response.solo_self_found);
        assert!(response.current_location.is_none());
        assert!(response.zones.is_empty());
    }

    // ============= EnrichedLocationState Tests =============

    #[test]
    fn test_enriched_location_state_from_location_minimal() {
        let location = LocationState::new("Unknown Zone".to_string());
        let enriched = EnrichedLocationState::from_location_minimal(&location);

        assert_eq!(enriched.zone_name, "Unknown Zone");
        assert_eq!(enriched.act, 0);
        assert!(!enriched.is_town);
        assert_eq!(enriched.location_type, LocationType::Zone);
        assert!(enriched.area_id.is_none());
        assert!(enriched.area_level.is_none());
        assert!(!enriched.has_waypoint);
    }

    // ============= Serialization Round-trip Tests =============

    #[test]
    fn test_character_data_serialization_roundtrip() {
        let character = CharacterData::new(
            "test-id".to_string(),
            "TestChar".to_string(),
            CharacterClass::Monk,
            Ascendency::Invoker,
            League::TheFateOfTheVaal,
            true,
            false,
        );

        let json = serde_json::to_string(&character).unwrap();
        let deserialized: CharacterData = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, character.id);
        assert_eq!(deserialized.profile.name, character.profile.name);
        assert_eq!(deserialized.profile.class, character.profile.class);
        assert_eq!(
            deserialized.profile.ascendency,
            character.profile.ascendency
        );
        assert_eq!(deserialized.profile.league, character.profile.league);
        assert_eq!(deserialized.profile.hardcore, character.profile.hardcore);
    }

    #[test]
    fn test_characters_index_serialization_roundtrip() {
        let mut index = CharactersIndex::new();
        index.add_character("char1".to_string());
        index.add_character("char2".to_string());
        index.set_active_character(Some("char1".to_string()));

        let json = serde_json::to_string(&index).unwrap();
        let deserialized: CharactersIndex = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.character_ids.len(), 2);
        assert_eq!(deserialized.active_character_id, Some("char1".to_string()));
    }
}
