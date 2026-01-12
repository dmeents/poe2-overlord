use log::debug;
use scraper::{ElementRef, Html, Selector};

/// Base parser utilities shared across all wiki parsers
pub struct BaseParser;

impl BaseParser {
    /// Extracts text content from an element, trimmed and collapsed
    pub fn extract_text(element: &ElementRef) -> String {
        element.text().collect::<String>().trim().to_string()
    }

    /// Checks if an element is a section heading matching the given name (case-insensitive)
    pub fn is_section_heading(element: &ElementRef, section_name: &str) -> bool {
        let element_name = element.value().name();
        if element_name == "h2" || element_name == "h3" {
            let heading_text = Self::extract_text(element).to_lowercase();
            heading_text.contains(&section_name.to_lowercase())
        } else {
            false
        }
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

    /// Extracts all list items from a specific section
    pub fn extract_section_list_items(document: &Html, section_name: &str) -> Vec<String> {
        let mut items = Vec::new();
        let list_selector = Selector::parse("ul li").unwrap();
        let mut in_section = false;

        for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
            let element_name = element.value().name();

            if element_name == "h2" || element_name == "h3" {
                if Self::is_section_heading(&element, section_name) {
                    in_section = true;
                } else if in_section {
                    break;
                }
            } else if element_name == "ul" && in_section {
                for list_item in element.select(&list_selector) {
                    let text = Self::extract_text(&list_item);
                    if !text.is_empty() {
                        items.push(text);
                    }
                }
            }
        }

        items
    }
}
