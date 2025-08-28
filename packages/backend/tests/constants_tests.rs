use app_lib::utils::{OperatingSystem, PoeClientLogPaths};

#[test]
fn test_default_path_not_empty() {
    let path = PoeClientLogPaths::get_default_path_string();
    assert!(!path.is_empty());
}

#[test]
fn test_paths_for_all_oses() {
    let windows_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::Windows);
    let macos_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::MacOs);
    let linux_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::Linux);

    assert!(windows_path.to_string_lossy().contains("Path of Exile"));
    assert!(macos_path.to_string_lossy().contains("Path of Exile"));
    assert!(linux_path.to_string_lossy().contains("Path of Exile"));
}

#[test]
fn test_windows_path_structure() {
    let windows_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::Windows);
    let path_str = windows_path.to_string_lossy();

    assert!(path_str.contains("Program Files (x86)"));
    assert!(path_str.contains("Grinding Gear Games"));
    assert!(path_str.contains("Path of Exile"));
    assert!(path_str.contains("logs"));
    assert!(path_str.contains("Client.txt"));
}

#[test]
fn test_macos_path_structure() {
    let macos_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::MacOs);
    let path_str = macos_path.to_string_lossy();

    assert!(path_str.contains("Library"));
    assert!(path_str.contains("Application Support"));
    assert!(path_str.contains("Path of Exile"));
    assert!(path_str.contains("logs"));
    assert!(path_str.contains("Client.txt"));
}

#[test]
fn test_linux_path_structure() {
    let linux_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::Linux);
    let path_str = linux_path.to_string_lossy();

    assert!(path_str.contains(".var"));
    assert!(path_str.contains("app"));
    assert!(path_str.contains("com.valvesoftware.Steam"));
    assert!(path_str.contains("Steam"));
    assert!(path_str.contains("steamapps"));
    assert!(path_str.contains("common"));
    assert!(path_str.contains("Path of Exile 2"));
    assert!(path_str.contains("logs"));
    assert!(path_str.contains("Client.txt"));
}

#[test]
fn test_unknown_os_fallback() {
    let unknown_path = PoeClientLogPaths::get_path_for_os(&OperatingSystem::Unknown);
    assert_eq!(unknown_path, std::path::PathBuf::from("Client.txt"));
}

#[test]
fn test_current_os_default_path() {
    let default_path = PoeClientLogPaths::get_default_path();
    let default_path_str = PoeClientLogPaths::get_default_path_string();

    assert!(!default_path_str.is_empty());
    assert_eq!(default_path.to_string_lossy(), default_path_str);
    assert!(default_path_str.contains("Path of Exile"));
}
