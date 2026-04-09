#[cfg(test)]
mod tests {
    use crate::domain::character::models::*;

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
    fn test_character_class_from_str() {
        assert_eq!(
            "Warrior".parse::<CharacterClass>().unwrap(),
            CharacterClass::Warrior
        );
        assert_eq!(
            "Sorceress".parse::<CharacterClass>().unwrap(),
            CharacterClass::Sorceress
        );
        assert_eq!(
            "Ranger".parse::<CharacterClass>().unwrap(),
            CharacterClass::Ranger
        );
        assert_eq!(
            "Huntress".parse::<CharacterClass>().unwrap(),
            CharacterClass::Huntress
        );
        assert_eq!(
            "Monk".parse::<CharacterClass>().unwrap(),
            CharacterClass::Monk
        );
        assert_eq!(
            "Mercenary".parse::<CharacterClass>().unwrap(),
            CharacterClass::Mercenary
        );
        assert_eq!(
            "Witch".parse::<CharacterClass>().unwrap(),
            CharacterClass::Witch
        );
        assert_eq!(
            "Druid".parse::<CharacterClass>().unwrap(),
            CharacterClass::Druid
        );
        assert!("Unknown".parse::<CharacterClass>().is_err());
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
    fn test_ascendency_display() {
        assert_eq!(Ascendency::Titan.to_string(), "Titan");
        assert_eq!(Ascendency::SmithOfKatava.to_string(), "Smith of Katava");
        assert_eq!(
            Ascendency::DiscipleOfVarashta.to_string(),
            "Disciple of Varashta"
        );
        assert_eq!(
            Ascendency::AcolyteOfChayula.to_string(),
            "Acolyte of Chayula"
        );
        assert_eq!(
            Ascendency::GemlingLegionnaire.to_string(),
            "Gemling Legionnaire"
        );
        assert_eq!(Ascendency::BloodMage.to_string(), "Blood Mage");
    }

    #[test]
    fn test_ascendency_from_str() {
        assert_eq!("Titan".parse::<Ascendency>().unwrap(), Ascendency::Titan);
        assert_eq!(
            "Smith of Katava".parse::<Ascendency>().unwrap(),
            Ascendency::SmithOfKatava
        );
        assert_eq!(
            "Disciple of Varashta".parse::<Ascendency>().unwrap(),
            Ascendency::DiscipleOfVarashta
        );
        assert_eq!(
            "Acolyte of Chayula".parse::<Ascendency>().unwrap(),
            Ascendency::AcolyteOfChayula
        );
        assert_eq!(
            "Gemling Legionnaire".parse::<Ascendency>().unwrap(),
            Ascendency::GemlingLegionnaire
        );
        assert_eq!(
            "Blood Mage".parse::<Ascendency>().unwrap(),
            Ascendency::BloodMage
        );
        assert!("Unknown".parse::<Ascendency>().is_err());
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
    fn test_league_display() {
        assert_eq!(League::Standard.to_string(), "Standard");
        assert_eq!(League::RiseOfTheAbyssal.to_string(), "Rise of the Abyssal");
        assert_eq!(League::TheFateOfTheVaal.to_string(), "The Fate of the Vaal");
    }

    #[test]
    fn test_league_from_str() {
        assert_eq!("Standard".parse::<League>().unwrap(), League::Standard);
        assert_eq!(
            "Rise of the Abyssal".parse::<League>().unwrap(),
            League::RiseOfTheAbyssal
        );
        assert_eq!(
            "The Fate of the Vaal".parse::<League>().unwrap(),
            League::TheFateOfTheVaal
        );
        assert!("Unknown".parse::<League>().is_err());
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
            &Ascendency::Tactician,
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

    // ============= LocationType Tests =============

    #[test]
    fn test_location_type_display() {
        assert_eq!(LocationType::Zone.to_string(), "Zone");
    }

    #[test]
    fn test_location_type_default() {
        let location_type: LocationType = Default::default();
        assert_eq!(location_type, LocationType::Zone);
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
}
