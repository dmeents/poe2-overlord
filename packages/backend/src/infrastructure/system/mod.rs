//! System-level infrastructure for OS detection and path management
//! 
//! Provides cross-platform utilities for detecting the operating system
//! and managing platform-specific file paths, particularly for POE2 log files.

pub mod os_detection;
pub mod paths;

pub use os_detection::{detect_os, get_os_name, OperatingSystem};
pub use paths::PoeClientLogPaths;
