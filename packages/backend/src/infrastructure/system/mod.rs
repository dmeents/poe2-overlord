pub mod os_detection;
pub mod paths;

// Re-export commonly used items
pub use os_detection::{detect_os, get_os_name, OperatingSystem};
pub use paths::PoeClientLogPaths;
