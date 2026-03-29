use log::debug;
use scraper::{ElementRef, Html, Selector};

/// Base parser utilities shared across all wiki parsers
pub struct BaseParser;

impl BaseParser {
    /// Extracts text content from an element, trimmed and collapsed
    pub fn extract_text(element: &ElementRef) -> String {
        element.text().collect::<String>().trim().to_string()
    }

    /// Checks if an element is a section heading matching the given name (case-insensitive).
    /// Uses `matches_heading_name` for synonym support and `[edit]` stripping.
    pub fn is_section_heading(element: &ElementRef, section_name: &str) -> bool {
        Self::matches_heading_name(element, section_name)
    }

    /// Matches a heading element against a section name, with synonym support and edit-link stripping.
    ///
    /// Handles:
    /// - `[edit]` / `[edit section]` spans appended by MediaWiki
    /// - Common aliases (e.g. "Boss Monsters" → bosses, "NPC List" → npcs)
    /// - Fallback substring match
    pub fn matches_heading_name(element: &ElementRef, section_name: &str) -> bool {
        let tag = element.value().name();
        if tag != "h2" && tag != "h3" {
            return false;
        }

        // Strip edit-link spans added by MediaWiki before comparing text
        let raw = Self::extract_text(element).to_lowercase();
        let clean = raw
            .replace("[edit]", "")
            .replace("[edit section]", "")
            .trim()
            .to_string();

        let needle = section_name.to_lowercase();

        // Exact or substring match on the primary name
        if clean == needle || clean.contains(&needle) {
            return true;
        }

        // Synonym map — normalized section names → accepted aliases
        let synonyms: &[&str] = match needle.as_str() {
            "boss" | "bosses" => &["boss monsters", "unique monsters"],
            "npc" | "npcs" => &["npc list", "non-player characters", "characters"],
            "points_of_interest" | "points of interest" => {
                &["notable locations", "landmarks", "notable areas"]
            }
            _ => &[],
        };

        for synonym in synonyms {
            if clean.contains(synonym) {
                return true;
            }
        }

        false
    }

    /// Extracts list items from a list element
    pub fn extract_list_items(list_element: &ElementRef) -> Vec<String> {
        let list_selector = Selector::parse("li").unwrap();
        list_element
            .select(&list_selector)
            .map(|item| Self::extract_text(&item))
            .filter(|text| !text.is_empty())
            .collect()
    }

    /// Extracts a value from a table row by key (case-insensitive)
    pub fn extract_table_value(table: &Html, key: &str) -> Option<String> {
        let row_selector = Selector::parse("tr").unwrap();
        let cell_selector = Selector::parse("td, th").unwrap();
        let key_lower = key.to_lowercase();

        for row in table.select(&row_selector) {
            let cells: Vec<_> = row.select(&cell_selector).collect();
            if cells.len() >= 2 {
                let first_cell = Self::extract_text(&cells[0]);
                let first_cell_lower = first_cell.to_lowercase();

                if first_cell_lower == key_lower {
                    let value = Self::extract_text(&cells[1]);
                    debug!("Found table value for '{}': '{}'", key, value);
                    if !value.is_empty() {
                        return Some(value);
                    }
                }
            }
        }

        None
    }

    /// Parses a number from a string, extracting just the numeric portion
    pub fn parse_number(text: &str) -> Option<u32> {
        text.split_whitespace()
            .next()
            .and_then(|num_str| num_str.parse().ok())
    }

    /// Checks if text contains a pattern (case-insensitive)
    pub fn contains_case_insensitive(text: &str, pattern: &str) -> bool {
        text.to_lowercase().contains(&pattern.to_lowercase())
    }

    /// Splits text before a delimiter (e.g., extracting "Name" from "Name - Description")
    pub fn split_before_delimiter(text: &str, delimiters: &[char]) -> String {
        for delimiter in delimiters {
            if let Some(pos) = text.find(*delimiter) {
                return text[..pos].trim().to_string();
            }
        }
        text.trim().to_string()
    }

    /// Returns the HTML of the first `div.info-card` whose subheading is NOT "Tooltip".
    /// On pages with multiple info-cards (hideouts, mechanic zones), this selects the
    /// primary zone card and skips tooltip/auxillary cards.
    pub fn find_primary_info_card_html(document: &Html) -> Option<String> {
        let card_selector = Selector::parse("div.info-card").unwrap();
        let subheading_selector = Selector::parse(".subheading").unwrap();

        for card in document.select(&card_selector) {
            let subheading_text = card
                .select(&subheading_selector)
                .next()
                .map(|el| Self::extract_text(&el))
                .unwrap_or_default();

            if subheading_text != "Tooltip" {
                return Some(card.html());
            }
        }
        None
    }

    /// Collects all text items from a list element (ul, ol, or dl).
    fn collect_from_list(list_el: &ElementRef) -> Vec<String> {
        let tag = list_el.value().name();
        let child_selector = if tag == "dl" {
            Selector::parse("dt, dd").unwrap()
        } else {
            Selector::parse("li").unwrap()
        };
        list_el
            .select(&child_selector)
            .map(|item| Self::extract_text(&item))
            .filter(|text| !text.is_empty())
            .collect()
    }

    /// Extracts all list items from a specific named section of the document.
    ///
    /// Uses sibling-walking from the matched heading to correctly handle:
    /// - Lists inside `div`/`section` wrappers (MediaWiki content divs)
    /// - `ol` and `dl` in addition to `ul`
    /// - `figure` elements (skipped)
    /// - Multi-list sections (all lists collected until next heading)
    /// - Same/higher-rank headings as stop markers
    pub fn extract_section_list_items(document: &Html, section_name: &str) -> Vec<String> {
        let heading_selector = Selector::parse("h2, h3").unwrap();
        let list_selector = Selector::parse("ul, ol, dl").unwrap();

        for heading in document.select(&heading_selector) {
            if !Self::matches_heading_name(&heading, section_name) {
                continue;
            }

            let heading_rank = heading.value().name(); // "h2" or "h3"
            let mut items = Vec::new();

            for sibling in heading.next_siblings() {
                let el = match ElementRef::wrap(sibling) {
                    Some(el) => el,
                    None => continue, // skip text nodes
                };

                let tag = el.value().name();

                // Stop at a same-or-higher-rank heading
                if tag == "h2" || (tag == "h3" && heading_rank == "h3") {
                    break;
                }

                // Direct list elements
                if matches!(tag, "ul" | "ol" | "dl") {
                    items.extend(Self::collect_from_list(&el));
                    continue;
                }

                // Descend into div/section wrappers (MediaWiki wraps content in mw-parser-output divs)
                if matches!(tag, "div" | "section") {
                    for list_el in el.select(&list_selector) {
                        items.extend(Self::collect_from_list(&list_el));
                    }
                    continue;
                }

                // Skip figures — they appear between headings and list content on wiki pages
                // Skip tables and other non-list block elements
            }

            if !items.is_empty() {
                return items;
            }
        }

        vec![]
    }
}
