use log::{debug, info};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

/// Manages the lifecycle of background tasks across the application
/// 
/// Provides centralized registration, tracking, and shutdown of async tasks.
/// Ensures proper cleanup of resources when tasks are no longer needed.
#[derive(Clone)]
pub struct TaskManager {
    /// Thread-safe map of task names to their join handles
    tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Registers a new background task with the manager
    /// 
    /// If a task with the same name already exists, it will be aborted
    /// and replaced with the new task. This ensures no duplicate tasks
    /// are running simultaneously.
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

    pub async fn unregister_task(&self, name: &str) -> Option<JoinHandle<()>> {
        let mut tasks = self.tasks.lock().await;
        tasks.remove(name)
    }

    /// Gracefully shuts down all registered background tasks
    /// 
    /// Iterates through all registered tasks and aborts them if they're still running.
    /// This is typically called during application shutdown to ensure clean termination.
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

    pub async fn get_task_count(&self) -> usize {
        let tasks = self.tasks.lock().await;
        tasks.len()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
