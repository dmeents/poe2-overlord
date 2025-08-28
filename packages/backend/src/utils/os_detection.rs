use std::env;

/// Represents the detected operating system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
    Unknown,
}

/// Detect the current operating system
pub fn detect_os() -> OperatingSystem {
    match env::consts::OS {
        "windows" => OperatingSystem::Windows,
        "macos" => OperatingSystem::MacOs,
        "linux" => OperatingSystem::Linux,
        _ => OperatingSystem::Unknown,
    }
}

/// Get the current operating system as a string
pub fn get_os_name() -> &'static str {
    match detect_os() {
        OperatingSystem::Windows => "Windows",
        OperatingSystem::MacOs => "macOS",
        OperatingSystem::Linux => "Linux",
        OperatingSystem::Unknown => "Unknown",
    }
}

/// Check if the current OS is Windows
pub fn is_windows() -> bool {
    detect_os() == OperatingSystem::Windows
}

/// Check if the current OS is macOS
pub fn is_macos() -> bool {
    detect_os() == OperatingSystem::MacOs
}

/// Check if the current OS is Linux
pub fn is_linux() -> bool {
    detect_os() == OperatingSystem::Linux
}
