use crate::domain::walkthrough::models::WalkthroughGuide;
use crate::domain::walkthrough::traits::WalkthroughRepository;
use crate::errors::AppError;
use crate::infrastructure::file_management::FileService;
use log::info;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Repository for loading walkthrough guide data with in-memory caching.
///
/// The guide is loaded from disk once on first access and cached in memory.
/// All subsequent calls read from the cache, preventing race conditions
/// and improving performance for concurrent access.
pub struct WalkthroughRepositoryImpl {
    guide_path: PathBuf,
    /// In-memory cache of the guide (loaded once, never invalidated)
    cached_guide: Arc<RwLock<Option<WalkthroughGuide>>>,
    /// Flag to prevent redundant load attempts (fast path check)
    data_loaded: Arc<AtomicBool>,
}

impl WalkthroughRepositoryImpl {
    pub fn new(guide_path: PathBuf) -> Self {
        Self {
            guide_path,
            cached_guide: Arc::new(RwLock::new(None)),
            data_loaded: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Lazy-loads guide on first access (thread-safe with double-check locking).
    async fn ensure_data_loaded(&self) -> Result<(), AppError> {
        // Fast path: check without lock
        if self.data_loaded.load(Ordering::Relaxed) {
            return Ok(());
        }

        // Acquire write lock to load data
        let mut cache = self.cached_guide.write().await;

        // Double-check after acquiring lock (another thread may have loaded)
        if self.data_loaded.load(Ordering::Relaxed) {
            return Ok(());
        }

        // Load from disk
        let guide: WalkthroughGuide = FileService::read_json(&self.guide_path).await?;
        info!(
            "Successfully loaded walkthrough guide with {} acts (cached in memory)",
            guide.acts.len()
        );

        *cache = Some(guide);
        self.data_loaded.store(true, Ordering::Relaxed);

        Ok(())
    }
}

#[async_trait::async_trait]
impl WalkthroughRepository for WalkthroughRepositoryImpl {
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError> {
        // Ensure data is loaded (lazy init)
        self.ensure_data_loaded().await?;

        // Fast path: read from cache with read lock
        let cache = self.cached_guide.read().await;
        cache.clone().ok_or_else(|| {
            AppError::internal_error("load_guide", "Guide cache is empty after load attempt")
        })
    }
}
