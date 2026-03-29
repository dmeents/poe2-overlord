use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct DescriptionParser;

impl DescriptionParser {
    pub fn parse(document: &Html) -> Option<String> {
        // Primary: targeted flavour text within the primary info-card
        if let Some(card_html) = BaseParser::find_primary_info_card_html(document) {
            let card_doc = Html::parse_fragment(&card_html);
            let flavour_selector = Selector::parse("em[class*='flavour']").unwrap();
            if let Some(el) = card_doc.select(&flavour_selector).next() {
                let text = BaseParser::extract_text(&el);
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }

        // Fallback: first italic/em in the full document with >10 chars
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
