use std::env;

/// Represents the supported operating systems for the application
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
    Unknown,
}

/// Detects the current operating system using Rust's built-in OS detection
/// 
/// Uses `std::env::consts::OS` to determine the platform at compile time.
/// Returns an enum variant representing the detected OS or Unknown for unsupported platforms.
pub fn detect_os() -> OperatingSystem {
    match env::consts::OS {
        "windows" => OperatingSystem::Windows,
        "macos" => OperatingSystem::MacOs,
        "linux" => OperatingSystem::Linux,
        _ => OperatingSystem::Unknown,
    }
}

/// Returns a human-readable string representation of the operating system
/// 
/// Provides a user-friendly name for the detected OS, useful for logging
/// and user interface display purposes.
pub fn get_os_name() -> &'static str {
    match detect_os() {
        OperatingSystem::Windows => "Windows",
        OperatingSystem::MacOs => "macOS",
        OperatingSystem::Linux => "Linux",
        OperatingSystem::Unknown => "Unknown",
    }
}
