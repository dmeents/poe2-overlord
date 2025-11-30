use crate::domain::wiki_scraping::models::WikiZoneData;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait WikiScrapingService: Send + Sync {
    async fn fetch_zone_data(&self, zone_name: &str) -> AppResult<WikiZoneData>;
}
