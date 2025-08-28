use app_lib::utils::{OperatingSystem, detect_os, get_os_name, is_windows, is_macos, is_linux};

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

#[test]
fn test_os_enum_variants() {
    // Test that all OS variants can be created and compared
    let windows = OperatingSystem::Windows;
    let macos = OperatingSystem::MacOs;
    let linux = OperatingSystem::Linux;
    let unknown = OperatingSystem::Unknown;

    assert_ne!(windows, macos);
    assert_ne!(windows, linux);
    assert_ne!(windows, unknown);
    assert_ne!(macos, linux);
    assert_ne!(macos, unknown);
    assert_ne!(linux, unknown);

    // Test that each OS is different from others
    assert_eq!(windows, OperatingSystem::Windows);
    assert_eq!(macos, OperatingSystem::MacOs);
    assert_eq!(linux, OperatingSystem::Linux);
    assert_eq!(unknown, OperatingSystem::Unknown);
}

#[test]
fn test_os_checker_functions() {
    let current_os = detect_os();
    
    // Only one OS checker should return true
    let windows_check = is_windows();
    let macos_check = is_macos();
    let linux_check = is_linux();
    
    // Count how many are true
    let true_count = [windows_check, macos_check, linux_check]
        .iter()
        .filter(|&&x| x)
        .count();
    
    // Exactly one should be true
    assert_eq!(true_count, 1, "Exactly one OS checker should return true");
    
    // The detected OS should match the checker function
    match current_os {
        OperatingSystem::Windows => assert!(windows_check),
        OperatingSystem::MacOs => assert!(macos_check),
        OperatingSystem::Linux => assert!(linux_check),
        OperatingSystem::Unknown => {
            // If unknown, all checkers should be false
            assert!(!windows_check);
            assert!(!macos_check);
            assert!(!linux_check);
        }
    }
}

#[test]
fn test_os_name_consistency() {
    let detected_os = detect_os();
    let os_name = get_os_name();
    
    // Verify that the OS name matches the detected OS
    match detected_os {
        OperatingSystem::Windows => assert_eq!(os_name, "Windows"),
        OperatingSystem::MacOs => assert_eq!(os_name, "macOS"),
        OperatingSystem::Linux => assert_eq!(os_name, "Linux"),
        OperatingSystem::Unknown => assert_eq!(os_name, "Unknown"),
    }
}
