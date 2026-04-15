use crate::domain::wiki_scraping::models::WikiZoneData;
use crate::domain::wiki_scraping::parsers::{
    ActParser, AreaLevelParser, BossesParser, ConnectedZonesParser, DescriptionParser,
    HasWaypointParser, ImageUrlParser, InfoboxParser, IsTownParser, NpcsParser,
    PointsOfInterestParser, ZoneTypeParser,
};
use crate::errors::{AppError, AppResult};
use log::{debug, info, warn};
use scraper::Html;

pub struct WikiParser;

impl WikiParser {
    pub fn parse_zone_data(
        zone_name: &str,
        html_content: &str,
        wiki_url: &str,
    ) -> AppResult<WikiZoneData> {
        let document = Html::parse_document(html_content);

        if InfoboxParser::is_redirect_page(&document) {
            return Err(AppError::internal_error(
                "parse_zone_data",
                &format!("Zone '{zone_name}' redirects to another page"),
            ));
        }

        let mut zone_data = WikiZoneData::new(zone_name.to_string(), wiki_url.to_string());
        let infobox = InfoboxParser::extract(&document);

        if infobox.is_some() {
            info!("Found infobox for zone '{zone_name}'");
        } else {
            info!("No infobox found for zone '{zone_name}'");
        }

        zone_data.act = ActParser::parse(infobox.as_ref(), &document);
        zone_data.area_level = AreaLevelParser::parse(infobox.as_ref());
        zone_data.is_town = IsTownParser::parse(&document);
        zone_data.has_waypoint = HasWaypointParser::parse(&document);
        zone_data.zone_type = ZoneTypeParser::parse(&document);
        zone_data.connected_zones = ConnectedZonesParser::parse(infobox.as_ref(), &document);
        zone_data.bosses = BossesParser::parse(infobox.as_ref(), &document);
        zone_data.npcs = NpcsParser::parse(&document);
        zone_data.description = DescriptionParser::parse(&document);
        zone_data.points_of_interest = PointsOfInterestParser::parse(&document);
        zone_data.image_url = ImageUrlParser::parse(&document);

        // Diagnostics: log which parsers returned nothing
        if zone_data.act == 0 {
            debug!("'{zone_name}': ActParser returned 0");
        }
        if zone_data.area_level.is_none() {
            debug!("'{zone_name}': AreaLevelParser returned None");
        }
        if zone_data.connected_zones.is_empty() {
            debug!("'{zone_name}': ConnectedZonesParser returned empty");
        }
        if zone_data.bosses.is_empty() {
            debug!("'{zone_name}': BossesParser returned empty");
        }
        if zone_data.npcs.is_empty() {
            debug!("'{zone_name}': NpcsParser returned empty");
        }
        if zone_data.description.is_none() {
            debug!("'{zone_name}': DescriptionParser returned None");
        }

        // Warn when infobox was found but most parsers came back empty
        if infobox.is_some() {
            let empty_count = [
                zone_data.act == 0,
                zone_data.area_level.is_none(),
                zone_data.connected_zones.is_empty(),
                zone_data.bosses.is_empty(),
                zone_data.npcs.is_empty(),
                zone_data.description.is_none(),
            ]
            .iter()
            .filter(|&&empty| empty)
            .count();

            if empty_count >= 3 {
                warn!(
                    "'{zone_name}': infobox found but {empty_count}/6 parsers returned empty — wiki structure may have changed"
                );
            }
        }

        // Validate that we extracted meaningful data
        // At minimum, we should have either: act info, area_level, or connected zones
        let has_basic_data = zone_data.act > 0
            || zone_data.area_level.is_some()
            || !zone_data.connected_zones.is_empty();

        if !has_basic_data && infobox.is_some() {
            // We found an infobox but couldn't extract any basic data - warn but continue
            info!(
                "Warning: Limited data extracted for '{zone_name}'. Wiki page structure may have changed."
            );
        }

        info!(
            "Extracted wiki data for '{}': Act {}, {} bosses, {} NPCs, {} connected zones",
            zone_name,
            zone_data.act,
            zone_data.bosses.len(),
            zone_data.npcs.len(),
            zone_data.connected_zones.len()
        );

        Ok(zone_data)
    }
}
