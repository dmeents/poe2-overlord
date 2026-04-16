use crate::errors::AppResult;
use async_trait::async_trait;

use super::models::{
    GameDataVersion, Item, ItemCategory, ItemSearchParams, ItemSearchResult,
};

#[async_trait]
pub trait ItemDataRepository: Send + Sync {
    /// Returns the currently imported game data version, or None if not yet imported.
    async fn get_version(&self) -> AppResult<Option<GameDataVersion>>;

    /// Full replacement import: replaces all items and categories in a transaction.
    /// Existing favourites are lost (their item IDs may change between patches).
    async fn import_data(
        &self,
        patch_version: &str,
        extracted_at: &str,
        categories: &[ItemCategory],
        items: &[Item],
    ) -> AppResult<()>;

    /// Fetch a single item by its primary key.
    async fn get_item(&self, id: &str) -> AppResult<Option<Item>>;

    /// Search/filter items with optional full-text query, category, and level range.
    async fn search_items(&self, params: &ItemSearchParams) -> AppResult<ItemSearchResult>;

    /// All categories.
    async fn get_categories(&self) -> AppResult<Vec<ItemCategory>>;

    /// Toggle favourite status. Returns true if the item is now a favourite.
    async fn toggle_favorite(&self, item_id: &str) -> AppResult<bool>;

    /// All favourited items (joined with items table).
    async fn get_favorites(&self) -> AppResult<Vec<Item>>;

    /// Look up a base item (is_unique = 0) by exact name. Used to bridge economy ↔ item_data
    /// domains without a foreign-key relationship (poe.ninja slugs ≠ game metadata paths).
    async fn get_item_by_name(&self, name: &str) -> AppResult<Option<Item>>;
}

#[async_trait]
pub trait ItemDataService: Send + Sync {
    /// Called on startup: compare bundled version.json with DB version and import if newer.
    async fn ensure_data_imported(&self) -> AppResult<()>;

    async fn get_item(&self, id: &str) -> AppResult<Option<Item>>;
    async fn search_items(&self, params: ItemSearchParams) -> AppResult<ItemSearchResult>;
    async fn get_categories(&self) -> AppResult<Vec<ItemCategory>>;
    async fn get_version(&self) -> AppResult<Option<GameDataVersion>>;
    async fn toggle_favorite(&self, item_id: &str) -> AppResult<bool>;
    async fn get_favorites(&self) -> AppResult<Vec<Item>>;
    async fn get_item_by_name(&self, name: &str) -> AppResult<Option<Item>>;
}
