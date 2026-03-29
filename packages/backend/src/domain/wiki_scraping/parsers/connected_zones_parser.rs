use crate::domain::wiki_scraping::parsers::base::BaseParser;
use log::debug;
use scraper::{Html, Selector};

pub struct ConnectedZonesParser;

impl ConnectedZonesParser {
    pub fn parse(infobox: Option<&Html>, document: &Html) -> Vec<String> {
        if let Some(infobox) = infobox {
            let zones = Self::parse_from_infobox(infobox);
            if !zones.is_empty() {
                return zones;
            }
        }

        Self::parse_from_page_text(document)
    }

    fn parse_from_infobox(infobox: &Html) -> Vec<String> {
        let row_selector = Selector::parse("tr").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        for row in infobox.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td, th").unwrap()).collect();
            if cells.len() >= 2 {
                let first_cell = BaseParser::extract_text(&cells[0]);
                let first_cell_lower = first_cell.to_lowercase();

                if first_cell_lower == "connections" {
                    let zones: Vec<String> = cells[1]
                        .select(&link_selector)
                        .map(|link| BaseParser::extract_text(&link))
                        .filter(|zone| !zone.is_empty())
                        .collect();

                    return zones;
                }
            }
        }

        Vec::new()
    }

    fn parse_from_page_text(document: &Html) -> Vec<String> {
        const PATTERNS: &[&str] = &[
            "connected to",
            "connects to",
            "leads to",
            "adjacent to",
            "accessed from",
        ];

        let text_selector = Selector::parse("p").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        for paragraph in document.select(&text_selector) {
            // Use original text for extraction; lowercase only for pattern matching
            let text = paragraph.text().collect::<String>();
            let text_lower = text.to_lowercase();

            for &pattern in PATTERNS {
                if let Some(pattern_pos) = text_lower.find(pattern) {
                    // Prefer <a> links that appear after the pattern position
                    // — links are more precise than comma-splitting raw text
                    let links: Vec<String> = paragraph
                        .select(&link_selector)
                        .filter_map(|a| {
                            let link_text = BaseParser::extract_text(&a);
                            if !link_text.is_empty() {
                                let link_lower = link_text.to_lowercase();
                                if let Some(link_pos) = text_lower.find(&link_lower) {
                                    if link_pos > pattern_pos {
                                        return Some(link_text);
                                    }
                                }
                            }
                            None
                        })
                        .collect();

                    if !links.is_empty() {
                        debug!(
                            "Found {} connected zones via '{}' pattern (links)",
                            links.len(),
                            pattern
                        );
                        return links;
                    }

                    // Fallback: comma-split from ORIGINAL (case-preserved) text
                    let after_pattern = text[pattern_pos + pattern.len()..].trim_start();
                    let zone_text = if let Some(period_pos) = after_pattern.find('.') {
                        &after_pattern[..period_pos]
                    } else {
                        after_pattern
                    };

                    let zones: Vec<String> = zone_text
                        .split(',')
                        .flat_map(|part| part.split(" and "))
                        .map(|zone| zone.trim().to_string())
                        .filter(|zone| !zone.is_empty())
                        .collect();

                    if !zones.is_empty() {
                        debug!(
                            "Found {} connected zones via '{}' pattern (text split)",
                            zones.len(),
                            pattern
                        );
                        return zones;
                    }
                }
            }
        }

        Vec::new()
    }
}
