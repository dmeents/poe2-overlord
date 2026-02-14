use crate::domain::game_monitoring::{
    models::{GameMonitoringConfig, GameProcessStatus},
    traits::ProcessDetector,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::debug;
use std::ffi::OsString;
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

    /// Check if a path string contains POE2 identifiers.
    /// Handles both Unix paths and Wine-style paths (Z:\...\PathOfExileSteam.exe).
    fn path_contains_poe(&self, path_str: &str) -> bool {
        let path_lower = path_str.to_lowercase();

        // Skip if the path contains our own project directory (poe2-overlord)
        if path_lower.contains("poe2-overlord") {
            return false;
        }

        // Extract filename - handle both Unix (/) and Windows (\) separators
        let filename = path_str
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or("")
            .to_lowercase();

        // Check if the filename matches any POE2 process names
        for target in &self.config.process_names {
            let target_lower = target.to_lowercase();

            // Exact match or match with .exe extension
            if filename == target_lower || filename == format!("{}.exe", target_lower) {
                return true;
            }
        }

        // Also check for common POE2 executable patterns in filename
        filename.contains("pathofexile")
    }

    /// Check the command line for POE2 identifiers.
    /// On Linux/Proton, the exe path might be wine64-preloader, but the
    /// command line contains the actual Windows executable path.
    fn cmdline_contains_poe(&self, cmd: &[OsString]) -> bool {
        for arg in cmd {
            let arg_str = arg.to_string_lossy();
            if self.path_contains_poe(&arg_str) {
                return true;
            }
        }
        false
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
                    "Found POE2 process by name: {:?} (PID: {})",
                    process.name(),
                    pid.as_u32()
                );

                return Ok(GameProcessStatus::new(
                    process.name().to_string_lossy().to_string(),
                    pid.as_u32(),
                    true,
                ));
            }

            // On Linux/Proton, check the executable path
            if let Some(exe_path) = process.exe() {
                if self.path_contains_poe(&exe_path.to_string_lossy()) {
                    debug!(
                        "Found POE2 process by exe path: {} (PID: {})",
                        exe_path.display(),
                        pid.as_u32()
                    );

                    let display_name = exe_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| process_name.clone());

                    return Ok(GameProcessStatus::new(display_name, pid.as_u32(), true));
                }
            }

            // On Linux/Proton with Wine, the exe might be wine64-preloader
            // but the command line contains the actual game path
            let cmd = process.cmd();
            if !cmd.is_empty() && self.cmdline_contains_poe(cmd) {
                debug!(
                    "Found POE2 process by cmdline: {:?} (PID: {})",
                    cmd,
                    pid.as_u32()
                );

                // Extract game name from command line
                let display_name = cmd
                    .iter()
                    .map(|arg| arg.to_string_lossy())
                    .find(|arg| self.path_contains_poe(arg))
                    .and_then(|path| path.rsplit(['/', '\\']).next().map(|s| s.to_string()))
                    .unwrap_or_else(|| "PathOfExile".to_string());

                return Ok(GameProcessStatus::new(display_name, pid.as_u32(), true));
            }
        }

        Ok(GameProcessStatus::not_running())
    }

    fn get_config(&self) -> &GameMonitoringConfig {
        &self.config
    }
}
