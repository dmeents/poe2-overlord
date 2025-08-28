use app_lib::utils::{detect_os, get_os_name, OperatingSystem};

#[test]
fn test_detect_os() {
    let os = detect_os();
    assert!(matches!(
        os,
        OperatingSystem::Windows | OperatingSystem::MacOs | OperatingSystem::Linux | OperatingSystem::Unknown
    ));
}

#[test]
fn test_get_os_name() {
    let os_name = get_os_name();
    assert!(!os_name.is_empty());
    assert!(matches!(
        os_name,
        "Windows" | "macOS" | "Linux" | "Unknown"
    ));
}

#[test]
fn test_os_consistency() {
    let detected_os = detect_os();
    let os_name = get_os_name();
    
    match detected_os {
        OperatingSystem::Windows => assert_eq!(os_name, "Windows"),
        OperatingSystem::MacOs => assert_eq!(os_name, "macOS"),
        OperatingSystem::Linux => assert_eq!(os_name, "Linux"),
        OperatingSystem::Unknown => assert_eq!(os_name, "Unknown"),
    }
}

#[test]
fn test_os_enum_variants() {
    // Test that all enum variants can be created and compared
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
    
    assert_eq!(windows, OperatingSystem::Windows);
    assert_eq!(macos, OperatingSystem::MacOs);
    assert_eq!(linux, OperatingSystem::Linux);
    assert_eq!(unknown, OperatingSystem::Unknown);
}

#[test]
fn test_os_detection_platform_specific() {
    let detected_os = detect_os();
    
    #[cfg(target_os = "windows")]
    {
        assert_eq!(detected_os, OperatingSystem::Windows);
    }
    
    #[cfg(target_os = "macos")]
    {
        assert_eq!(detected_os, OperatingSystem::MacOs);
    }
    
    #[cfg(target_os = "linux")]
    {
        assert_eq!(detected_os, OperatingSystem::Linux);
    }
}
