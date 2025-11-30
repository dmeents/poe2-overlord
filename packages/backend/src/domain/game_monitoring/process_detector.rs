use crate::domain::game_monitoring::{
    models::{GameMonitoringConfig, GameProcessStatus},
    traits::ProcessDetector,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::debug;
use sysinfo::System;

/// Concrete implementation of process detection for Path of Exile 2
///
/// Uses the sysinfo crate to scan running processes and identify POE2 game instances.
/// Supports configurable process name matching for flexibility across different game versions.
///
/// This implementation is specific to game monitoring and is part of the domain layer
/// rather than general infrastructure, as it's tightly coupled to game monitoring concerns.
pub struct ProcessDetectorImpl {
    config: GameMonitoringConfig,
}

impl ProcessDetectorImpl {
    /// Creates a new process detector with default configuration
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

#[async_trait]
impl ProcessDetector for ProcessDetectorImpl {
    /// Performs a system scan to detect POE2 game processes
    ///
    /// Efficiently scans running processes by only refreshing process information
    /// (not all system information) and matches against configured process names.
    /// Returns either a running process status or a "not running" status.
    async fn check_game_process(&self) -> AppResult<GameProcessStatus> {
        // Create system and refresh all information
        let mut system = System::new_all();
        system.refresh_all();

        // Scan processes for POE2 instances
        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

            // Check if any configured process name matches the current process
            if self
                .config
                .process_names
                .iter()
                .any(|name| process_name.contains(&name.to_lowercase()))
            {
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

        // No process found
        Ok(GameProcessStatus::not_running())
    }

    fn get_config(&self) -> &GameMonitoringConfig {
        &self.config
    }
}
