use crate::errors::AppResult;
use crate::models::ProcessInfo;
use sysinfo::System;

pub struct ProcessMonitor;

impl ProcessMonitor {
    /// Check if Path of Exile 2 process is running
    pub fn check_poe2_process() -> AppResult<ProcessInfo> {
        let mut system = System::new_all();
        system.refresh_all();

        // Check for POE2 process names
        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

            if process_name.contains("pathofexile2")
                || process_name.contains("poe2")
                || process_name.contains("pathofexile")
            {
                return Ok(ProcessInfo {
                    name: process.name().to_string_lossy().to_string(),
                    pid: pid.as_u32(),
                    running: true,
                });
            }
        }

        Ok(ProcessInfo {
            name: "Not Found".to_string(),
            pid: 0,
            running: false,
        })
    }
}
