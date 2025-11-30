use crate::domain::wiki_scraping::parsers::base::BaseParser;
use log::debug;
use scraper::{Html, Selector};

pub struct ConnectedZonesParser;

impl ConnectedZonesParser {
    pub fn parse(infobox: Option<&Html>, document: &Html) -> Vec<String> {
        if let Some(infobox) = infobox {
            let zones = Self::parse_from_infobox(infobox);
            if !zones.is_empty() {
                return zones;
            }
        }

        Self::parse_from_page_text(document)
    }

    fn parse_from_infobox(infobox: &Html) -> Vec<String> {
        let row_selector = Selector::parse("tr").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        for row in infobox.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td, th").unwrap()).collect();
            if cells.len() >= 2 {
                let first_cell = BaseParser::extract_text(&cells[0]);
                let first_cell_lower = first_cell.to_lowercase();

                if first_cell_lower == "connections" {
                    let zones: Vec<String> = cells[1]
                        .select(&link_selector)
                        .map(|link| BaseParser::extract_text(&link))
                        .filter(|zone| !zone.is_empty())
                        .collect();

                    return zones;
                }
            }
        }

        Vec::new()
    }

    fn parse_from_page_text(document: &Html) -> Vec<String> {
        let text_selector = Selector::parse("p").unwrap();

        for paragraph in document.select(&text_selector) {
            let text = paragraph.text().collect::<String>();

            if text.contains("connected to") {
                if let Some(connected_pos) = text.find("connected to") {
                    let after_connected = &text[connected_pos + 12..].trim();

                    let zone_text = if let Some(period_pos) = after_connected.find('.') {
                        &after_connected[..period_pos]
                    } else {
                        after_connected
                    };

                    let zones: Vec<String> = zone_text
                        .split(',')
                        .flat_map(|part| part.split(" and "))
                        .map(|zone| zone.trim().to_string())
                        .filter(|zone| !zone.is_empty())
                        .collect();

                    if !zones.is_empty() {
                        debug!("Found {} connected zones in page text", zones.len());
                        return zones;
                    }
                }
            }
        }

        Vec::new()
    }
}
