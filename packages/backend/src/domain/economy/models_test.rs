#[cfg(test)]
mod tests {
    use crate::domain::economy::models::*;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    fn test_tier_selection_high_value_primary() {
        let config = TierConfig::default();
        let primary_value = 1.5;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let tier = CurrencyExchangeRate::select_optimal_tier(
            primary_value,
            secondary_rate,
            tertiary_rate,
            &config,
        );

        assert_eq!(tier, CurrencyTier::Primary);
    }

    #[test]
    fn test_tier_selection_medium_value_secondary() {
        let config = TierConfig::default();
        let primary_value = 0.02;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let tier = CurrencyExchangeRate::select_optimal_tier(
            primary_value,
            secondary_rate,
            tertiary_rate,
            &config,
        );

        assert_eq!(tier, CurrencyTier::Secondary);
    }

    #[test]
    fn test_tier_selection_low_value_tertiary() {
        let config = TierConfig::default();
        let primary_value = 0.00003226;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let tier = CurrencyExchangeRate::select_optimal_tier(
            primary_value,
            secondary_rate,
            tertiary_rate,
            &config,
        );

        assert_eq!(tier, CurrencyTier::Tertiary);
    }

    #[test]
    fn test_calculate_value_in_tier_primary() {
        let primary_value = 1.5;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let result = CurrencyExchangeRate::calculate_value_in_tier(
            primary_value,
            CurrencyTier::Primary,
            secondary_rate,
            tertiary_rate,
        );

        assert_eq!(result, 1.5);
    }

    #[test]
    fn test_calculate_value_in_tier_secondary() {
        let primary_value = 0.2773;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let result = CurrencyExchangeRate::calculate_value_in_tier(
            primary_value,
            CurrencyTier::Secondary,
            secondary_rate,
            tertiary_rate,
        );

        assert!((result - 11.88).abs() < 0.1);
    }

    #[test]
    fn test_calculate_value_in_tier_tertiary() {
        let primary_value = 0.00003226;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let result = CurrencyExchangeRate::calculate_value_in_tier(
            primary_value,
            CurrencyTier::Tertiary,
            secondary_rate,
            tertiary_rate,
        );

        assert!((result - 0.0592).abs() < 0.001);
    }

    #[test]
    fn test_finalize_display_value_no_inversion() {
        let (value, inverted) = CurrencyExchangeRate::finalize_display_value(11.88);
        assert_eq!(value, 11.88);
        assert!(!inverted);
    }

    #[test]
    fn test_finalize_display_value_with_inversion() {
        let (value, inverted) = CurrencyExchangeRate::finalize_display_value(0.0592);
        assert!((value - 16.89).abs() < 0.1);
        assert!(inverted);
    }

    #[test]
    fn test_finalize_display_value_edge_case_exactly_one() {
        let (value, inverted) = CurrencyExchangeRate::finalize_display_value(1.0);
        assert_eq!(value, 1.0);
        assert!(!inverted);
    }

    #[test]
    fn test_tier_selection_annulment_orb_example() {
        let config = TierConfig::default();
        let primary_value = 0.2773;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let tier = CurrencyExchangeRate::select_optimal_tier(
            primary_value,
            secondary_rate,
            tertiary_rate,
            &config,
        );

        assert_eq!(tier, CurrencyTier::Primary);

        let secondary_value = primary_value * secondary_rate;
        assert!((secondary_value - 11.88).abs() < 0.1);
    }

    #[test]
    fn test_tertiary_currency_detection() {
        // Tertiary currency selection requires rates to be set
        // The currency with the lowest rate (highest value) is selected
        let divine_item = CurrencyItem {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "divine-orb".to_string(),
        };

        let chaos_item = CurrencyItem {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "chaos-orb".to_string(),
        };

        let exalted_item = CurrencyItem {
            id: "exalted".to_string(),
            name: "Exalted Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "exalted-orb".to_string(),
        };

        // Set up rates - exalted has rate 5.0 (lower = higher value = tertiary)
        let mut rates = HashMap::new();
        rates.insert("divine".to_string(), 1.0);
        rates.insert("chaos".to_string(), 42.86);
        rates.insert("exalted".to_string(), 5.0);

        let core = CurrencyCore {
            items: vec![
                divine_item.clone(),
                chaos_item.clone(),
                exalted_item.clone(),
            ],
            rates,
            primary: "divine".to_string(),
            secondary: "chaos".to_string(),
        };

        let tertiary = core.get_tertiary_currency();
        assert!(tertiary.is_some());
        assert_eq!(tertiary.unwrap().id, "exalted");

        // Test with different primary/secondary - chaos should be tertiary
        let mut rates2 = HashMap::new();
        rates2.insert("divine".to_string(), 1.0);
        rates2.insert("chaos".to_string(), 42.86);
        rates2.insert("exalted".to_string(), 5.0);

        let core2 = CurrencyCore {
            items: vec![divine_item, chaos_item, exalted_item],
            rates: rates2,
            primary: "divine".to_string(),
            secondary: "exalted".to_string(),
        };
        let tertiary2 = core2.get_tertiary_currency();
        assert!(tertiary2.is_some());
        // chaos (42.86) is the only non-primary/non-secondary
        assert_eq!(tertiary2.unwrap().id, "chaos");
    }

    #[test]
    fn test_tertiary_currency_deterministic_selection() {
        // When multiple currencies qualify as tertiary, select the highest value one
        // (lowest exchange rate = highest value)
        let divine_item = CurrencyItem {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "divine-orb".to_string(),
        };

        let chaos_item = CurrencyItem {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "chaos-orb".to_string(),
        };

        let exalted_item = CurrencyItem {
            id: "exalted".to_string(),
            name: "Exalted Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "exalted-orb".to_string(),
        };

        let annul_item = CurrencyItem {
            id: "annul".to_string(),
            name: "Orb of Annulment".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "annul-orb".to_string(),
        };

        // Set up rates - exalted (5.0) has lower rate than annul (10.0)
        // So exalted should be selected as tertiary (highest value)
        let mut rates = HashMap::new();
        rates.insert("divine".to_string(), 1.0);
        rates.insert("chaos".to_string(), 42.86);
        rates.insert("exalted".to_string(), 5.0);
        rates.insert("annul".to_string(), 10.0);

        let core = CurrencyCore {
            items: vec![divine_item, chaos_item, exalted_item, annul_item],
            rates,
            primary: "divine".to_string(),
            secondary: "chaos".to_string(),
        };

        let tertiary = core.get_tertiary_currency();
        assert!(tertiary.is_some());
        // Exalted has lower rate (5.0) than annul (10.0), so it's selected
        assert_eq!(tertiary.unwrap().id, "exalted");
    }

    #[test]
    fn test_tertiary_currency_requires_rates() {
        // If a currency doesn't have a rate entry, it won't be selected as tertiary
        let divine_item = CurrencyItem {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "divine-orb".to_string(),
        };

        let chaos_item = CurrencyItem {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "chaos-orb".to_string(),
        };

        let exalted_item = CurrencyItem {
            id: "exalted".to_string(),
            name: "Exalted Orb".to_string(),
            image: "/image.png".to_string(),
            category: "Currency".to_string(),
            details_id: "exalted-orb".to_string(),
        };

        // No rates set - tertiary should be None
        let core = CurrencyCore {
            items: vec![divine_item, chaos_item, exalted_item],
            rates: HashMap::new(),
            primary: "divine".to_string(),
            secondary: "chaos".to_string(),
        };

        let tertiary = core.get_tertiary_currency();
        assert!(tertiary.is_none());
    }

    #[test]
    fn test_primary_currency_displays_in_secondary() {
        // When the economy item IS the primary currency,
        // it should display its value in the secondary currency
        let _primary_currency = CurrencyInfo {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image_url: "https://web.poecdn.com/image/divine.png".to_string(),
        };

        let secondary_currency = CurrencyInfo {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image_url: "https://web.poecdn.com/image/chaos.png".to_string(),
        };

        let primary_value = 1.0; // 1 Divine = 1 Divine (in primary terms)
        let secondary_rate = 42.86; // 1 Divine = 42.86 Chaos
        let tertiary_rate = 1836.0;

        // Build display value for the primary currency itself
        let display_value = CurrencyExchangeRate::build_display_value(
            primary_value,
            CurrencyTier::Secondary, // Should use Secondary tier
            secondary_rate,
            tertiary_rate,
            &secondary_currency, // Should display in secondary currency
        );

        assert_eq!(display_value.tier, CurrencyTier::Secondary);
        assert_eq!(display_value.currency_id, "chaos");
        assert_eq!(display_value.currency_name, "Chaos Orb");
        // 1 Divine = 42.86 Chaos
        assert!((display_value.value - 42.86).abs() < 0.01);
        assert!(!display_value.inverted);
    }

    #[test]
    fn test_secondary_currency_displays_in_primary() {
        // When the economy item IS the secondary currency,
        // it should display its value in the primary currency
        let primary_currency = CurrencyInfo {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image_url: "https://web.poecdn.com/image/divine.png".to_string(),
        };

        let _secondary_currency = CurrencyInfo {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image_url: "https://web.poecdn.com/image/chaos.png".to_string(),
        };

        // Chaos Orb's primary_value in terms of Divine
        let primary_value = 1.0 / 42.86; // ~0.0233 Divine per Chaos
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        // Build display value for the secondary currency itself
        let display_value = CurrencyExchangeRate::build_display_value(
            primary_value,
            CurrencyTier::Primary, // Should use Primary tier
            secondary_rate,
            tertiary_rate,
            &primary_currency, // Should display in primary currency
        );

        assert_eq!(display_value.tier, CurrencyTier::Primary);
        assert_eq!(display_value.currency_id, "divine");
        assert_eq!(display_value.currency_name, "Divine Orb");
        // Since value is < 1, it should be inverted
        assert!(display_value.inverted);
        // Should show ~42.86 (inverted from 0.0233)
        assert!((display_value.value - 42.86).abs() < 0.5);
    }

    #[test]
    fn test_tertiary_currency_displays_in_secondary() {
        // When the economy item IS the tertiary currency,
        // it should display its value in the secondary currency
        let secondary_currency = CurrencyInfo {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image_url: "https://web.poecdn.com/image/chaos.png".to_string(),
        };

        let _tertiary_currency = CurrencyInfo {
            id: "exalted".to_string(),
            name: "Exalted Orb".to_string(),
            image_url: "https://web.poecdn.com/image/exalted.png".to_string(),
        };

        // Exalted Orb's primary_value in terms of Divine
        let primary_value = 1.0 / 1836.0; // Very small value in Divine
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        // Build display value for the tertiary currency itself
        let display_value = CurrencyExchangeRate::build_display_value(
            primary_value,
            CurrencyTier::Secondary, // Should use Secondary tier
            secondary_rate,
            tertiary_rate,
            &secondary_currency, // Should display in secondary currency
        );

        assert_eq!(display_value.tier, CurrencyTier::Secondary);
        assert_eq!(display_value.currency_id, "chaos");
        assert_eq!(display_value.currency_name, "Chaos Orb");
        // 1 Exalted in terms of Chaos = (1/1836) * 42.86 = ~0.0233 Chaos
        // Since < 1, should be inverted to show ~42.86
        assert!(display_value.inverted);
    }

    #[test]
    fn test_normal_item_not_affected_by_special_case() {
        // Regular items (not primary/secondary/tertiary) should work as before
        let primary_currency = CurrencyInfo {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image_url: "https://web.poecdn.com/image/divine.png".to_string(),
        };

        let _secondary_currency = CurrencyInfo {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image_url: "https://web.poecdn.com/image/chaos.png".to_string(),
        };

        // Annulment Orb example
        let primary_value = 0.2773;
        let secondary_rate = 42.86;
        let tertiary_rate = 1836.0;

        let config = TierConfig::default();
        let tier = CurrencyExchangeRate::select_optimal_tier(
            primary_value,
            secondary_rate,
            tertiary_rate,
            &config,
        );

        // Should select Primary tier for this value
        assert_eq!(tier, CurrencyTier::Primary);

        let display_value = CurrencyExchangeRate::build_display_value(
            primary_value,
            tier,
            secondary_rate,
            tertiary_rate,
            &primary_currency,
        );

        assert_eq!(display_value.tier, CurrencyTier::Primary);
        assert_eq!(display_value.currency_id, "divine");
        assert_eq!(display_value.currency_name, "Divine Orb");
        // Values < 1 get inverted by finalize_display_value: 1/0.2773 = ~3.606
        assert!((display_value.value - 3.606).abs() < 0.01);
        assert!(display_value.inverted);
    }

    #[test]
    fn test_tertiary_currency_optional() {
        // When there's no tertiary currency (only 2 items in core),
        // the system should handle it gracefully
        let divine_item = CurrencyItem {
            id: "divine".to_string(),
            name: "Divine Orb".to_string(),
            image: "/image/divine.png".to_string(),
            category: "Currency".to_string(),
            details_id: "divine-orb".to_string(),
        };

        let chaos_item = CurrencyItem {
            id: "chaos".to_string(),
            name: "Chaos Orb".to_string(),
            image: "/image/chaos.png".to_string(),
            category: "Currency".to_string(),
            details_id: "chaos-orb".to_string(),
        };

        let core = CurrencyCore {
            items: vec![divine_item.clone(), chaos_item.clone()],
            rates: HashMap::new(),
            primary: "divine".to_string(),
            secondary: "chaos".to_string(),
        };

        let tertiary = core.get_tertiary_currency();
        assert!(tertiary.is_none());
    }

    // ========== FromStr Tests for EconomyType ==========

    #[test]
    fn test_economy_type_from_str_all_variants() {
        assert_eq!(EconomyType::from_str("Currency"), Ok(EconomyType::Currency));
        assert_eq!(
            EconomyType::from_str("Fragments"),
            Ok(EconomyType::Fragments)
        );
        assert_eq!(EconomyType::from_str("Abyss"), Ok(EconomyType::Abyss));
        assert_eq!(
            EconomyType::from_str("UncutGems"),
            Ok(EconomyType::UncutGems)
        );
        assert_eq!(
            EconomyType::from_str("LineageSupportGems"),
            Ok(EconomyType::LineageSupportGems)
        );
        assert_eq!(
            EconomyType::from_str("Essences"),
            Ok(EconomyType::Essences)
        );
        assert_eq!(
            EconomyType::from_str("SoulCores"),
            Ok(EconomyType::SoulCores)
        );
        assert_eq!(EconomyType::from_str("Idols"), Ok(EconomyType::Idols));
        assert_eq!(EconomyType::from_str("Runes"), Ok(EconomyType::Runes));
        assert_eq!(EconomyType::from_str("Ritual"), Ok(EconomyType::Ritual));
        assert_eq!(
            EconomyType::from_str("Expedition"),
            Ok(EconomyType::Expedition)
        );
        assert_eq!(
            EconomyType::from_str("Delirium"),
            Ok(EconomyType::Delirium)
        );
        assert_eq!(EconomyType::from_str("Breach"), Ok(EconomyType::Breach));
    }

    #[test]
    fn test_economy_type_from_str_unknown() {
        assert!(EconomyType::from_str("Unknown").is_err());
        assert!(EconomyType::from_str("").is_err());
        assert!(EconomyType::from_str("currency").is_err()); // case-sensitive
    }

    #[test]
    fn test_economy_type_roundtrip() {
        // Test that as_str() and from_str() are inverses
        for economy_type in EconomyType::all() {
            let str_repr = economy_type.as_str();
            let parsed = EconomyType::from_str(str_repr).unwrap();
            assert_eq!(economy_type, parsed);
        }
    }

    #[test]
    fn test_economy_type_parse_method() {
        // Test the .parse() method works (uses FromStr internally)
        let result: Result<EconomyType, _> = "Currency".parse();
        assert_eq!(result, Ok(EconomyType::Currency));

        let result: Result<EconomyType, _> = "Invalid".parse();
        assert!(result.is_err());
    }
}
