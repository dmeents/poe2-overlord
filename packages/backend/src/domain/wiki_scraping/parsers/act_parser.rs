use crate::domain::wiki_scraping::parsers::base::BaseParser;
use log::debug;
use scraper::{Html, Selector};

pub struct ActParser;

impl ActParser {
    pub fn parse(infobox: Option<&Html>, document: &Html) -> u32 {
        if let Some(infobox) = infobox {
            if let Some(act) = Self::parse_from_infobox(infobox) {
                return act;
            }
        }

        Self::parse_from_page_text(document).unwrap_or(0)
    }

    fn parse_from_infobox(infobox: &Html) -> Option<u32> {
        BaseParser::extract_table_value(infobox, "Act").and_then(|s| BaseParser::parse_number(&s))
    }

    fn parse_from_page_text(document: &Html) -> Option<u32> {
        let text_selector = Selector::parse("p").unwrap();

        for paragraph in document.select(&text_selector) {
            let text = paragraph.text().collect::<String>().to_lowercase();

            if text.contains("area in act") {
                if let Some(act_pos) = text.find("area in act") {
                    let after_act = &text[act_pos + 11..];
                    if let Ok(num) = after_act
                        .trim()
                        .chars()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u32>()
                    {
                        debug!("Found act {num} in page text");
                        return Some(num);
                    }
                }
            }
        }

        None
    }
}
