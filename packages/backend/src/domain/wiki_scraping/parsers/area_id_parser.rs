use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::Html;

pub struct AreaIdParser;

impl AreaIdParser {
    pub fn parse(infobox: Option<&Html>) -> Option<String> {
        infobox.and_then(|ib| BaseParser::extract_table_value(ib, "Id"))
    }
}
