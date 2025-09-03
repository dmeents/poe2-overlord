use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Manages background tasks by tracking their JoinHandles and providing lifecycle control.
/// Uses a thread-safe HashMap to store task names mapped to their JoinHandles,
/// allowing for registration, unregistration, and graceful shutdown of background tasks.
#[derive(Clone)]
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl TaskManager {
    /// Creates a new TaskManager instance with an empty task registry.
    /// Initializes the internal HashMap wrapped in Arc<Mutex> for thread-safe access.
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a new background task with the given name and JoinHandle.
    /// If a task with the same name already exists, it will be replaced and the old task
    /// will be aborted if it's still running. This ensures only one task per name exists.
    pub async fn register_task(&self, name: String, handle: JoinHandle<()>) {
        let mut tasks = self.tasks.lock().await;

        if let Some(old_handle) = tasks.insert(name.clone(), handle) {
            debug!("Replaced existing task: {}", name);
            if !old_handle.is_finished() {
                old_handle.abort();
            }
        } else {
            debug!("Registered new task: {}", name);
        }
    }

    /// Removes a task from the registry by name and returns its JoinHandle if found.
    /// The caller is responsible for handling the returned JoinHandle (e.g., awaiting or aborting it).
    /// Returns None if no task with the given name exists.
    pub async fn unregister_task(&self, name: &str) -> Option<JoinHandle<()>> {
        let mut tasks = self.tasks.lock().await;
        tasks.remove(name)
    }

    /// Gracefully shuts down all registered background tasks by aborting them.
    /// Iterates through all tasks in the registry, aborts any that are still running,
    /// and clears the entire task registry. This is typically called during application shutdown.
    pub async fn shutdown_all_tasks(&self) {
        info!("Shutting down all background tasks...");

        let mut tasks = self.tasks.lock().await;

        for (name, handle) in tasks.drain() {
            debug!("Aborting task: {}", name);
            if !handle.is_finished() {
                handle.abort();
            }
        }

        info!("All background tasks shut down");
    }

    /// Returns the current number of registered background tasks.
    /// This provides a quick way to check how many tasks are currently being managed.
    pub async fn get_task_count(&self) -> usize {
        let tasks = self.tasks.lock().await;
        tasks.len()
    }
}

impl Default for TaskManager {
    /// Provides a default implementation that creates a new TaskManager instance.
    /// This allows TaskManager to be used in contexts that require Default trait implementation.
    fn default() -> Self {
        Self::new()
    }
}
