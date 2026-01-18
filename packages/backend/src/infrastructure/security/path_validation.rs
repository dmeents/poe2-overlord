//! Path validation for security - prevents path traversal attacks
//!
//! This module provides validation for file paths to prevent security vulnerabilities
//! such as path traversal attacks (../) and access to unauthorized directories.

use crate::errors::{AppError, AppResult};
use crate::infrastructure::expand_tilde;
use std::path::{Path, PathBuf};

/// Validates file paths for security vulnerabilities
///
/// PathValidator ensures that file paths:
/// 1. Do not contain path traversal sequences (..)
/// 2. Are within allowed root directories
/// 3. Have valid file extensions for their use case
/// 4. Are properly canonicalized (symlinks resolved)
pub struct PathValidator {
    /// Allowed base directories for file access
    allowed_roots: Vec<PathBuf>,
    /// Allowed file extensions (e.g., ["txt", "log"])
    allowed_extensions: Vec<String>,
}

impl PathValidator {
    /// Create a new PathValidator for POE log files
    ///
    /// This validator is configured with platform-specific allowed directories
    /// where POE2 is typically installed, and only allows .txt and .log extensions.
    pub fn new_for_poe_logs() -> Self {
        Self {
            allowed_roots: Self::get_allowed_poe_roots(),
            allowed_extensions: vec!["txt".to_string(), "log".to_string()],
        }
    }

    /// Get platform-specific allowed installation directories
    fn get_allowed_poe_roots() -> Vec<PathBuf> {
        let mut roots = Vec::new();

        // Add home directory - POE can be installed in various user locations
        if let Some(home) = dirs::home_dir() {
            roots.push(home.clone());

            // Common game locations within home
            roots.push(home.join(".var")); // Flatpak apps
            roots.push(home.join(".local")); // Local apps
            roots.push(home.join("Games")); // Common game directory
            roots.push(home.join("games")); // Lowercase variant
        }

        #[cfg(target_os = "windows")]
        {
            // Windows-specific paths
            roots.push(PathBuf::from("C:\\Program Files (x86)"));
            roots.push(PathBuf::from("C:\\Program Files"));
            roots.push(PathBuf::from("C:\\Games"));
            roots.push(PathBuf::from("D:\\Games"));
            roots.push(PathBuf::from("D:\\Program Files"));
            roots.push(PathBuf::from("D:\\Program Files (x86)"));
            roots.push(PathBuf::from("E:\\Games"));
            roots.push(PathBuf::from("E:\\Program Files"));

            // Steam common locations
            if let Some(home) = dirs::home_dir() {
                roots.push(home.join("AppData"));
            }
        }

        #[cfg(target_os = "macos")]
        {
            roots.push(PathBuf::from("/Applications"));
            if let Some(home) = dirs::home_dir() {
                roots.push(home.join("Library/Application Support"));
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Linux-specific paths (Steam, Lutris, etc.)
            roots.push(PathBuf::from("/opt"));
            roots.push(PathBuf::from("/usr/share"));
        }

        roots
    }

    /// Validate a path for security vulnerabilities
    ///
    /// Returns the canonicalized path if valid, or an error if the path
    /// fails any security check.
    pub fn validate_path(&self, path: &str) -> AppResult<PathBuf> {
        // Step 1: Basic checks
        if path.trim().is_empty() {
            return Err(AppError::validation_error(
                "validate_path",
                "Path cannot be empty",
            ));
        }

        // Step 2: Expand tilde
        let expanded = expand_tilde(path);

        // Step 3: Detect path traversal patterns BEFORE canonicalization
        self.check_path_traversal(&expanded)?;

        // Step 4: Canonicalize to absolute path
        let canonical = self.canonicalize_safe(&expanded)?;

        // Step 5: Verify within allowed roots
        self.check_allowed_roots(&canonical)?;

        // Step 6: Verify file extension
        self.check_file_extension(&canonical)?;

        Ok(canonical)
    }

    /// Check for path traversal sequences
    ///
    /// Detects ".." sequences that could be used to escape allowed directories.
    /// This check is performed BEFORE canonicalization to catch attack attempts
    /// that might be normalized away.
    fn check_path_traversal(&self, path: &Path) -> AppResult<()> {
        let path_str = path.to_string_lossy();

        // Check for explicit traversal sequences
        // This catches both Unix (..) and Windows (..\) styles
        if path_str.contains("..") {
            return Err(AppError::security_error(
                "path_traversal_check",
                "Path contains directory traversal sequence (..)",
            ));
        }

        Ok(())
    }

    /// Safely canonicalize path
    ///
    /// Resolves the path to an absolute canonical form, handling cases where
    /// the file may not exist yet (validates parent directory instead).
    fn canonicalize_safe(&self, path: &Path) -> AppResult<PathBuf> {
        // If the path exists, canonicalize it directly
        if path.exists() {
            return path.canonicalize().map_err(|e| {
                AppError::file_system_error(
                    "canonicalize_path",
                    &format!("Failed to canonicalize path: {}", e),
                )
            });
        }

        // For non-existent files, we need to:
        // 1. Find the deepest existing ancestor
        // 2. Canonicalize that ancestor
        // 3. Append the remaining path components
        let mut existing_ancestor = path.to_path_buf();
        let mut remaining_components: Vec<_> = Vec::new();

        while !existing_ancestor.exists() {
            if let Some(file_name) = existing_ancestor.file_name() {
                remaining_components.push(file_name.to_os_string());
            }
            if let Some(parent) = existing_ancestor.parent() {
                existing_ancestor = parent.to_path_buf();
            } else {
                // No existing ancestor found - path is completely invalid
                return Err(AppError::validation_error(
                    "canonicalize_path",
                    "No part of the path exists",
                ));
            }
        }

        // Canonicalize the existing ancestor
        let canonical_ancestor = existing_ancestor.canonicalize().map_err(|e| {
            AppError::file_system_error(
                "canonicalize_path",
                &format!("Failed to canonicalize existing path: {}", e),
            )
        })?;

        // Rebuild the full path with remaining components (in reverse order)
        let mut result = canonical_ancestor;
        for component in remaining_components.into_iter().rev() {
            result = result.join(component);
        }

        Ok(result)
    }

    /// Verify path is within allowed root directories
    ///
    /// The canonicalized path must start with one of the allowed root directories.
    /// This prevents access to sensitive system directories.
    fn check_allowed_roots(&self, canonical: &Path) -> AppResult<()> {
        // Check if canonical path starts with any allowed root
        let is_allowed = self.allowed_roots.iter().any(|root| {
            // Handle case where root doesn't exist (can't canonicalize)
            if let Ok(canonical_root) = root.canonicalize() {
                canonical.starts_with(&canonical_root)
            } else {
                // If root can't be canonicalized, check string prefix
                canonical.starts_with(root)
            }
        });

        if !is_allowed {
            return Err(AppError::security_error(
                "path_root_check",
                &format!(
                    "Path '{}' is outside allowed directories. POE log files must be in game installation or home directories.",
                    canonical.display()
                ),
            ));
        }

        Ok(())
    }

    /// Verify file has an allowed extension
    fn check_file_extension(&self, path: &Path) -> AppResult<()> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        match extension {
            Some(ext) if self.allowed_extensions.contains(&ext) => Ok(()),
            Some(ext) => Err(AppError::validation_error(
                "file_extension_check",
                &format!(
                    "Invalid file extension '{}'. Expected: {}",
                    ext,
                    self.allowed_extensions.join(", ")
                ),
            )),
            None => Err(AppError::validation_error(
                "file_extension_check",
                &format!(
                    "File must have an extension. Expected: {}",
                    self.allowed_extensions.join(", ")
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rejects_path_traversal() {
        let validator = PathValidator::new_for_poe_logs();

        // Basic traversal
        let result = validator.validate_path("../../../etc/passwd");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("traversal"));

        // Traversal in middle of path
        let result = validator.validate_path("/home/user/../../../etc/passwd");
        assert!(result.is_err());
    }

    #[test]
    fn test_rejects_empty_path() {
        let validator = PathValidator::new_for_poe_logs();

        assert!(validator.validate_path("").is_err());
        assert!(validator.validate_path("   ").is_err());
        assert!(validator.validate_path("\t\n").is_err());
    }

    #[test]
    fn test_rejects_invalid_extension() {
        let validator = PathValidator::new_for_poe_logs();

        // These paths would be in home dir (allowed root) but wrong extension
        let home = dirs::home_dir().unwrap_or_default();
        let exe_path = home.join("test.exe");
        let result = validator.validate_path(&exe_path.to_string_lossy());

        // Should fail on extension check (might fail earlier on existence)
        assert!(result.is_err());
    }

    #[test]
    fn test_accepts_txt_extension() {
        let validator = PathValidator::new_for_poe_logs();

        // Create a path in home dir with .txt extension
        let home = dirs::home_dir().unwrap_or_default();
        let txt_path = home.join("test.txt");

        // If this file doesn't exist, it should still pass extension check
        // (might fail on existence check, but that's different)
        let result = validator.validate_path(&txt_path.to_string_lossy());

        // Should not fail due to extension
        if let Err(e) = &result {
            assert!(!e.to_string().contains("extension"));
        }
    }

    #[test]
    fn test_accepts_log_extension() {
        let validator = PathValidator::new_for_poe_logs();

        let home = dirs::home_dir().unwrap_or_default();
        let log_path = home.join("test.log");

        let result = validator.validate_path(&log_path.to_string_lossy());

        // Should not fail due to extension
        if let Err(e) = &result {
            assert!(!e.to_string().contains("extension"));
        }
    }

    #[test]
    fn test_expands_tilde() {
        let validator = PathValidator::new_for_poe_logs();

        // Path with tilde should be expanded
        let result = validator.validate_path("~/test.txt");

        // Should not fail due to tilde - might fail for other reasons
        if let Err(e) = &result {
            assert!(!e.to_string().contains("~"));
        }
    }

    #[test]
    fn test_rejects_sensitive_system_paths() {
        let validator = PathValidator::new_for_poe_logs();

        // These should fail as they're outside allowed roots
        #[cfg(unix)]
        {
            let result = validator.validate_path("/etc/passwd.txt");
            assert!(result.is_err());

            let result = validator.validate_path("/root/secret.txt");
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_path_traversal_patterns() {
        let validator = PathValidator::new_for_poe_logs();

        // Various traversal patterns
        let patterns = vec![
            "../test.txt",
            "..\\test.txt",
            "foo/../bar/../../../etc/passwd",
            "./../../test.txt",
            "test/../../../etc/passwd",
        ];

        for pattern in patterns {
            let result = validator.validate_path(pattern);
            assert!(result.is_err(), "Pattern '{}' should be rejected", pattern);
        }
    }
}
