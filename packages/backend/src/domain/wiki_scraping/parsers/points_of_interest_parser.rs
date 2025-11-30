use crate::domain::wiki_scraping::parsers::base::BaseParser;
use scraper::{Html, Selector};

pub struct PointsOfInterestParser;

impl PointsOfInterestParser {
    pub fn parse(document: &Html) -> Vec<String> {
        let mut points = Vec::new();
        let list_selector = Selector::parse("ul li").unwrap();
        let mut in_poi_section = false;

        for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
            let element_name = element.value().name();

            if element_name == "h2" || element_name == "h3" {
                if BaseParser::is_section_heading(&element, "points of interest") {
                    in_poi_section = true;
                } else if in_poi_section {
                    break;
                }
            } else if element_name == "ul" && in_poi_section {
                for list_item in element.select(&list_selector) {
                    let text = BaseParser::extract_text(&list_item);

                    if let Some(colon_pos) = text.find(':') {
                        let poi_name = text[..colon_pos].trim().to_string();
                        if !poi_name.is_empty() {
                            points.push(poi_name);
                        }
                    } else if !text.is_empty() && text.starts_with('[') {
                        points.push(text);
                    }
                }
                break;
            }
        }

        points
    }
}
