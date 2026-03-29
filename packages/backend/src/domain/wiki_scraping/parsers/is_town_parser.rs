use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct IsTownParser;

impl IsTownParser {
    /// Parses town status from the full document.
    ///
    /// Detection priority:
    /// 1. Subheading text == "Town area" (primary signal)
    /// 2. Icon title == "Town Hub" (header signal)
    /// 3. Infobox Id row contains "_town" (fallback)
    pub fn parse(document: &Html) -> bool {
        let card_html = match BaseParser::find_primary_info_card_html(document) {
            Some(html) => html,
            None => return false,
        };
        let card_doc = Html::parse_fragment(&card_html);

        // Primary: subheading text
        let subheading_selector = Selector::parse(".subheading").unwrap();
        if let Some(subheading) = card_doc.select(&subheading_selector).next() {
            if BaseParser::extract_text(&subheading) == "Town area" {
                return true;
            }
        }

        // Secondary: icon title "Town Hub"
        let span_selector = Selector::parse(".info-card__header .right span[title]").unwrap();
        if let Some(span) = card_doc.select(&span_selector).next() {
            if span.value().attr("title") == Some("Town Hub") {
                return true;
            }
        }

        // Fallback: infobox Id row contains "_town"
        let table_selector = Selector::parse("table").unwrap();
        if let Some(table_el) = card_doc.select(&table_selector).next() {
            let table_html = table_el.html();
            let table_frag = Html::parse_fragment(&table_html);
            if let Some(id_val) = BaseParser::extract_table_value(&table_frag, "Id") {
                if id_val.to_lowercase().contains("_town") {
                    return true;
                }
            }
        }

        false
    }
}
