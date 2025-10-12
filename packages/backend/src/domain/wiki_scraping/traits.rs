use crate::domain::wiki_scraping::models::WikiZoneData;
use crate::errors::AppResult;
use async_trait::async_trait;

/// Trait for fetching zone data from the PoE2 wiki
#[async_trait]
pub trait WikiScrapingService: Send + Sync {
    /// Fetches zone data from the wiki for a given zone name
    async fn fetch_zone_data(&self, zone_name: &str) -> AppResult<WikiZoneData>;
    
    /// Checks if zone data should be refreshed based on last update time
    fn should_refresh(&self, last_updated: chrono::DateTime<chrono::Utc>) -> bool;
    
    /// Gets the wiki URL for a zone name
    fn get_wiki_url(&self, zone_name: &str) -> String;
}
