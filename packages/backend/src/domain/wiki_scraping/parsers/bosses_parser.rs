use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::Html;

pub struct BossesParser;

impl BossesParser {
    pub fn parse(document: &Html) -> Vec<String> {
        // Try dedicated boss/unique sections first
        let boss_section = BaseParser::extract_section_list_items(document, "boss");
        if !boss_section.is_empty() {
            return boss_section;
        }

        let unique_section = BaseParser::extract_section_list_items(document, "unique");
        if !unique_section.is_empty() {
            return unique_section;
        }

        // Fallback: filter monsters by patterns common to bosses
        BaseParser::extract_section_list_items(document, "monsters")
            .into_iter()
            .filter(|monster| Self::is_likely_boss(monster))
            .collect()
    }

    /// Heuristic to identify if a monster is likely a boss
    fn is_likely_boss(name: &str) -> bool {
        let lower = name.to_lowercase();

        // Pattern: "Name the Title" (e.g., "Hillock the Blacksmith")
        if lower.contains(" the ") {
            return true;
        }

        // Pattern: contains "boss" keyword
        if lower.contains("boss") {
            return true;
        }

        // Pattern: contains "unique" keyword
        if lower.contains("unique") {
            return true;
        }

        // Pattern: major story bosses often have single capitalized names
        // e.g., "Hillock", "Merveil", "Brutus", "Innocence", "Kitava"
        let words: Vec<&str> = name.split_whitespace().collect();
        if words.len() == 1 {
            if let Some(first_char) = name.chars().next() {
                // Single proper noun is likely a named boss
                if first_char.is_uppercase() && name.len() > 4 {
                    return true;
                }
            }
        }

        false
    }
}
