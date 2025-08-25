use crate::models::ProcessInfo;
use sysinfo::System;

pub struct ProcessMonitor;

impl ProcessMonitor {
    pub fn new() -> Self {
        Self
    }

    pub fn check_poe2_process() -> Result<ProcessInfo, String> {
        let mut system = System::new_all();
        system.refresh_all();
        
        // Look for Path of Exile 2 process (adjust process name as needed)
        let poe2_processes = ["pathofexile_x64steam.exe", "pathofexile_x64.exe", "pathofexile.exe", "poe2"];
        
        for (pid, process) in system.processes() {
            // Convert process name to string and make it lowercase for comparison
            let process_name_str = process.name().to_string_lossy().to_lowercase();
            for poe2_name in &poe2_processes {
                if process_name_str.contains(poe2_name) {
                    return Ok(ProcessInfo {
                        name: process.name().to_string_lossy().to_string(),
                        pid: pid.as_u32(),
                        running: true,
                    });
                }
            }
        }
        
        Ok(ProcessInfo {
            name: "Not Found".to_string(),
            pid: 0,
            running: false,
        })
    }
}
