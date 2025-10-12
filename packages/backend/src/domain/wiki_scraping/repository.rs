use crate::errors::{AppError, AppResult};
use log::error;
use reqwest::Client;
use std::time::Duration;

/// HTTP client wrapper for making requests to the PoE2 wiki
pub struct WikiRepositoryImpl {
    client: Client,
    base_url: String,
}

impl WikiRepositoryImpl {
    /// Creates a new wiki repository with default configuration
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("poe2-overlord/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "https://www.poe2wiki.net/wiki".to_string(),
        }
    }

    /// Fetches the HTML content of a wiki page
    pub async fn fetch_page(&self, zone_name: &str) -> AppResult<String> {
        let url = format!("{}/{}", self.base_url, self.format_zone_name_for_url(zone_name));
        
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to fetch wiki page for zone '{}': {}", zone_name, e);
                AppError::network_error("fetch_wiki_page", &e.to_string())
            })?;

        if !response.status().is_success() {
            return Err(AppError::network_error(
                "fetch_wiki_page",
                &format!("HTTP {}: {}", response.status(), url),
            ));
        }

        let content = response.text().await.map_err(|e| {
            error!("Failed to read response body for zone '{}': {}", zone_name, e);
            AppError::network_error("read_response_body", &e.to_string())
        })?;

        Ok(content)
    }

    /// Formats a zone name for use in wiki URLs
    fn format_zone_name_for_url(&self, zone_name: &str) -> String {
        zone_name
            .replace(' ', "_")
            .replace("'", "%27")
            .replace("-", "_")
    }
}
