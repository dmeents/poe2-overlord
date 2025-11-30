use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::Html;

pub struct BossesParser;

impl BossesParser {
    pub fn parse(document: &Html) -> Vec<String> {
        BaseParser::extract_section_list_items(document, "monsters")
            .into_iter()
            .filter(|monster| monster.contains(" the "))
            .collect()
    }
}
