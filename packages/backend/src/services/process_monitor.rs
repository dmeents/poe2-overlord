use crate::errors::AppResult;
use crate::models::ProcessInfo;
use std::collections::HashSet;
use sysinfo::System;

pub struct ProcessMonitor {
    process_names: HashSet<String>,
}

impl Default for ProcessMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessMonitor {
    pub fn new() -> Self {
        let mut process_names = HashSet::new();
        process_names.insert("pathofexile".to_string());
        process_names.insert("pathofexile2".to_string());
        process_names.insert("poe2".to_string());
        
        Self { process_names }
    }

    /// Add a process name to monitor
    pub fn add_process_name(&mut self, name: String) {
        self.process_names.insert(name.to_lowercase());
    }

    /// Remove a process name from monitoring
    pub fn remove_process_name(&mut self, name: &str) {
        self.process_names.remove(&name.to_lowercase());
    }

    /// Get all monitored process names
    pub fn get_process_names(&self) -> Vec<String> {
        self.process_names.iter().cloned().collect()
    }

    pub fn check_poe2_process() -> AppResult<ProcessInfo> {
        let monitor = Self::new();
        monitor.check_processes()
    }

    /// Check for any of the monitored processes
    pub fn check_processes(&self) -> AppResult<ProcessInfo> {
        let mut system = System::new_all();
        system.refresh_all();

        for (pid, process) in system.processes() {
            let process_name_str = process.name().to_string_lossy().to_lowercase();
            
            if self.process_names.iter().any(|name| process_name_str.contains(name)) {
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
