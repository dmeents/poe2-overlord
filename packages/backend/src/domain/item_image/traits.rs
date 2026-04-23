use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait ItemImageService: Send + Sync {
    /// Given a `web.poecdn.com/image/Art/...` URL (as stored on item records),
    /// return a `data:image/webp;base64,...` URL for the POE2 asset, using a
    /// disk cache under the app data dir. Fetches from cdn.poe2db.tw on a
    /// cache miss.
    async fn get_image_data_url(&self, poecdn_url: &str) -> AppResult<String>;
}
