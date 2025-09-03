use log::{debug, info};
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

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
        
        // Wait for a short time to allow tasks to complete gracefully
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // The runtime will be dropped when the Arc goes out of scope
        debug!("Shared Tokio runtime shutdown completed");
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default runtime manager")
    }
}
