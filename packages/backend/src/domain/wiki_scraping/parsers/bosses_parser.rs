use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct BossesParser;

impl BossesParser {
    /// Parses boss names from the wiki page.
    ///
    /// Priority:
    /// 1. Infobox "Bosses" table row — extracts `<a>` link text from the cell (most reliable)
    /// 2. Dedicated "Bosses" / "Boss Monsters" section in the document body
    /// 3. Dedicated "Unique Monsters" section
    /// 4. Heuristic filter over the "Monsters" section
    pub fn parse(infobox: Option<&Html>, document: &Html) -> Vec<String> {
        // Primary: infobox Bosses table row
        if let Some(infobox) = infobox {
            let bosses = Self::parse_from_infobox(infobox);
            if !bosses.is_empty() {
                return bosses;
            }
        }

        // Secondary: dedicated boss/unique sections
        let boss_section = BaseParser::extract_section_list_items(document, "boss");
        if !boss_section.is_empty() {
            return boss_section;
        }

        let unique_section = BaseParser::extract_section_list_items(document, "unique");
        if !unique_section.is_empty() {
            return unique_section;
        }

        // Fallback: filter monsters by heuristic
        BaseParser::extract_section_list_items(document, "monsters")
            .into_iter()
            .filter(|monster| Self::is_likely_boss(monster))
            .collect()
    }

    /// Extracts boss names from the infobox "Bosses" table row.
    /// Prefers `<a>` link text; falls back to comma-splitting raw cell text.
    fn parse_from_infobox(infobox: &Html) -> Vec<String> {
        let row_selector = Selector::parse("tr").unwrap();
        let th_selector = Selector::parse("th").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        for row in infobox.select(&row_selector) {
            let th_text = row
                .select(&th_selector)
                .next()
                .map(|th| BaseParser::extract_text(&th))
                .unwrap_or_default();

            if th_text.to_lowercase() == "bosses" {
                if let Some(td) = row.select(&td_selector).next() {
                    // Prefer link text (wiki page links to each boss)
                    let links: Vec<String> = td
                        .select(&link_selector)
                        .map(|a| BaseParser::extract_text(&a))
                        .filter(|t| !t.is_empty())
                        .collect();

                    if !links.is_empty() {
                        return links;
                    }

                    // Fallback: comma-split the raw cell text
                    let text = BaseParser::extract_text(&td);
                    if !text.is_empty() {
                        return text
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }
                }
            }
        }

        vec![]
    }

    /// Heuristic to identify if a monster is likely a boss.
    /// Checks for " the " pattern, "boss" keyword, or "unique" keyword.
    fn is_likely_boss(name: &str) -> bool {
        let lower = name.to_lowercase();
        lower.contains(" the ") || lower.contains("boss") || lower.contains("unique")
    }
}
