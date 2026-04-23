use std::path::{Path, PathBuf};
use std::sync::Arc;

use async_trait::async_trait;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use log::{debug, warn};
use reqwest::Client;
use tokio::fs;
use tokio::sync::Mutex;

use crate::errors::{AppError, AppResult};

use super::traits::ItemImageService;

const POECDN_PREFIX: &str = "https://web.poecdn.com/image/";
const POE2DB_PREFIX: &str = "https://cdn.poe2db.tw/image/";
const POE2DB_REFERER: &str = "https://poe2db.tw/";

pub struct ItemImageServiceImpl {
    client: Client,
    cache_dir: PathBuf,
    /// Serialize concurrent fetches per URL so a tooltip opening twice in
    /// rapid succession doesn't issue two network requests.
    in_flight: Mutex<std::collections::HashMap<String, Arc<Mutex<()>>>>,
}

impl ItemImageServiceImpl {
    pub fn new(cache_dir: PathBuf) -> AppResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .user_agent("poe2-overlord/1.0")
            .redirect(reqwest::redirect::Policy::limited(3))
            .build()
            .map_err(|e| {
                AppError::network_error(
                    "item_image::new",
                    &format!("Failed to build HTTP client: {e}"),
                )
            })?;
        Ok(Self {
            client,
            cache_dir,
            in_flight: Mutex::new(std::collections::HashMap::new()),
        })
    }

    /// Convert a `web.poecdn.com/image/Art/...<ext>` URL into the equivalent
    /// `cdn.poe2db.tw/image/Art/....webp` URL. Non-matching inputs return None.
    fn to_poe2db_url(poecdn_url: &str) -> Option<String> {
        let path = poecdn_url.strip_prefix(POECDN_PREFIX)?;
        // Strip any current extension and append .webp
        let stem = match path.rfind('.') {
            Some(idx) => &path[..idx],
            None => path,
        };
        Some(format!("{POE2DB_PREFIX}{stem}.webp"))
    }

    /// Derive a disk cache path from the upstream path. Uses the same
    /// subtree structure as the URL to keep debugging easy.
    fn cache_path_for(&self, poecdn_url: &str) -> Option<PathBuf> {
        let path = poecdn_url.strip_prefix(POECDN_PREFIX)?;
        let stem = match path.rfind('.') {
            Some(idx) => &path[..idx],
            None => path,
        };
        Some(self.cache_dir.join(format!("{stem}.webp")))
    }

    async fn read_cached(&self, path: &Path) -> Option<Vec<u8>> {
        match fs::read(path).await {
            Ok(bytes) if !bytes.is_empty() => Some(bytes),
            _ => None,
        }
    }

    async fn write_cached(path: &Path, bytes: &[u8]) -> AppResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                AppError::file_system_error(
                    "item_image::write_cached",
                    &format!("Failed to create cache dir {parent:?}: {e}"),
                )
            })?;
        }
        fs::write(path, bytes).await.map_err(|e| {
            AppError::file_system_error(
                "item_image::write_cached",
                &format!("Failed to write cache file {path:?}: {e}"),
            )
        })
    }

    async fn fetch_from_upstream(&self, url: &str) -> AppResult<Vec<u8>> {
        let resp = self
            .client
            .get(url)
            .header(reqwest::header::REFERER, POE2DB_REFERER)
            .send()
            .await
            .map_err(|e| {
                AppError::network_error(
                    "item_image::fetch",
                    &format!("Failed to fetch {url}: {e}"),
                )
            })?;

        if !resp.status().is_success() {
            return Err(AppError::network_error(
                "item_image::fetch",
                &format!("HTTP {} for {url}", resp.status()),
            ));
        }

        let bytes = resp.bytes().await.map_err(|e| {
            AppError::network_error(
                "item_image::fetch",
                &format!("Failed to read body for {url}: {e}"),
            )
        })?;

        Ok(bytes.to_vec())
    }
}

#[async_trait]
impl ItemImageService for ItemImageServiceImpl {
    async fn get_image_data_url(&self, poecdn_url: &str) -> AppResult<String> {
        let Some(cache_path) = self.cache_path_for(poecdn_url) else {
            // URL isn't the shape we handle — return it untouched so the
            // caller can still render what they had.
            return Ok(poecdn_url.to_string());
        };

        if let Some(bytes) = self.read_cached(&cache_path).await {
            return Ok(encode_data_url(&bytes));
        }

        // Serialize concurrent fetches for the same URL.
        let lock = {
            let mut map = self.in_flight.lock().await;
            map.entry(poecdn_url.to_string())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };
        let _guard = lock.lock().await;

        // Re-check cache under the lock — another task may have populated it.
        if let Some(bytes) = self.read_cached(&cache_path).await {
            return Ok(encode_data_url(&bytes));
        }

        let Some(upstream_url) = Self::to_poe2db_url(poecdn_url) else {
            return Ok(poecdn_url.to_string());
        };

        debug!("Fetching POE2 item image: {upstream_url}");
        let bytes = self.fetch_from_upstream(&upstream_url).await?;
        if let Err(e) = Self::write_cached(&cache_path, &bytes).await {
            warn!("Failed to cache image to {cache_path:?}: {e}");
        }

        Ok(encode_data_url(&bytes))
    }
}

fn encode_data_url(bytes: &[u8]) -> String {
    let b64 = BASE64.encode(bytes);
    format!("data:image/webp;base64,{b64}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_poecdn_png_to_poe2db_webp() {
        let got = ItemImageServiceImpl::to_poe2db_url(
            "https://web.poecdn.com/image/Art/2DItems/Currency/CurrencyModValues.png",
        );
        assert_eq!(
            got.as_deref(),
            Some("https://cdn.poe2db.tw/image/Art/2DItems/Currency/CurrencyModValues.webp"),
        );
    }

    #[test]
    fn rewrites_preserves_subdirs() {
        let got = ItemImageServiceImpl::to_poe2db_url(
            "https://web.poecdn.com/image/Art/2DItems/Currency/Runes/FireRune.png",
        );
        assert_eq!(
            got.as_deref(),
            Some("https://cdn.poe2db.tw/image/Art/2DItems/Currency/Runes/FireRune.webp"),
        );
    }

    #[test]
    fn non_poecdn_url_is_not_rewritten() {
        assert!(ItemImageServiceImpl::to_poe2db_url("https://example.com/foo.png").is_none());
    }
}
