use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::Html;

pub struct MonstersParser;

impl MonstersParser {
    pub fn parse(document: &Html) -> Vec<String> {
        BaseParser::extract_section_list_items(document, "monsters")
    }
}
