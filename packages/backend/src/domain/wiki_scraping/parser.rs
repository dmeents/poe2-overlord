use crate::domain::wiki_scraping::models::WikiZoneData;
use crate::errors::{AppError, AppResult};
use log::info;
use scraper::{Html, Selector};

/// HTML parser for extracting zone data from PoE2 wiki pages
pub struct WikiParser;

impl WikiParser {
        /// Parses HTML content to extract zone data
        pub fn parse_zone_data(zone_name: &str, html_content: &str, wiki_url: &str) -> AppResult<WikiZoneData> {
            let document = Html::parse_document(html_content);
            
            // Check if this is a valid zone page
            if Self::is_redirect_page(&document) {
                return Err(AppError::internal_error(
                    "parse_zone_data",
                    &format!("Zone '{}' redirects to another page", zone_name),
                ));
            }

            let mut zone_data = WikiZoneData::new(zone_name.to_string(), wiki_url.to_string());
            
            // Extract data from infobox
            if let Some(infobox) = Self::extract_infobox(&document) {
                info!("Found infobox for zone '{}'", zone_name);
                zone_data.area_id = Self::extract_area_id(&infobox);
                zone_data.act = Self::extract_act(&infobox).unwrap_or(0);
                zone_data.area_level = Self::extract_area_level(&infobox);
                zone_data.is_town = Self::extract_is_town(&infobox);
                zone_data.has_waypoint = Self::extract_has_waypoint(&infobox);
                zone_data.tags = Self::extract_tags(&infobox);
                zone_data.connected_zones = Self::extract_connected_zones(&infobox);
                info!("Extracted from infobox: act={}, level={:?}, town={}, waypoint={}", 
                      zone_data.act, zone_data.area_level, zone_data.is_town, zone_data.has_waypoint);
            } else {
                info!("No infobox found for zone '{}'", zone_name);
            }

            // Extract additional data from the page content
            zone_data.bosses = Self::extract_bosses(&document);
            zone_data.monsters = Self::extract_monsters(&document);
            zone_data.description = Self::extract_description(&document);
            zone_data.points_of_interest = Self::extract_points_of_interest(&document);

            Ok(zone_data)
        }

    /// Checks if the page is a redirect
    fn is_redirect_page(document: &Html) -> bool {
        let title_selector = Selector::parse("title").unwrap();
        if let Some(title) = document.select(&title_selector).next() {
            let title_text = title.text().collect::<String>();
            return title_text.contains("redirect") || title_text.contains("Redirect");
        }
        false
    }

    /// Extracts the infobox table from the document
    fn extract_infobox(document: &Html) -> Option<Html> {
        // Try multiple selectors to find the infobox
        let selectors = [
            "table.infobox",
            "table.wikitable", 
            "table[class*='infobox']",
            "table[class*='wikitable']",
            "table.responsive-table",
            "table"
        ];
        
        for selector_str in &selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let html = element.html();
                    // Check if this looks like an infobox (has key-value pairs)
                    if html.contains("Act") || html.contains("Area level") || html.contains("Id") {
                        return Some(Html::parse_fragment(&html));
                    }
                }
            }
        }
        None
    }

    /// Extracts area ID from infobox
    fn extract_area_id(infobox: &Html) -> Option<String> {
        Self::extract_table_value(infobox, "Id")
    }

    /// Extracts act number from infobox
    fn extract_act(infobox: &Html) -> Option<u32> {
        Self::extract_table_value(infobox, "Act")
            .and_then(|s| s.parse().ok())
    }

    /// Extracts area level from infobox
    fn extract_area_level(infobox: &Html) -> Option<u32> {
        Self::extract_table_value(infobox, "Area level")
            .and_then(|s| s.parse().ok())
    }

    /// Extracts town status from infobox
    fn extract_is_town(infobox: &Html) -> bool {
        Self::extract_table_value(infobox, "Waypoint")
            .map(|s| s.to_lowercase().contains("yes"))
            .unwrap_or(false)
    }

    /// Extracts waypoint status from infobox
    fn extract_has_waypoint(infobox: &Html) -> bool {
        Self::extract_table_value(infobox, "Waypoint")
            .map(|s| s.to_lowercase().contains("yes"))
            .unwrap_or(false)
    }

    /// Extracts tags from infobox
    fn extract_tags(infobox: &Html) -> Vec<String> {
        Self::extract_table_value(infobox, "Tags")
            .map(|s| s.split(',').map(|tag| tag.trim().to_string()).collect())
            .unwrap_or_default()
    }

    /// Extracts connected zones from infobox
    fn extract_connected_zones(infobox: &Html) -> Vec<String> {
        Self::extract_table_value(infobox, "Connections")
            .map(|s| {
                s.split(',')
                    .map(|zone| zone.trim().to_string())
                    .filter(|zone| !zone.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

        /// Extracts bosses from the monsters section
        fn extract_bosses(document: &Html) -> Vec<String> {
            let mut bosses = Vec::new();
            
            // Look for the Monsters section
            let heading_selector = Selector::parse("h2, h3").unwrap();
            let mut found_monsters_section = false;
            
            for heading in document.select(&heading_selector) {
                let heading_text = heading.text().collect::<String>().to_lowercase();
                if heading_text.contains("monsters") {
                    found_monsters_section = true;
                    break;
                }
            }
            
            if found_monsters_section {
                // Look for bold text in the monsters section (bosses are usually bold)
                let bold_selector = Selector::parse("b, strong").unwrap();
                for bold_element in document.select(&bold_selector) {
                    let text = bold_element.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text.len() > 3 {
                        bosses.push(text);
                    }
                }
            }
            
            bosses
        }

        /// Extracts monsters from the monsters section
        fn extract_monsters(document: &Html) -> Vec<String> {
            let mut monsters = Vec::new();
            
            // Look for the Monsters section
            let heading_selector = Selector::parse("h2, h3").unwrap();
            let mut found_monsters_section = false;
            
            for heading in document.select(&heading_selector) {
                let heading_text = heading.text().collect::<String>().to_lowercase();
                if heading_text.contains("monsters") {
                    found_monsters_section = true;
                    break;
                }
            }
            
            if found_monsters_section {
                // Look for list items in the monsters section
                let list_selector = Selector::parse("ul li, ol li").unwrap();
                for list_item in document.select(&list_selector) {
                    let text = list_item.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text.len() > 2 {
                        monsters.push(text);
                    }
                }
            }
            
            monsters
        }

        /// Extracts description from the page
        fn extract_description(document: &Html) -> Option<String> {
            // Look for italic text that might be a description
            let italic_selector = Selector::parse("i, em").unwrap();
            for italic_element in document.select(&italic_selector) {
                let text = italic_element.text().collect::<String>().trim().to_string();
                if !text.is_empty() && text.len() > 10 {
                    return Some(text);
                }
            }
            None
        }

        /// Extracts points of interest from the page
        fn extract_points_of_interest(document: &Html) -> Vec<String> {
            let mut points = Vec::new();
            
            // Look for "Points of interest" section
            let heading_selector = Selector::parse("h2, h3").unwrap();
            let mut found_poi_section = false;
            
            for heading in document.select(&heading_selector) {
                let heading_text = heading.text().collect::<String>().to_lowercase();
                if heading_text.contains("points of interest") {
                    found_poi_section = true;
                    break;
                }
            }
            
            if found_poi_section {
                // Look for list items in the points of interest section
                let list_selector = Selector::parse("ul li, ol li").unwrap();
                for list_item in document.select(&list_selector) {
                    let text = list_item.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text.len() > 3 {
                        points.push(text);
                    }
                }
            }
            
            points
        }

    /// Helper to extract a value from a table row
    fn extract_table_value(infobox: &Html, key: &str) -> Option<String> {
        let row_selector = Selector::parse("tr").unwrap();
        for row in infobox.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td, th").unwrap()).collect();
            if cells.len() >= 2 {
                let first_cell = cells[0].text().collect::<String>();
                if first_cell.trim() == key {
                    return Some(cells[1].text().collect::<String>().trim().to_string());
                }
            }
        }
        None
    }
}
