pub mod os_detection;
pub mod paths;

pub use os_detection::{detect_os, get_os_name, OperatingSystem};
pub use paths::PoeClientLogPaths;
