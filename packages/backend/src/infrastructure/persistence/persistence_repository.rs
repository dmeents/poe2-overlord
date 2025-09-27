use crate::errors::AppResult;
use crate::infrastructure::persistence::{
    AtomicWriter, DirectoryManager, FileLockManager, FileOperations, JsonStorage,
};
use async_trait::async_trait;
use log::debug;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;
use std::path::PathBuf;

#[async_trait]
pub trait PersistenceRepository<T>: Send + Sync
where
    T: Send + Sync + Serialize + DeserializeOwned + Clone + Default,
{
    async fn save(&self, data: &T) -> AppResult<()>;

    async fn load(&self) -> AppResult<T>;

    async fn exists(&self) -> AppResult<bool>;

    async fn delete(&self) -> AppResult<()>;

    async fn load_or_default(&self) -> AppResult<T>;
}

#[derive(Clone)]
pub struct PersistenceRepositoryImpl<T> {
    file_path: PathBuf,
    _phantom: PhantomData<T>,
}

impl<T> PersistenceRepositoryImpl<T>
where
    T: Send + Sync + Serialize + DeserializeOwned + Clone + Default,
{
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            _phantom: PhantomData,
        }
    }

    pub fn new_in_config_dir(file_name: &str) -> AppResult<Self> {
        let config_dir = DirectoryManager::ensure_config_directory()?;
        let file_path = config_dir.join(file_name);
        Ok(Self::new(file_path))
    }

    pub fn new_in_data_dir(file_name: &str) -> AppResult<Self> {
        let data_dir = DirectoryManager::ensure_data_directory()?;
        let file_path = data_dir.join(file_name);
        Ok(Self::new(file_path))
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

#[async_trait]
impl<T> PersistenceRepository<T> for PersistenceRepositoryImpl<T>
where
    T: Send + Sync + Serialize + DeserializeOwned + Clone + Default,
{
    async fn save(&self, data: &T) -> AppResult<()> {
        let file_path = self.file_path.to_string_lossy().to_string();
        FileLockManager::global()
            .with_file_lock(&file_path, || async {
                let json_content = JsonStorage::serialize(data)?;
                AtomicWriter::write_atomic_async(&self.file_path, &json_content).await?;
                debug!("Data saved successfully to {:?}", self.file_path);
                Ok(())
            })
            .await
    }

    async fn load(&self) -> AppResult<T> {
        let file_path = self.file_path.to_string_lossy().to_string();
        FileLockManager::global()
            .with_file_lock(&file_path, || async {
                if !FileOperations::file_exists(&self.file_path) {
                    debug!("No data file found, returning default");
                    return Ok(T::default());
                }

                let data: T = JsonStorage::load_from_file(&self.file_path)?;
                debug!("Data loaded successfully from {:?}", self.file_path);
                Ok(data)
            })
            .await
    }

    async fn exists(&self) -> AppResult<bool> {
        Ok(FileOperations::file_exists(&self.file_path))
    }

    async fn delete(&self) -> AppResult<()> {
        if FileOperations::file_exists(&self.file_path) {
            FileOperations::delete_file(&self.file_path)?;
        }
        debug!("Data file deleted: {:?}", self.file_path);
        Ok(())
    }

    async fn load_or_default(&self) -> AppResult<T> {
        if self.exists().await? {
            self.load().await
        } else {
            Ok(T::default())
        }
    }
}

#[async_trait]
pub trait ScopedPersistenceRepository<T, K>: Send + Sync
where
    T: Send + Sync + Serialize + DeserializeOwned + Clone + Default,
    K: Send + Sync + Clone + ToString,
{
    async fn save_scoped(&self, key: &K, data: &T) -> AppResult<()>;

    async fn load_scoped(&self, key: &K) -> AppResult<Option<T>>;

    async fn delete_scoped(&self, key: &K) -> AppResult<()>;

    async fn exists_scoped(&self, key: &K) -> AppResult<bool>;
}

#[derive(Clone)]
pub struct ScopedPersistenceRepositoryImpl<T, K> {
    base_directory: PathBuf,
    file_prefix: String,
    file_suffix: String,
    _phantom: PhantomData<(T, K)>,
}

impl<T, K> ScopedPersistenceRepositoryImpl<T, K>
where
    T: Send + Sync + Serialize + DeserializeOwned + Clone + Default,
    K: Send + Sync + Clone + ToString,
{
    pub fn new(base_directory: PathBuf, file_prefix: String, file_suffix: String) -> Self {
        Self {
            base_directory,
            file_prefix,
            file_suffix,
            _phantom: PhantomData,
        }
    }

    pub fn new_in_data_dir(file_prefix: &str, file_suffix: &str) -> AppResult<Self> {
        let data_dir = DirectoryManager::ensure_data_directory()?;
        Ok(Self::new(
            data_dir,
            file_prefix.to_string(),
            file_suffix.to_string(),
        ))
    }

    fn get_file_path(&self, key: &K) -> PathBuf {
        let file_name = format!(
            "{}{}{}",
            self.file_prefix,
            key.to_string(),
            self.file_suffix
        );
        self.base_directory.join(file_name)
    }
}

#[async_trait]
impl<T, K> ScopedPersistenceRepository<T, K> for ScopedPersistenceRepositoryImpl<T, K>
where
    T: Send + Sync + Serialize + DeserializeOwned + Clone + Default,
    K: Send + Sync + Clone + ToString,
{
    async fn save_scoped(&self, key: &K, data: &T) -> AppResult<()> {
        let file_path = self.get_file_path(key);
        let file_path_str = file_path.to_string_lossy().to_string();
        FileLockManager::global()
            .with_file_lock(&file_path_str, || async {
                let json_content = JsonStorage::serialize(data)?;
                AtomicWriter::write_atomic_async(&file_path, &json_content).await?;
                debug!("Scoped data saved successfully to {:?}", file_path);
                Ok(())
            })
            .await
    }

    async fn load_scoped(&self, key: &K) -> AppResult<Option<T>> {
        let file_path = self.get_file_path(key);
        let file_path_str = file_path.to_string_lossy().to_string();
        FileLockManager::global()
            .with_file_lock(&file_path_str, || async {
                if !FileOperations::file_exists(&file_path) {
                    debug!("No scoped data file found for key: {}", key.to_string());
                    return Ok(None);
                }

                let data: T = JsonStorage::load_from_file(&file_path)?;
                debug!("Scoped data loaded successfully from {:?}", file_path);
                Ok(Some(data))
            })
            .await
    }

    async fn delete_scoped(&self, key: &K) -> AppResult<()> {
        let file_path = self.get_file_path(key);
        let file_path_str = file_path.to_string_lossy().to_string();
        FileLockManager::global()
            .with_file_lock(&file_path_str, || async {
                if FileOperations::file_exists(&file_path) {
                    FileOperations::delete_file(&file_path)?;
                }
                debug!("Scoped data file deleted: {:?}", file_path);
                Ok(())
            })
            .await
    }

    async fn exists_scoped(&self, key: &K) -> AppResult<bool> {
        let file_path = self.get_file_path(key);
        Ok(FileOperations::file_exists(&file_path))
    }
}
