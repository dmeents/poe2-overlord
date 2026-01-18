use crate::domain::game_monitoring::{
    models::{GameMonitoringConfig, GameProcessStatus},
    traits::ProcessDetector,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::debug;
use sysinfo::System;

pub struct ProcessDetectorImpl {
    config: GameMonitoringConfig,
}

impl ProcessDetectorImpl {
    pub fn new() -> Self {
        Self {
            config: GameMonitoringConfig::default(),
        }
    }
}

impl Default for ProcessDetectorImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessDetectorImpl {
    /// Matches process names exactly to avoid false positives.
    /// Handles both with and without .exe extension.
    fn matches_poe_process(&self, process_name: &str) -> bool {
        for target in &self.config.process_names {
            let target_lower = target.to_lowercase();

            // Exact match
            if process_name == target_lower {
                return true;
            }

            // Match with .exe extension if target doesn't have it
            if !target_lower.ends_with(".exe") && process_name == format!("{}.exe", target_lower) {
                return true;
            }
        }

        false
    }

    /// On Linux/Proton, the process name might be generic (e.g., "Main").
    /// Check the executable path for POE2 identifiers.
    fn matches_poe_exe_path(&self, exe_path: &std::path::Path) -> bool {
        let path_str = exe_path.to_string_lossy().to_lowercase();

        // Check if the path contains any POE2 identifiers
        for target in &self.config.process_names {
            let target_lower = target.to_lowercase();
            if path_str.contains(&target_lower) {
                return true;
            }
        }

        // Also check for common POE2 path patterns (Wine/Proton paths)
        path_str.contains("path of exile 2") || path_str.contains("pathofexile2")
    }
}

#[async_trait]
impl ProcessDetector for ProcessDetectorImpl {
    async fn check_game_process(&self) -> AppResult<GameProcessStatus> {
        let mut system = System::new_all();
        system.refresh_all();

        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

            // First try exact process name matching (works on Windows)
            if self.matches_poe_process(&process_name) {
                debug!(
                    "Found POE2 process: {:?} (PID: {})",
                    process.name(),
                    pid.as_u32()
                );

                return Ok(GameProcessStatus::new(
                    process.name().to_string_lossy().to_string(),
                    pid.as_u32(),
                    true,
                ));
            }

            // On Linux/Proton, process names can be generic (e.g., "Main")
            // Check the executable path as a fallback
            if let Some(exe_path) = process.exe() {
                if self.matches_poe_exe_path(exe_path) {
                    debug!(
                        "Found POE2 process by exe path: {} (PID: {})",
                        exe_path.display(),
                        pid.as_u32()
                    );

                    // Use the exe filename as the display name
                    let display_name = exe_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| process_name.clone());

                    return Ok(GameProcessStatus::new(
                        display_name,
                        pid.as_u32(),
                        true,
                    ));
                }
            }
        }

        Ok(GameProcessStatus::not_running())
    }

    fn get_config(&self) -> &GameMonitoringConfig {
        &self.config
    }
}
