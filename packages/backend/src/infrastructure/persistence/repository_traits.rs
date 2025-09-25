use crate::errors::AppResult;
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

#[async_trait]
pub trait Repository<T, ID>: Send + Sync 
where
    T: Send + Sync + Clone,
    ID: Send + Sync + Clone,
{
    async fn save(&self, entity: &T) -> AppResult<()>;
    
    async fn find_by_id(&self, id: &ID) -> AppResult<Option<T>>;
    
    async fn find_all(&self) -> AppResult<Vec<T>>;
    
    async fn delete_by_id(&self, id: &ID) -> AppResult<()>;
    
    async fn exists(&self, id: &ID) -> AppResult<bool>;
}

#[async_trait]
pub trait FileRepository<T>: Send + Sync 
where
    T: Send + Sync + Serialize + DeserializeOwned,
{
    fn get_file_path(&self) -> &PathBuf;
    
    async fn load_from_file(&self) -> AppResult<Option<T>>;
    
    async fn save_to_file(&self, data: &T) -> AppResult<()>;
    
    async fn delete_file(&self) -> AppResult<()>;
    
    async fn file_exists(&self) -> AppResult<bool>;
}

#[async_trait]
pub trait InMemoryRepository<T>: Send + Sync 
where
    T: Send + Sync + Clone,
{
    async fn get_all_data(&self) -> AppResult<Vec<T>>;
    
    async fn add_data(&self, data: T) -> AppResult<()>;
    
    async fn update_data(&self, data: T) -> AppResult<()>;
    
    async fn remove_data(&self, data: T) -> AppResult<()>;
    
    async fn clear_all_data(&self) -> AppResult<()>;
    
    async fn get_data_count(&self) -> AppResult<usize>;
}

#[async_trait]
pub trait QueryRepository<T, Q>: Send + Sync 
where
    T: Send + Sync + Clone,
    Q: Send + Sync,
{
    async fn find_by_query(&self, query: &Q) -> AppResult<Vec<T>>;
    
    async fn find_first_by_query(&self, query: &Q) -> AppResult<Option<T>>;
    
    async fn count_by_query(&self, query: &Q) -> AppResult<usize>;
    
    async fn exists_by_query(&self, query: &Q) -> AppResult<bool>;
}

#[async_trait]
pub trait AggregationRepository<T, R>: Send + Sync 
where
    T: Send + Sync + Clone,
    R: Send + Sync,
{
    async fn calculate_aggregate(&self, data: &[T]) -> AppResult<R>;
    
    async fn get_top_n(&self, n: usize) -> AppResult<Vec<T>>;
    
    async fn get_grouped(&self) -> AppResult<std::collections::HashMap<String, Vec<T>>>;
}

#[async_trait]
pub trait ValidationRepository<T>: Send + Sync 
where
    T: Send + Sync,
{
    async fn validate_before_save(&self, entity: &T) -> AppResult<()>;
    
    async fn validate_before_update(&self, entity: &T) -> AppResult<()>;
    
    async fn validate_before_delete(&self, entity: &T) -> AppResult<()>;
    
    async fn check_business_rules(&self, entity: &T) -> AppResult<()>;
}

#[async_trait]
pub trait CacheRepository<T, K>: Send + Sync 
where
    T: Send + Sync + Clone,
    K: Send + Sync + Clone,
{
    async fn get_from_cache(&self, key: &K) -> AppResult<Option<T>>;
    
    async fn put_in_cache(&self, key: K, value: T) -> AppResult<()>;
    
    async fn remove_from_cache(&self, key: &K) -> AppResult<()>;
    
    async fn clear_cache(&self) -> AppResult<()>;
    
    async fn cache_exists(&self, key: &K) -> AppResult<bool>;
}
