use crate::services::process_monitor::ProcessMonitor;

#[tauri::command]
pub async fn check_poe2_process() -> Result<crate::models::ProcessInfo, String> {
    ProcessMonitor::check_poe2_process().map_err(|e| format!("Failed to check POE2 process: {}", e))
}
