pub mod monitoring;
pub mod network;
pub mod parsing;
pub mod persistence;
pub mod runtime;
pub mod system;
pub mod tauri;
pub mod time;

pub use monitoring::process_monitor::ProcessMonitorImpl;
pub use runtime::{RuntimeManager, TaskManager};
pub use system::{detect_os, get_os_name, OperatingSystem, PoeClientLogPaths};
pub use time::{
    calculate_active_session_duration_seconds, calculate_session_duration_from_timestamps,
    calculate_session_duration_seconds, validate_duration, validate_no_session_overlap,
    validate_session_data, validate_timestamp_order, ValidationResult,
};
