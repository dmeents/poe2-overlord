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
}

#[async_trait]
impl ProcessDetector for ProcessDetectorImpl {
    async fn check_game_process(&self) -> AppResult<GameProcessStatus> {
        let mut system = System::new_all();
        system.refresh_all();

        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

            // Use exact matching to avoid false positives (e.g., "mypoe2tool")
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
        }

        Ok(GameProcessStatus::not_running())
    }

    fn get_config(&self) -> &GameMonitoringConfig {
        &self.config
    }
}
