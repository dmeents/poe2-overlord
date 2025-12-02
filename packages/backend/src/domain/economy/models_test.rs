#[cfg(test)]
mod tests {
    use crate::domain::economy::models::*;
    use std::collections::HashMap;

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
        assert_eq!(inverted, false);
    }

    #[test]
    fn test_finalize_display_value_with_inversion() {
        let (value, inverted) = CurrencyExchangeRate::finalize_display_value(0.0592);
        assert!((value - 16.89).abs() < 0.1);
        assert_eq!(inverted, true);
    }

    #[test]
    fn test_finalize_display_value_edge_case_exactly_one() {
        let (value, inverted) = CurrencyExchangeRate::finalize_display_value(1.0);
        assert_eq!(value, 1.0);
        assert_eq!(inverted, false);
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

        let core = CurrencyCore {
            items: vec![
                divine_item.clone(),
                chaos_item.clone(),
                exalted_item.clone(),
            ],
            rates: HashMap::new(),
            primary: "divine".to_string(),
            secondary: "chaos".to_string(),
        };

        let tertiary = core.get_tertiary_currency();
        assert!(tertiary.is_some());
        assert_eq!(tertiary.unwrap().id, "exalted");

        let core2 = CurrencyCore {
            items: vec![divine_item, chaos_item, exalted_item],
            rates: HashMap::new(),
            primary: "divine".to_string(),
            secondary: "exalted".to_string(),
        };
        let tertiary2 = core2.get_tertiary_currency();
        assert!(tertiary2.is_some());
        assert_eq!(tertiary2.unwrap().id, "chaos");
    }
}
