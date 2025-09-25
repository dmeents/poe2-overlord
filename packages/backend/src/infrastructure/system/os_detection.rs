use std::env;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOs,
    Linux,
    Unknown,
}

pub fn detect_os() -> OperatingSystem {
    match env::consts::OS {
        "windows" => OperatingSystem::Windows,
        "macos" => OperatingSystem::MacOs,
        "linux" => OperatingSystem::Linux,
        _ => OperatingSystem::Unknown,
    }
}

pub fn get_os_name() -> &'static str {
    match detect_os() {
        OperatingSystem::Windows => "Windows",
        OperatingSystem::MacOs => "macOS",
        OperatingSystem::Linux => "Linux",
        OperatingSystem::Unknown => "Unknown",
    }
}
