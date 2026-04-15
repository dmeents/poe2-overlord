use crate::domain::wiki_scraping::{
    models::WikiZoneData, parser::WikiParser, repository::WikiRepository,
    traits::WikiScrapingService, url_utils,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::{error, info};
use std::sync::Arc;

pub struct WikiScrapingServiceImpl {
    repository: Arc<WikiRepository>,
}

impl WikiScrapingServiceImpl {
    pub fn new() -> AppResult<Self> {
        let repository = WikiRepository::new()?;
        Ok(Self {
            repository: Arc::new(repository),
        })
    }
}

#[async_trait]
impl WikiScrapingService for WikiScrapingServiceImpl {
    async fn fetch_zone_data(&self, zone_name: &str) -> AppResult<WikiZoneData> {
        let wiki_url = url_utils::get_wiki_url(zone_name);
        info!("Fetching wiki data for '{zone_name}' from: {wiki_url}");

        let html_content = match self.repository.fetch_page(zone_name).await {
            Ok(content) => {
                info!(
                    "Successfully fetched HTML content for '{}' ({} bytes)",
                    zone_name,
                    content.len()
                );
                content
            }
            Err(e) => {
                error!("Failed to fetch HTML content for '{zone_name}': {e}");
                return Err(e);
            }
        };

        match WikiParser::parse_zone_data(zone_name, &html_content, &wiki_url) {
            Ok(zone_data) => {
                info!("Successfully parsed wiki data for '{zone_name}'");
                Ok(zone_data)
            }
            Err(e) => {
                error!("Failed to parse zone data for '{zone_name}': {e}");
                Err(e)
            }
        }
    }
}
