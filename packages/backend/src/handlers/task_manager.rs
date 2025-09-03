use log::{debug, info};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_task(&self, name: String, handle: JoinHandle<()>) {
        if let Ok(mut tasks) = self.tasks.lock() {
            if let Some(old_handle) = tasks.insert(name.clone(), handle) {
                debug!("Replaced existing task: {}", name);
                // Abort the old task if it's still running
                if !old_handle.is_finished() {
                    old_handle.abort();
                }
            } else {
                debug!("Registered new task: {}", name);
            }
        }
    }

    pub fn unregister_task(&self, name: &str) -> Option<JoinHandle<()>> {
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.remove(name)
        } else {
            None
        }
    }

    pub fn shutdown_all_tasks(&self) {
        info!("Shutting down all background tasks...");

        if let Ok(mut tasks) = self.tasks.lock() {
            for (name, handle) in tasks.drain() {
                debug!("Aborting task: {}", name);
                if !handle.is_finished() {
                    handle.abort();
                }
            }
        }

        info!("All background tasks shut down");
    }

    pub fn get_task_count(&self) -> usize {
        if let Ok(tasks) = self.tasks.lock() {
            tasks.len()
        } else {
            0
        }
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
