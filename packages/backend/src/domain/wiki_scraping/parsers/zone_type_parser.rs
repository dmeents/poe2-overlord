use crate::domain::wiki_scraping::parsers::base::BaseParser;
use crate::domain::zone_configuration::models::ZoneType;
use scraper::{Html, Selector};

pub struct ZoneTypeParser;

impl ZoneTypeParser {
    /// Derives the zone type from the info-card subheading text.
    ///
    /// | Subheading    | ZoneType  |
    /// |---------------|-----------|
    /// | "Town area"   | Town      |
    /// | "area"        | Campaign  |
    /// | "Map area"    | Map       |
    /// | "Hideout area"| Hideout   |
    /// | other/absent  | Unknown   |
    pub fn parse(document: &Html) -> ZoneType {
        let card_html = match BaseParser::find_primary_info_card_html(document) {
            Some(html) => html,
            None => return ZoneType::Unknown,
        };
        let card_doc = Html::parse_fragment(&card_html);

        let subheading_selector = Selector::parse(".subheading").unwrap();
        let subheading_text = card_doc
            .select(&subheading_selector)
            .next()
            .map(|el| BaseParser::extract_text(&el))
            .unwrap_or_default();

        match subheading_text.as_str() {
            "Town area" => ZoneType::Town,
            "area" => ZoneType::Campaign,
            "Map area" => ZoneType::Map,
            "Hideout area" => ZoneType::Hideout,
            _ => ZoneType::Unknown,
        }
    }
}
