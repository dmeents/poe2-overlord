use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct DescriptionParser;

impl DescriptionParser {
    pub fn parse(document: &Html) -> Option<String> {
        let italic_selector = Selector::parse("i, em").unwrap();

        for italic_element in document.select(&italic_selector) {
            let text = BaseParser::extract_text(&italic_element);

            if text.len() > 10 {
                return Some(text);
            }
        }

        None
    }
}
