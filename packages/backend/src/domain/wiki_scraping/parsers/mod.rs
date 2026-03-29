pub mod base;
pub mod infobox_parser;

// Field-specific parsers (one per JSON field)
pub mod act_parser;
pub mod area_level_parser;
pub mod bosses_parser;
pub mod connected_zones_parser;
pub mod description_parser;
pub mod has_waypoint_parser;
pub mod image_url_parser;
pub mod is_town_parser;
pub mod npcs_parser;
pub mod points_of_interest_parser;
pub mod zone_type_parser;

// Test modules
#[cfg(test)]
pub mod act_parser_test;
#[cfg(test)]
pub mod area_level_parser_test;
#[cfg(test)]
pub mod base_test;
#[cfg(test)]
pub mod bosses_parser_test;
#[cfg(test)]
pub mod connected_zones_parser_test;
#[cfg(test)]
pub mod description_parser_test;
#[cfg(test)]
pub mod has_waypoint_parser_test;
#[cfg(test)]
pub mod image_url_parser_test;
#[cfg(test)]
pub mod infobox_parser_test;
#[cfg(test)]
pub mod is_town_parser_test;
#[cfg(test)]
pub mod npcs_parser_test;
#[cfg(test)]
pub mod points_of_interest_parser_test;
#[cfg(test)]
pub mod zone_type_parser_test;

// Re-export parsers for convenience
pub use base::BaseParser;
pub use infobox_parser::InfoboxParser;

pub use act_parser::ActParser;
pub use area_level_parser::AreaLevelParser;
pub use bosses_parser::BossesParser;
pub use connected_zones_parser::ConnectedZonesParser;
pub use description_parser::DescriptionParser;
pub use has_waypoint_parser::HasWaypointParser;
pub use image_url_parser::ImageUrlParser;
pub use is_town_parser::IsTownParser;
pub use npcs_parser::NpcsParser;
pub use points_of_interest_parser::PointsOfInterestParser;
pub use zone_type_parser::ZoneTypeParser;
