use crate::errors::AppResult;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

#[async_trait]
pub trait FileOperations {
    async fn read_json<T: DeserializeOwned + Send>(&self, path: &Path) -> AppResult<T>;

    async fn read_json_optional<T: DeserializeOwned + Send>(
        &self,
        path: &Path,
    ) -> AppResult<Option<T>>;

    async fn write_json<T: Serialize + Sync>(&self, path: &Path, data: &T) -> AppResult<()>;

    async fn delete(&self, path: &Path) -> AppResult<()>;

    async fn exists(&self, path: &Path) -> AppResult<bool>;

    async fn ensure_parent_dir(&self, path: &Path) -> AppResult<()>;
}
