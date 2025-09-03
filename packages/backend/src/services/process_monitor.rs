use crate::errors::AppResult;
use crate::models::ProcessInfo;
use sysinfo::System;

/// Process monitor for checking POE2 process status
pub struct ProcessMonitor;

impl ProcessMonitor {
    /// Check if Path of Exile 2 process is running
    pub fn check_poe2_process() -> AppResult<ProcessInfo> {
        let mut system = System::new_all();
        system.refresh_all();

        // Define POE2 process names to search for
        let poe2_process_names = ["pathofexile2", "poe2", "pathofexile"];

        // Check for POE2 process names
        for (pid, process) in system.processes() {
            let process_name = process.name().to_string_lossy().to_lowercase();

            if poe2_process_names
                .iter()
                .any(|name| process_name.contains(name))
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
