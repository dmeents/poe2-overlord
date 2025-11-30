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

#[async_trait]
impl ProcessDetector for ProcessDetectorImpl {
    async fn check_game_process(&self) -> AppResult<GameProcessStatus> {
        let mut system = System::new_all();
        system.refresh_all();

        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

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

        Ok(GameProcessStatus::not_running())
    }

    fn get_config(&self) -> &GameMonitoringConfig {
        &self.config
    }
}
