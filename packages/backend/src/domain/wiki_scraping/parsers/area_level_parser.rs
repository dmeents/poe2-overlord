use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::Html;

pub struct AreaLevelParser;

impl AreaLevelParser {
    pub fn parse(infobox: Option<&Html>) -> Option<u32> {
        infobox
            .and_then(|ib| BaseParser::extract_table_value(ib, "Area level"))
            .and_then(|s| BaseParser::parse_number(&s))
    }
}
