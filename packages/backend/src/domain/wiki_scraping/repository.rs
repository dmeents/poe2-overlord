use crate::domain::wiki_scraping::url_utils;
use crate::errors::{AppError, AppResult};
use log::error;
use reqwest::Client;
use std::time::Duration;

pub struct WikiRepository {
    client: Client,
}

impl WikiRepository {
    pub fn new() -> AppResult<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("poe2-overlord/1.0")
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .map_err(|e| {
                error!("Failed to create HTTP client: {}", e);
                AppError::network_error("create_http_client", &e.to_string())
            })?;

        Ok(Self { client })
    }

    pub async fn fetch_page(&self, zone_name: &str) -> AppResult<String> {
        let url = url_utils::get_wiki_url(zone_name);

        let response = self.client.get(&url).send().await.map_err(|e| {
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
            error!(
                "Failed to read response body for zone '{}': {}",
                zone_name, e
            );
            AppError::network_error("read_response_body", &e.to_string())
        })?;

        Ok(content)
    }
}

