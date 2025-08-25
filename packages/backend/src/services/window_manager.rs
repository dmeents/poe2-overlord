use tauri::{LogicalPosition, Position, WebviewWindow, Window};

pub struct WindowManager;

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WindowManager {
    pub fn new() -> Self {
        Self
    }

    pub fn toggle_overlay_visibility(window: &Window) -> Result<bool, String> {
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

    pub fn set_window_position(window: &Window, x: i32, y: i32) -> Result<(), String> {
        window
            .set_position(Position::Logical(LogicalPosition {
                x: x as f64,
                y: y as f64,
            }))
            .map_err(|e| e.to_string())
    }

    pub fn get_window_position(window: &Window) -> Result<(i32, i32), String> {
        match window.outer_position() {
            Ok(position) => Ok((position.x, position.y)),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn set_always_on_top(window: &Window, always_on_top: bool) -> Result<(), String> {
        window
            .set_always_on_top(always_on_top)
            .map_err(|e| e.to_string())
    }

    pub fn configure_overlay_window(window: &WebviewWindow) -> Result<(), String> {
        window.set_always_on_top(true).map_err(|e| e.to_string())?;
        window.set_skip_taskbar(true).map_err(|e| e.to_string())?;
        Ok(())
    }
}
