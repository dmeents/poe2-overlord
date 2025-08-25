use crate::services::ProcessMonitor;
use log;
use tauri::{App, Emitter, Manager, WebviewWindow};
use tokio::time::{interval, Duration};

pub fn setup_app(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    if cfg!(debug_assertions) {
        app.handle().plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )?;
    }

    // Get main window and start process monitoring
    if let Some(main_window) = app.get_webview_window("main") {
        log::info!("Starting POE2 process monitoring");

        // Start process monitoring in the background
        start_process_monitoring(main_window);
    }

    Ok(())
}

fn start_process_monitoring(window: WebviewWindow) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut interval = interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                match ProcessMonitor::check_poe2_process() {
                    Ok(process_info) => {
                        let _ = window.emit("poe2-process-status", &process_info);
                    }
                    Err(e) => {
                        log::error!("Error checking POE2 process: {}", e);
                    }
                }
            }
        });
    });
}
