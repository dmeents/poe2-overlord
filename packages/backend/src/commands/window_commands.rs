use crate::services::WindowManager;
use tauri::Window;

#[tauri::command]
pub async fn toggle_overlay_visibility(window: Window) -> Result<bool, String> {
    WindowManager::toggle_overlay_visibility(&window)
}

#[tauri::command]
pub async fn set_window_position(window: Window, x: i32, y: i32) -> Result<(), String> {
    WindowManager::set_window_position(&window, x, y)
}

#[tauri::command]
pub async fn get_window_position(window: Window) -> Result<(i32, i32), String> {
    WindowManager::get_window_position(&window)
}

#[tauri::command]
pub async fn set_always_on_top(window: Window, always_on_top: bool) -> Result<(), String> {
    WindowManager::set_always_on_top(&window, always_on_top)
}
