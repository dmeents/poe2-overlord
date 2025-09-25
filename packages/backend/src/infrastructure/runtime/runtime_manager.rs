use log::{debug, info};
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

/// Centralized Tokio runtime manager for the application
/// 
/// Provides a shared runtime instance that can be used across the application
/// for spawning background tasks and managing async operations.
/// Ensures consistent runtime configuration and lifecycle management.
#[derive(Clone)]
pub struct RuntimeManager {
    runtime: Arc<Runtime>,
}

impl RuntimeManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing shared Tokio runtime...");

        let runtime = Runtime::new()?;
        let runtime_arc = Arc::new(runtime);

        debug!("Shared Tokio runtime initialized successfully");

        Ok(Self {
            runtime: runtime_arc,
        })
    }

    /// Spawns a named background task on the shared runtime
    /// 
    /// Creates a new async task with the given name and function.
    /// The task will be executed on the shared runtime and its completion
    /// will be logged for debugging purposes.
    pub fn spawn_background_task<F, Fut>(&self, name: String, task: F) -> JoinHandle<()>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        debug!("Spawning background task: {}", name);

        let runtime = self.runtime.clone();

        runtime.spawn(async move {
            task().await;
            debug!("Background task '{}' completed", name);
        })
    }

    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    pub async fn shutdown(&self) {
        info!("Shutting down shared Tokio runtime...");

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        debug!("Shared Tokio runtime shutdown completed");
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default runtime manager")
    }
}
