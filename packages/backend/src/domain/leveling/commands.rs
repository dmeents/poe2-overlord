use std::sync::Arc;
use tauri::State;

use crate::{to_command_result, CommandResult};

use super::models::LevelingStats;
use super::traits::LevelingService;

#[tauri::command]
pub async fn get_leveling_stats(
    character_id: String,
    leveling_service: State<'_, Arc<dyn LevelingService + Send + Sync>>,
) -> CommandResult<LevelingStats> {
    to_command_result(leveling_service.get_leveling_stats(&character_id).await)
}
