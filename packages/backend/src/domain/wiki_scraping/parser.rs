use crate::domain::wiki_scraping::models::WikiZoneData;
use crate::errors::{AppError, AppResult};
use log::{debug, info};
use scraper::{Html, Selector};

/// HTML parser for extracting zone data from PoE2 wiki pages
pub struct WikiParser;

impl WikiParser {
    /// Parses HTML content to extract zone data
    pub fn parse_zone_data(
        zone_name: &str,
        html_content: &str,
        wiki_url: &str,
    ) -> AppResult<WikiZoneData> {
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

            // Log infobox HTML for debugging
            debug!("=== INFOBOX HTML FOR '{}' ===", zone_name);
            let infobox_html = infobox.html();
            let preview_len = infobox_html.len().min(1000);
            debug!("{}", &infobox_html[..preview_len]);
            if infobox_html.len() > 1000 {
                debug!("... (truncated {} more chars)", infobox_html.len() - 1000);
            }
            debug!("==============================");

            zone_data.area_id = Self::extract_area_id(&infobox);
            debug!("Extracted area_id: {:?}", zone_data.area_id);

            zone_data.act = Self::extract_act(&infobox).unwrap_or_else(|| {
                debug!("Act not found in infobox, trying fallback extraction from page text");
                Self::extract_act_fallback(&document).unwrap_or(0)
            });
            debug!("Extracted act: {}", zone_data.act);

            zone_data.area_level = Self::extract_area_level(&infobox).or_else(|| {
                debug!("Area level not found in infobox, trying fallback extraction");
                Self::extract_area_level_fallback(&document)
            });
            debug!("Extracted area_level: {:?}", zone_data.area_level);

            zone_data.is_town = Self::extract_is_town(&infobox);
            debug!("Extracted is_town: {}", zone_data.is_town);

            zone_data.has_waypoint = Self::extract_has_waypoint(&infobox);
            debug!("Extracted has_waypoint: {}", zone_data.has_waypoint);

            zone_data.connected_zones = Self::extract_connected_zones(&infobox);
            if zone_data.connected_zones.is_empty() {
                debug!("No connections found in infobox, trying fallback extraction");
                zone_data.connected_zones = Self::extract_connected_zones_fallback(&document);
            }
            debug!("Extracted connected_zones: {:?}", zone_data.connected_zones);

            info!(
                "Extracted from infobox: act={}, level={:?}, town={}, waypoint={}",
                zone_data.act, zone_data.area_level, zone_data.is_town, zone_data.has_waypoint
            );
        } else {
            info!("No infobox found for zone '{}'", zone_name);
        }

        // Extract additional data from the page content
        zone_data.bosses = Self::extract_bosses(&document);
        debug!(
            "Extracted {} bosses: {:?}",
            zone_data.bosses.len(),
            zone_data.bosses
        );

        zone_data.monsters = Self::extract_monsters(&document);
        debug!(
            "Extracted {} monsters: {:?}",
            zone_data.monsters.len(),
            zone_data.monsters
        );

        zone_data.npcs = Self::extract_npcs(&document);
        debug!(
            "Extracted {} NPCs: {:?}",
            zone_data.npcs.len(),
            zone_data.npcs
        );

        zone_data.description = Self::extract_description(&document);
        debug!("Extracted description: {:?}", zone_data.description);

        zone_data.points_of_interest = Self::extract_points_of_interest(&document);
        debug!(
            "Extracted {} points of interest: {:?}",
            zone_data.points_of_interest.len(),
            zone_data.points_of_interest
        );

        zone_data.image_url = Self::extract_image_url(&document);
        debug!("Extracted image_url: {:?}", zone_data.image_url);

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
            "table",
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
        let result = Self::extract_table_value(infobox, "Id");
        debug!("extract_area_id: Looking for 'Id' key, found: {:?}", result);
        result
    }

    /// Extracts act number from infobox
    fn extract_act(infobox: &Html) -> Option<u32> {
        let raw_value = Self::extract_table_value(infobox, "Act");
        debug!(
            "extract_act: Looking for 'Act' key, found raw value: {:?}",
            raw_value
        );
        let parsed = raw_value.and_then(|s| {
            // Try to parse the value, handling potential whitespace or extra characters
            let trimmed = s.trim();
            // Sometimes the value might have extra text, extract just the number
            trimmed
                .split_whitespace()
                .next()
                .and_then(|num_str| num_str.parse().ok())
        });
        debug!("extract_act: Parsed to: {:?}", parsed);
        parsed
    }

    /// Extracts area level from infobox
    fn extract_area_level(infobox: &Html) -> Option<u32> {
        let raw_value = Self::extract_table_value(infobox, "Area level");
        debug!(
            "extract_area_level: Looking for 'Area level' key, found raw value: {:?}",
            raw_value
        );
        let parsed = raw_value.and_then(|s| {
            // Try to parse the value, handling potential whitespace or extra characters
            let trimmed = s.trim();
            // Sometimes the value might have extra text, extract just the number
            trimmed
                .split_whitespace()
                .next()
                .and_then(|num_str| num_str.parse().ok())
        });
        debug!("extract_area_level: Parsed to: {:?}", parsed);
        parsed
    }

    /// Fallback method to extract act number from page description text
    /// Looks for patterns like "is an area in Act 1" or "Act 1 area"
    fn extract_act_fallback(document: &Html) -> Option<u32> {
        let text_selector = Selector::parse("p").unwrap();

        for paragraph in document.select(&text_selector) {
            let text = paragraph.text().collect::<String>().to_lowercase();

            // Look for "area in act X" or "act X" patterns
            if text.contains("area in act") {
                // Extract the number after "act"
                if let Some(act_pos) = text.find("area in act") {
                    let after_act = &text[act_pos + 11..]; // Skip "area in act"
                    if let Some(num_str) = after_act
                        .trim()
                        .chars()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<u32>()
                        .ok()
                    {
                        debug!("extract_act_fallback: Found act {} in page text", num_str);
                        return Some(num_str);
                    }
                }
            }
        }

        debug!("extract_act_fallback: No act found in page text");
        None
    }

    /// Fallback method to extract area level from page description
    fn extract_area_level_fallback(_document: &Html) -> Option<u32> {
        // Could be extended if needed, but typically area level is in the infobox
        debug!("extract_area_level_fallback: No fallback implemented yet");
        None
    }

    /// Extracts town status from infobox
    fn extract_is_town(infobox: &Html) -> bool {
        // Check if the infobox contains "Town area" text
        let root_selector = Selector::parse("*").unwrap();
        let infobox_text: String = infobox
            .select(&root_selector)
            .flat_map(|el| el.text())
            .collect();

        if infobox_text.contains("Town area") {
            debug!("extract_is_town: Found 'Town area' label, is_town=true");
            return true;
        }

        // Check if area_id contains "town"
        if let Some(area_id) = Self::extract_area_id(infobox) {
            if area_id.to_lowercase().contains("town") {
                debug!(
                    "extract_is_town: Area ID '{}' contains 'town', is_town=true",
                    area_id
                );
                return true;
            }
        }

        debug!("extract_is_town: No town indicators found, is_town=false");
        false
    }

    /// Extracts waypoint status from infobox
    fn extract_has_waypoint(infobox: &Html) -> bool {
        let raw_value = Self::extract_table_value(infobox, "Waypoint");
        debug!(
            "extract_has_waypoint: Looking for 'Waypoint' key, found raw value: {:?}",
            raw_value
        );
        let result = raw_value
            .map(|s| s.to_lowercase().contains("yes"))
            .unwrap_or(false);
        debug!("extract_has_waypoint: Determined has_waypoint={}", result);
        result
    }

    /// Extracts connected zones from infobox
    fn extract_connected_zones(infobox: &Html) -> Vec<String> {
        let row_selector = Selector::parse("tr").unwrap();
        let link_selector = Selector::parse("a").unwrap();

        debug!("extract_connected_zones: Looking for 'Connections' row");

        // Find the Connections row
        for row in infobox.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td, th").unwrap()).collect();
            if cells.len() >= 2 {
                let first_cell = cells[0].text().collect::<String>();
                let first_cell_trimmed = first_cell.trim();
                let first_cell_lower = first_cell_trimmed.to_lowercase();

                // Case-insensitive comparison
                if first_cell_lower == "connections" {
                    debug!("extract_connected_zones: Found Connections row");

                    // Extract all link texts from the second cell
                    let zones: Vec<String> = cells[1]
                        .select(&link_selector)
                        .map(|link| link.text().collect::<String>().trim().to_string())
                        .filter(|zone| !zone.is_empty())
                        .collect();

                    debug!(
                        "extract_connected_zones: Parsed {} zones: {:?}",
                        zones.len(),
                        zones
                    );
                    return zones;
                }
            }
        }

        debug!("extract_connected_zones: No Connections row found");
        Vec::new()
    }

    /// Fallback method to extract connected zones from page description text
    /// Looks for patterns like "is connected to X, Y, and Z"
    fn extract_connected_zones_fallback(document: &Html) -> Vec<String> {
        let text_selector = Selector::parse("p").unwrap();

        for paragraph in document.select(&text_selector) {
            let text = paragraph.text().collect::<String>();

            // Look for "connected to" pattern
            if text.contains("connected to") {
                if let Some(connected_pos) = text.find("connected to") {
                    let after_connected = &text[connected_pos + 12..].trim(); // Skip "connected to"

                    // Extract zone names - they are typically proper nouns separated by commas and "and"
                    // Example: "connected to The Grim Tangle, Mausoleum of the Praetor, Tomb of the Consort, and Hunting Grounds."

                    // Find the end of the zone list (usually a period or end of sentence)
                    let zone_text = if let Some(period_pos) = after_connected.find('.') {
                        &after_connected[..period_pos]
                    } else {
                        after_connected
                    };

                    // Split by commas and "and"
                    let zones: Vec<String> = zone_text
                        .split(',')
                        .flat_map(|part| part.split(" and "))
                        .map(|zone| zone.trim().to_string())
                        .filter(|zone| !zone.is_empty())
                        .collect();

                    if !zones.is_empty() {
                        debug!(
                            "extract_connected_zones_fallback: Found {} zones in page text: {:?}",
                            zones.len(),
                            zones
                        );
                        return zones;
                    }
                }
            }
        }

        debug!("extract_connected_zones_fallback: No connections found in page text");
        Vec::new()
    }

    /// Extracts bosses from the monsters section
    /// Bosses are typically rare/unique monsters that contain "the" in their name (e.g., "Vargir the Feral Mutt")
    fn extract_bosses(document: &Html) -> Vec<String> {
        let mut bosses = Vec::new();
        let list_selector = Selector::parse("ul li").unwrap();
        let mut in_monsters_section = false;

        for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
            let element_name = element.value().name();

            if element_name == "h2" || element_name == "h3" {
                let heading_text = element.text().collect::<String>().to_lowercase();
                if heading_text.contains("monsters") {
                    in_monsters_section = true;
                } else if in_monsters_section {
                    // We've moved to the next section, stop
                    break;
                }
            } else if element_name == "ul" && in_monsters_section {
                // Look for monsters that appear to be bosses/rares
                // Typically they contain "the" in their name as a title (e.g., "Name the Title")
                for list_item in element.select(&list_selector) {
                    let text = list_item.text().collect::<String>().trim().to_string();
                    if !text.is_empty() && text.contains(" the ") {
                        bosses.push(text);
                    }
                }
                break; // Only process the first list in the Monsters section
            }
        }

        debug!("Extracted {} bosses: {:?}", bosses.len(), bosses);
        bosses
    }

    /// Extracts monsters from the monsters section
    fn extract_monsters(document: &Html) -> Vec<String> {
        let mut monsters = Vec::new();

        // Look for the Monsters section and extract only items from that specific section
        let list_selector = Selector::parse("ul li").unwrap();
        let mut in_monsters_section = false;

        for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
            let element_name = element.value().name();

            if element_name == "h2" || element_name == "h3" {
                let heading_text = element.text().collect::<String>().to_lowercase();
                if heading_text.contains("monsters") {
                    in_monsters_section = true;
                } else if in_monsters_section {
                    // We've moved to the next section, stop
                    break;
                }
            } else if element_name == "ul" && in_monsters_section {
                // Extract monsters from this list only
                for list_item in element.select(&list_selector) {
                    let text = list_item.text().collect::<String>().trim().to_string();
                    if !text.is_empty() {
                        monsters.push(text);
                    }
                }
            }
        }

        debug!("Extracted {} monsters: {:?}", monsters.len(), monsters);
        monsters
    }

    /// Extracts NPCs from the NPCs section
    fn extract_npcs(document: &Html) -> Vec<String> {
        let mut npcs = Vec::new();

        // Look for the NPCs section
        let heading_selector = Selector::parse("h2, h3").unwrap();
        let list_selector = Selector::parse("ul li").unwrap();
        let mut found_npcs_section = false;

        for heading in document.select(&heading_selector) {
            let heading_text = heading.text().collect::<String>().to_lowercase();

            if heading_text.contains("npcs") {
                found_npcs_section = true;
                debug!("Found NPCs section");

                // Get the next sibling elements after this heading until we hit another heading
                // For simplicity, we'll look for list items in the NPCs section
                break;
            }
        }

        if found_npcs_section {
            // Parse NPCs from the list items
            // NPCs are typically in format "Name - Description" or just "Name"
            let mut in_npcs_section = false;

            for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
                let element_name = element.value().name();

                if element_name == "h2" || element_name == "h3" {
                    let heading_text = element.text().collect::<String>().to_lowercase();
                    if heading_text.contains("npcs") {
                        in_npcs_section = true;
                    } else if in_npcs_section {
                        // We've moved to the next section
                        break;
                    }
                } else if element_name == "ul" && in_npcs_section {
                    // Extract NPCs from this list
                    for list_item in element.select(&list_selector) {
                        let text = list_item.text().collect::<String>();

                        // NPC name is typically before the first dash, hyphen, or parenthesis
                        // Find both delimiters and use whichever comes first
                        let dash_pos = text.find('-');
                        let paren_pos = text.find('(');

                        let split_pos = match (dash_pos, paren_pos) {
                            (Some(d), Some(p)) => Some(d.min(p)),
                            (Some(d), None) => Some(d),
                            (None, Some(p)) => Some(p),
                            (None, None) => None,
                        };

                        let npc_name = if let Some(pos) = split_pos {
                            text[..pos].trim()
                        } else {
                            text.trim()
                        };

                        if !npc_name.is_empty() {
                            npcs.push(npc_name.to_string());
                            debug!("Found NPC: {}", npc_name);
                        }
                    }
                }
            }
        }

        debug!("Extracted {} NPCs: {:?}", npcs.len(), npcs);
        npcs
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
    /// Looks for entries like "[Miniboss arena] Crop Circle" or "[Notable landmark] Una's Home"
    fn extract_points_of_interest(document: &Html) -> Vec<String> {
        let mut points = Vec::new();
        let list_selector = Selector::parse("ul li").unwrap();
        let mut in_poi_section = false;

        for element in document.select(&Selector::parse("h2, h3, ul").unwrap()) {
            let element_name = element.value().name();

            if element_name == "h2" || element_name == "h3" {
                let heading_text = element.text().collect::<String>().to_lowercase();
                if heading_text.contains("points of interest") {
                    in_poi_section = true;
                } else if in_poi_section {
                    // We've moved to the next section, stop
                    break;
                }
            } else if element_name == "ul" && in_poi_section {
                // Extract points of interest from this list only
                for list_item in element.select(&list_selector) {
                    let text = list_item.text().collect::<String>().trim().to_string();

                    // Extract just the bracketed type and name (before the colon)
                    if let Some(colon_pos) = text.find(':') {
                        let poi_name = text[..colon_pos].trim().to_string();
                        if !poi_name.is_empty() {
                            points.push(poi_name);
                        }
                    } else if !text.is_empty() && text.starts_with('[') {
                        // If there's no colon but it starts with a bracket, include it
                        points.push(text);
                    }
                }
                break; // Only process the first list in the POI section
            }
        }

        debug!(
            "Extracted {} points of interest: {:?}",
            points.len(),
            points
        );
        points
    }

    /// Extracts the zone screenshot/image URL from the page
    /// Looks for images with patterns like "area_screenshot" or in the infobox
    fn extract_image_url(document: &Html) -> Option<String> {
        let img_selector = Selector::parse("img").unwrap();

        // First priority: Look for actual image src attributes
        // This gives us the direct URL to the JPG/PNG file
        for img in document.select(&img_selector) {
            if let Some(src) = img.value().attr("src") {
                // Check if it's a zone screenshot
                if src.contains("area_screenshot") || src.contains("screenshot") {
                    // Convert thumbnail URL to full image URL if needed
                    // Thumbnail URLs look like: /images/thumb/a/ab/Filename.jpg/300px-Filename.jpg
                    // Full URLs look like: /images/a/ab/Filename.jpg
                    let full_url = if src.contains("/thumb/") {
                        // Remove the thumbnail part and the size suffix
                        // Split on "/thumb/" to separate base from the rest
                        if let Some(thumb_pos) = src.find("/thumb/") {
                            let base = &src[..thumb_pos];
                            let after_thumb = &src[thumb_pos + 7..]; // Skip "/thumb/"

                            // The structure after /thumb/ is: path/to/file.jpg/300px-file.jpg
                            // We want to extract just path/to/file.jpg
                            if let Some(last_slash) = after_thumb.rfind('/') {
                                let original_path = &after_thumb[..last_slash];
                                format!("{}/{}", base, original_path)
                            } else {
                                src.to_string()
                            }
                        } else {
                            src.to_string()
                        }
                    } else {
                        src.to_string()
                    };

                    // Make sure it's an absolute URL
                    let absolute_url = if full_url.starts_with("http") {
                        full_url
                    } else if full_url.starts_with("//") {
                        format!("https:{}", full_url)
                    } else {
                        format!("https://www.poe2wiki.net{}", full_url)
                    };

                    debug!("Found direct image URL: {}", absolute_url);
                    return Some(absolute_url);
                }
            }
        }

        debug!("No image URL found");
        None
    }

    /// Helper to extract a value from a table row
    fn extract_table_value(infobox: &Html, key: &str) -> Option<String> {
        let row_selector = Selector::parse("tr").unwrap();
        debug!("extract_table_value: Searching for key '{}'", key);

        let key_lower = key.to_lowercase();

        for row in infobox.select(&row_selector) {
            let cells: Vec<_> = row.select(&Selector::parse("td, th").unwrap()).collect();
            if cells.len() >= 2 {
                let first_cell = cells[0].text().collect::<String>();
                let first_cell_trimmed = first_cell.trim();
                let first_cell_lower = first_cell_trimmed.to_lowercase();

                debug!(
                    "  Found row with key '{}' (comparing to '{}')",
                    first_cell_trimmed, key
                );

                // Case-insensitive comparison
                if first_cell_lower == key_lower {
                    let value = cells[1].text().collect::<String>().trim().to_string();
                    debug!("  MATCH! Extracted value: '{}'", value);
                    // Filter out empty values
                    if !value.is_empty() {
                        return Some(value);
                    }
                }
            }
        }

        debug!("  No match found for key '{}'", key);
        None
    }
}
