use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::Html;

pub struct HasWaypointParser;

impl HasWaypointParser {
    pub fn parse(infobox: Option<&Html>) -> bool {
        infobox
            .and_then(|ib| BaseParser::extract_table_value(ib, "Waypoint"))
            .map(|s| BaseParser::contains_case_insensitive(&s, "yes"))
            .unwrap_or(false)
    }
}
