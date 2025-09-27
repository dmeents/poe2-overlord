use crate::errors::AppResult;
use log::debug;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

/// Manages file-level locks to prevent race conditions during concurrent file operations.
///
/// This manager ensures that only one operation can modify a specific file at a time,
/// while allowing different files to be processed concurrently.
pub struct FileLockManager {
    locks: Arc<Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>>,
}

/// Global singleton instance of the file lock manager
/// This ensures all repository instances share the same lock manager
static GLOBAL_LOCK_MANAGER: OnceLock<FileLockManager> = OnceLock::new();

impl FileLockManager {
    /// Creates a new FileLockManager instance
    pub fn new() -> Self {
        Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Gets the global singleton instance of the file lock manager
    pub fn global() -> &'static FileLockManager {
        GLOBAL_LOCK_MANAGER.get_or_init(|| Self::new())
    }

    /// Executes an operation with a file-level lock
    ///
    /// This method ensures that only one operation can access a specific file at a time.
    /// The lock is automatically released when the operation completes.
    pub async fn with_file_lock<F, Fut, R>(&self, file_path: &str, operation: F) -> AppResult<R>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = AppResult<R>>,
    {
        // Get or create a lock for this file
        let file_lock = {
            let mut locks = self.locks.lock().await;
            locks
                .entry(file_path.to_string())
                .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
                .clone()
        };

        // Acquire the file lock
        let _guard = file_lock.lock().await;
        debug!("Acquired file lock for: {}", file_path);

        // Execute the operation
        let result = operation().await;

        debug!("Released file lock for: {}", file_path);
        result
    }

    /// Cleans up unused locks to prevent memory leaks
    ///
    /// This should be called periodically to remove locks for files that are no longer
    /// being accessed. For now, we keep all locks in memory as the overhead is minimal
    /// for the expected number of character files.
    pub async fn cleanup_unused_locks(&self) {
        // For now, we don't implement cleanup as the overhead is minimal
        // and character files are expected to be accessed regularly
        debug!("File lock cleanup called (no-op for now)");
    }
}

impl Default for FileLockManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for FileLockManager {
    fn clone(&self) -> Self {
        Self {
            locks: self.locks.clone(),
        }
    }
}
