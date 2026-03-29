use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct HasWaypointParser;

impl HasWaypointParser {
    /// Parses waypoint status from the full document.
    ///
    /// Waypoint is indicated by the icon title in `div.info-card__header .right span[title]`:
    /// - "Waypoint" → has waypoint
    /// - "Town Hub" → is a town (towns always have waypoints)
    /// - "No Waypoint" or absent → no waypoint
    pub fn parse(document: &Html) -> bool {
        let card_html = match BaseParser::find_primary_info_card_html(document) {
            Some(html) => html,
            None => return false,
        };
        let card_doc = Html::parse_fragment(&card_html);

        let span_selector = Selector::parse(".info-card__header .right span[title]").unwrap();
        if let Some(span) = card_doc.select(&span_selector).next() {
            if let Some(title) = span.value().attr("title") {
                return matches!(title, "Waypoint" | "Town Hub");
            }
        }

        false
    }
}
