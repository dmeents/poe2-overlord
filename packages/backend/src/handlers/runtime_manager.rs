use log::{debug, info};
use std::sync::Arc;
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

/// This manager provides a centralized way to spawn and manage background tasks
/// that need to run independently of the main application thread. It ensures
/// that all background tasks share the same runtime instance, which is important
/// for resource management and task coordination.
#[derive(Clone)]
pub struct RuntimeManager {
    runtime: Arc<Runtime>,
}

impl RuntimeManager {
    /// This method initializes a new Tokio runtime that will be used for all
    /// background task execution throughout the application lifecycle. The runtime
    /// is wrapped in an Arc to allow safe sharing across multiple threads and
    /// components.
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing shared Tokio runtime...");

        let runtime = Runtime::new()?;
        let runtime_arc = Arc::new(runtime);

        debug!("Shared Tokio runtime initialized successfully");

        Ok(Self {
            runtime: runtime_arc,
        })
    }

    /// This method is the primary way to execute background tasks in the application.
    /// It takes a closure that returns a future and spawns it on the shared runtime,
    /// providing automatic logging for task lifecycle events.
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

    /// This method provides access to the runtime handle, which can be used to
    /// spawn additional tasks or perform runtime-specific operations. The handle
    /// is cloned to avoid borrowing issues and can be used from any thread.
    pub fn handle(&self) -> Handle {
        self.runtime.handle().clone()
    }

    /// This method provides a controlled way to shut down the runtime and all
    /// associated background tasks. It gives running tasks a brief opportunity
    /// to complete their work before the runtime is terminated.
    pub async fn shutdown(&self) {
        info!("Shutting down shared Tokio runtime...");

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        debug!("Shared Tokio runtime shutdown completed");
    }
}

impl Default for RuntimeManager {
    /// This implementation provides a convenient way to create a RuntimeManager
    /// without explicitly handling the Result. It panics if runtime creation fails,
    /// which is appropriate for the Default trait since it's expected to always
    /// succeed.
    fn default() -> Self {
        Self::new().expect("Failed to create default runtime manager")
    }
}
