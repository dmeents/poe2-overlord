use crate::domain::wiki_scraping::{
    models::WikiZoneData, parser::WikiParser, repository::WikiRepositoryImpl,
    traits::WikiScrapingService,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{debug, error, info};
use std::sync::Arc;

/// Service for scraping zone data from the PoE2 wiki
pub struct WikiScrapingServiceImpl {
    repository: Arc<WikiRepositoryImpl>,
}

impl WikiScrapingServiceImpl {
    /// Creates a new wiki scraping service
    pub fn new() -> Self {
        Self {
            repository: Arc::new(WikiRepositoryImpl::new()),
        }
    }
}

#[async_trait]
impl WikiScrapingService for WikiScrapingServiceImpl {
    /// Fetches zone data from the wiki for a given zone name
    async fn fetch_zone_data(&self, zone_name: &str) -> AppResult<WikiZoneData> {
        let wiki_url = self.get_wiki_url(zone_name);
        info!("Fetching wiki data from: {}", wiki_url);

        // Fetch the HTML content
        let html_content = match self.repository.fetch_page(zone_name).await {
            Ok(content) => {
                info!(
                    "Successfully fetched HTML content for '{}' ({} bytes)",
                    zone_name,
                    content.len()
                );

                // Log a preview of the raw HTML for debugging
                debug!("=== RAW HTML PREVIEW FOR '{}' ===", zone_name);
                let preview_len = content.len().min(500);
                debug!("First {} chars: {}", preview_len, &content[..preview_len]);
                debug!("=====================================");

                content
            }
            Err(e) => {
                error!("Failed to fetch HTML content for '{}': {}", zone_name, e);
                return Err(e);
            }
        };

        // Parse the content
        match WikiParser::parse_zone_data(zone_name, &html_content, &wiki_url) {
            Ok(zone_data) => {
                info!(
                    "=== WIKI DATA EXTRACTED FOR '{}' ===\n\
                     Zone Name: {}\n\
                     Area ID: {:?}\n\
                     Act: {}\n\
                     Area Level: {:?}\n\
                     Is Town: {}\n\
                     Has Waypoint: {}\n\
                     Bosses ({} found): {:?}\n\
                     Monsters ({} found): {:?}\n\
                     NPCs ({} found): {:?}\n\
                     Connected Zones ({} found): {:?}\n\
                     Description: {:?}\n\
                     Points of Interest ({} found): {:?}\n\
                     Image URL: {:?}\n\
                     Wiki URL: {}\n\
                     ====================================",
                    zone_name,
                    zone_data.zone_name,
                    zone_data.area_id,
                    zone_data.act,
                    zone_data.area_level,
                    zone_data.is_town,
                    zone_data.has_waypoint,
                    zone_data.bosses.len(),
                    zone_data.bosses,
                    zone_data.monsters.len(),
                    zone_data.monsters,
                    zone_data.npcs.len(),
                    zone_data.npcs,
                    zone_data.connected_zones.len(),
                    zone_data.connected_zones,
                    zone_data.description,
                    zone_data.points_of_interest.len(),
                    zone_data.points_of_interest,
                    zone_data.image_url,
                    zone_data.wiki_url
                );
                Ok(zone_data)
            }
            Err(e) => {
                error!("Failed to parse zone data for '{}': {}", zone_name, e);
                Err(e)
            }
        }
    }

    /// Checks if zone data should be refreshed based on last update time
    fn should_refresh(&self, last_updated: chrono::DateTime<chrono::Utc>) -> bool {
        let now = chrono::Utc::now();
        let week_ago = now - chrono::Duration::weeks(1);
        last_updated < week_ago
    }

    /// Gets the wiki URL for a zone name
    fn get_wiki_url(&self, zone_name: &str) -> String {
        let formatted_name = zone_name
            .replace(' ', "_")
            .replace("'", "%27")
            .replace("-", "_");
        format!("https://www.poe2wiki.net/wiki/{}", formatted_name)
    }
}

impl Default for WikiScrapingServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}
