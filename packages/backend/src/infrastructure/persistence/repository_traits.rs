use crate::errors::AppResult;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

/// Generic repository trait for basic CRUD operations
#[async_trait]
pub trait Repository<T, ID>: Send + Sync 
where
    T: Send + Sync + Clone,
    ID: Send + Sync + Clone,
{
    /// Save an entity
    async fn save(&self, entity: &T) -> AppResult<()>;
    
    /// Find an entity by ID
    async fn find_by_id(&self, id: &ID) -> AppResult<Option<T>>;
    
    /// Find all entities
    async fn find_all(&self) -> AppResult<Vec<T>>;
    
    /// Delete an entity by ID
    async fn delete_by_id(&self, id: &ID) -> AppResult<()>;
    
    /// Check if an entity exists by ID
    async fn exists(&self, id: &ID) -> AppResult<bool>;
}

/// Generic file-based repository trait
#[async_trait]
pub trait FileRepository<T>: Send + Sync 
where
    T: Send + Sync + Serialize + DeserializeOwned,
{
    /// Get the file path for this repository
    fn get_file_path(&self) -> &PathBuf;
    
    /// Load data from file
    async fn load_from_file(&self) -> AppResult<Option<T>>;
    
    /// Save data to file
    async fn save_to_file(&self, data: &T) -> AppResult<()>;
    
    /// Delete the data file
    async fn delete_file(&self) -> AppResult<()>;
    
    /// Check if the data file exists
    async fn file_exists(&self) -> AppResult<bool>;
}

/// Generic in-memory repository trait for data management
#[async_trait]
pub trait InMemoryRepository<T>: Send + Sync 
where
    T: Send + Sync + Clone,
{
    /// Get all data in memory
    async fn get_all_data(&self) -> AppResult<Vec<T>>;
    
    /// Add data to memory
    async fn add_data(&self, data: T) -> AppResult<()>;
    
    /// Update data in memory
    async fn update_data(&self, data: T) -> AppResult<()>;
    
    /// Remove data from memory
    async fn remove_data(&self, data: T) -> AppResult<()>;
    
    /// Clear all data from memory
    async fn clear_all_data(&self) -> AppResult<()>;
    
    /// Get data count
    async fn get_data_count(&self) -> AppResult<usize>;
}

/// Generic query repository trait
#[async_trait]
pub trait QueryRepository<T, Q>: Send + Sync 
where
    T: Send + Sync + Clone,
    Q: Send + Sync,
{
    /// Find entities by query
    async fn find_by_query(&self, query: &Q) -> AppResult<Vec<T>>;
    
    /// Find first entity by query
    async fn find_first_by_query(&self, query: &Q) -> AppResult<Option<T>>;
    
    /// Count entities by query
    async fn count_by_query(&self, query: &Q) -> AppResult<usize>;
    
    /// Check if any entity exists by query
    async fn exists_by_query(&self, query: &Q) -> AppResult<bool>;
}

/// Generic aggregation repository trait
#[async_trait]
pub trait AggregationRepository<T, R>: Send + Sync 
where
    T: Send + Sync + Clone,
    R: Send + Sync,
{
    /// Calculate aggregate result
    async fn calculate_aggregate(&self, data: &[T]) -> AppResult<R>;
    
    /// Get top N entities by some criteria
    async fn get_top_n(&self, n: usize) -> AppResult<Vec<T>>;
    
    /// Get entities grouped by some criteria
    async fn get_grouped(&self) -> AppResult<std::collections::HashMap<String, Vec<T>>>;
}

/// Generic validation repository trait
#[async_trait]
pub trait ValidationRepository<T>: Send + Sync 
where
    T: Send + Sync,
{
    /// Validate entity before save
    async fn validate_before_save(&self, entity: &T) -> AppResult<()>;
    
    /// Validate entity before update
    async fn validate_before_update(&self, entity: &T) -> AppResult<()>;
    
    /// Validate entity before delete
    async fn validate_before_delete(&self, entity: &T) -> AppResult<()>;
    
    /// Check business rules
    async fn check_business_rules(&self, entity: &T) -> AppResult<()>;
}

/// Generic cache repository trait
#[async_trait]
pub trait CacheRepository<T, K>: Send + Sync 
where
    T: Send + Sync + Clone,
    K: Send + Sync + Clone,
{
    /// Get from cache
    async fn get_from_cache(&self, key: &K) -> AppResult<Option<T>>;
    
    /// Put in cache
    async fn put_in_cache(&self, key: K, value: T) -> AppResult<()>;
    
    /// Remove from cache
    async fn remove_from_cache(&self, key: &K) -> AppResult<()>;
    
    /// Clear cache
    async fn clear_cache(&self) -> AppResult<()>;
    
    /// Check if key exists in cache
    async fn cache_exists(&self, key: &K) -> AppResult<bool>;
}
