use crate::errors::AppResult;
use crate::models::ProcessInfo;
use sysinfo::System;

/// Process detector for checking Path of Exile 2 process status
pub struct ProcessDetector;

impl ProcessDetector {
    /// Check if Path of Exile 2 game process is running
    pub fn check_game_process() -> AppResult<ProcessInfo> {
        let mut system = System::new_all();
        system.refresh_all();

        // Define POE2 process names to search for
        // Include both the standalone and Steam versions
        let poe2_process_names = [
            "pathofexile2",
            "poe2",
            "pathofexile",
            "pathofexilesteam",
            "pathofexilesteam.exe",
            "pathofexile2.exe",
            "pathofexile.exe",
        ];

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
