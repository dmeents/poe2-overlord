use tauri::{Manager, Window, Emitter};
use serde::{Deserialize, Serialize};
use sysinfo::System;
use tokio::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct ProcessInfo {
    name: String,
    pid: u32,
    running: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct OverlayState {
    visible: bool,
    position: (i32, i32),
    size: (u32, u32),
    always_on_top: bool,
}

// Command to check if POE2 is running
#[tauri::command]
async fn check_poe2_process() -> Result<ProcessInfo, String> {
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

// Command to toggle overlay visibility
#[tauri::command]
async fn toggle_overlay_visibility(window: Window) -> Result<bool, String> {
    match window.is_visible() {
        Ok(visible) => {
            if visible {
                window.hide().map_err(|e| e.to_string())?;
                Ok(false)
            } else {
                window.show().map_err(|e| e.to_string())?;
                window.set_focus().map_err(|e| e.to_string())?;
                Ok(true)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

// Command to set window position
#[tauri::command]
async fn set_window_position(window: Window, x: i32, y: i32) -> Result<(), String> {
    use tauri::{LogicalPosition, Position};
    window.set_position(Position::Logical(LogicalPosition { x: x as f64, y: y as f64 }))
        .map_err(|e| e.to_string())
}

// Command to get window position
#[tauri::command]
async fn get_window_position(window: Window) -> Result<(i32, i32), String> {
    match window.outer_position() {
        Ok(position) => Ok((position.x, position.y)),
        Err(e) => Err(e.to_string()),
    }
}

// Command to set always on top
#[tauri::command]
async fn set_always_on_top(window: Window, always_on_top: bool) -> Result<(), String> {
    window.set_always_on_top(always_on_top)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_process::init())
    .invoke_handler(tauri::generate_handler![
        check_poe2_process,
        toggle_overlay_visibility,
        set_window_position,
        get_window_position,
        set_always_on_top
    ])
    .setup(|app| {
      // Setup logging
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // Get main window and configure it for overlay behavior
      if let Some(main_window) = app.get_webview_window("main") {
          log::info!("Configuring main window for overlay behavior");
          
          // Set overlay properties
          let _ = main_window.set_always_on_top(true);
          let _ = main_window.set_skip_taskbar(true);
          
          // Start process monitoring in the background using Tauri's async runtime
          let window_clone = main_window.clone();
          let app_handle = app.handle().clone();
          std::thread::spawn(move || {
              let rt = tokio::runtime::Runtime::new().unwrap();
              rt.block_on(async {
                  let mut interval = tokio::time::interval(Duration::from_secs(5));
                  loop {
                      interval.tick().await;
                      match check_poe2_process().await {
                          Ok(process_info) => {
                              let _ = window_clone.emit("poe2-process-status", &process_info);
                          }
                          Err(e) => {
                              log::error!("Error checking POE2 process: {}", e);
                          }
                      }
                  }
              });
          });
      }
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
