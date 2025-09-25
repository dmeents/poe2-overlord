pub mod app_setup;
pub mod service_orchestrator;
pub mod service_registry;

pub use app_setup::setup_app;
pub use service_orchestrator::{
    start_game_process_monitoring, start_log_monitoring, start_ping_event_emission,
    start_time_tracking_emission,
};
pub use service_registry::{ServiceInitializer, ServiceInstances};
