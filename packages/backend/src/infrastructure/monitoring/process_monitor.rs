use crate::domain::game_monitoring::{
    models::{GameProcessStatus, GameMonitoringConfig},
    traits::ProcessDetector,
};
use crate::errors::AppResult;
use async_trait::async_trait;
use log::debug;
use sysinfo::System;

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

    fn check_for_processes(&self, system: &System) -> Option<GameProcessStatus> {
        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

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
