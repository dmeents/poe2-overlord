use crate::domain::game_monitoring::{
    models::{GameProcessStatus, GameMonitoringConfig},
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
pub struct ProcessMonitorImpl {
    config: GameMonitoringConfig,
}

impl ProcessMonitorImpl {
    pub fn new() -> Self {
        Self {
            config: GameMonitoringConfig::default(),
        }
    }

    pub fn with_config(config: GameMonitoringConfig) -> Self {
        Self { config }
    }

    /// Scans all running processes to find POE2 game instances
    /// 
    /// Iterates through the system process list and matches against configured process names.
    /// Returns the first matching process found, or None if no POE2 processes are detected.
    fn check_for_processes(&self, system: &System) -> Option<GameProcessStatus> {
        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

            // Check if any configured process name matches the current process
            if self.config.process_names
                .iter()
                .any(|name| process_name.contains(&name.to_lowercase()))
            {
                debug!("Found POE2 process: {:?} (PID: {})", process.name(), pid.as_u32());
                return Some(GameProcessStatus::new(
                    process.name().to_string_lossy().to_string(),
                    pid.as_u32(),
                    true,
                ));
            }
        }

        None
    }
}

impl Default for ProcessMonitorImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProcessDetector for ProcessMonitorImpl {
    /// Performs a complete system scan to detect POE2 game processes
    /// 
    /// Refreshes the system information and scans all running processes.
    /// Returns either a running process status or a "not running" status.
    async fn check_game_process(&self) -> AppResult<GameProcessStatus> {
        debug!("Checking for Path of Exile 2 process...");
        
        let mut system = System::new_all();
        system.refresh_all();

        match self.check_for_processes(&system) {
            Some(status) => {
                debug!("POE2 process found: {} (PID: {})", status.name, status.pid);
                Ok(status)
            }
            None => {
                debug!("POE2 process not found");
                Ok(GameProcessStatus::not_running())
            }
        }
    }

    fn get_config(&self) -> &GameMonitoringConfig {
        &self.config
    }
}
