use crate::services::ProcessMonitor;

#[tauri::command]
pub async fn check_poe2_process() -> Result<crate::models::ProcessInfo, String> {
    ProcessMonitor::check_poe2_process()
}
