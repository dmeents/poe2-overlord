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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_detection() {
        let os = detect_os();
        assert_ne!(os, OperatingSystem::Unknown);
        
        // At least one of these should be true
        assert!(is_windows() || is_macos() || is_linux());
    }

    #[test]
    fn test_os_name() {
        let os_name = get_os_name();
        assert!(!os_name.is_empty());
        assert_ne!(os_name, "Unknown");
    }
}
