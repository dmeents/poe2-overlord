use crate::domain::wiki_scraping::models::WikiZoneData;
use crate::domain::wiki_scraping::parsers::{
    ActParser, AreaIdParser, AreaLevelParser, BossesParser, ConnectedZonesParser,
    DescriptionParser, HasWaypointParser, ImageUrlParser, InfoboxParser, IsTownParser,
    MonstersParser, NpcsParser, PointsOfInterestParser,
};
use crate::errors::{AppError, AppResult};
use log::info;
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
                &format!("Zone '{}' redirects to another page", zone_name),
            ));
        }

        let mut zone_data = WikiZoneData::new(zone_name.to_string(), wiki_url.to_string());
        let infobox = InfoboxParser::extract(&document);

        if infobox.is_some() {
            info!("Found infobox for zone '{}'", zone_name);
        } else {
            info!("No infobox found for zone '{}'", zone_name);
        }

        zone_data.area_id = AreaIdParser::parse(infobox.as_ref());
        zone_data.act = ActParser::parse(infobox.as_ref(), &document);
        zone_data.area_level = AreaLevelParser::parse(infobox.as_ref());
        zone_data.is_town = IsTownParser::parse(infobox.as_ref());
        zone_data.has_waypoint = HasWaypointParser::parse(infobox.as_ref());
        zone_data.connected_zones = ConnectedZonesParser::parse(infobox.as_ref(), &document);
        zone_data.bosses = BossesParser::parse(&document);
        zone_data.monsters = MonstersParser::parse(&document);
        zone_data.npcs = NpcsParser::parse(&document);
        zone_data.description = DescriptionParser::parse(&document);
        zone_data.points_of_interest = PointsOfInterestParser::parse(&document);
        zone_data.image_url = ImageUrlParser::parse(&document);

        info!(
            "Extracted wiki data for '{}': Act {}, {} bosses, {} monsters, {} NPCs, {} connected zones",
            zone_name,
            zone_data.act,
            zone_data.bosses.len(),
            zone_data.monsters.len(),
            zone_data.npcs.len(),
            zone_data.connected_zones.len()
        );

        Ok(zone_data)
    }
}
