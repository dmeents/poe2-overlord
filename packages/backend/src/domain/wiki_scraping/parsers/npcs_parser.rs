use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct NpcsParser;

impl NpcsParser {
    pub fn parse(document: &Html) -> Vec<String> {
        let mut npcs = Vec::new();
        let list_selector = Selector::parse("ul li").unwrap();
        let mut in_npcs_section = false;

        for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
            let element_name = element.value().name();

            if element_name == "h2" || element_name == "h3" {
                if BaseParser::is_section_heading(&element, "npcs") {
                    in_npcs_section = true;
                } else if in_npcs_section {
                    break;
                }
            } else if element_name == "ul" && in_npcs_section {
                for list_item in element.select(&list_selector) {
                    let text = BaseParser::extract_text(&list_item);
                    let npc_name = BaseParser::split_before_delimiter(&text, &['-', '(']);

                    if !npc_name.is_empty() {
                        npcs.push(npc_name);
                    }
                }
            }
        }

        npcs
    }
}
