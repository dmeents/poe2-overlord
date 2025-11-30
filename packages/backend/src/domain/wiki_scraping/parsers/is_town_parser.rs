use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct IsTownParser;

impl IsTownParser {
    pub fn parse(infobox: Option<&Html>) -> bool {
        let Some(infobox) = infobox else {
            return false;
        };

        // Check if the infobox contains "Town area" text
        let root_selector = Selector::parse("*").unwrap();
        let infobox_text: String = infobox
            .select(&root_selector)
            .flat_map(|el| el.text())
            .collect();

        if infobox_text.contains("Town area") {
            return true;
        }

        // Check if area_id contains "town"
        if let Some(area_id) = BaseParser::extract_table_value(infobox, "Id") {
            if BaseParser::contains_case_insensitive(&area_id, "town") {
                return true;
            }
        }

        false
    }
}
