use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
    pub running: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverlayState {
    pub visible: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub always_on_top: bool,
}
