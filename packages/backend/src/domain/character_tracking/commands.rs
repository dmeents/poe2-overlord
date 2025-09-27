use crate::domain::character_tracking::{
    models::CharacterTrackingData,
    traits::CharacterTrackingService,
};
use crate::infrastructure::tauri::{to_command_result, CommandResult};
use log::debug;
use std::sync::Arc;
use tauri::State;

/// Tauri command to get complete character tracking data for a character
#[tauri::command]
pub async fn get_character_tracking_data(
    character_id: String,
    character_tracking_service: State<'_, Arc<dyn CharacterTrackingService>>,
) -> CommandResult<Option<CharacterTrackingData>> {
    debug!("Getting character tracking data for character: {}", character_id);

    let data = to_command_result(
        character_tracking_service
            .get_character_data(&character_id)
            .await,
    )?;

    debug!("Retrieved character tracking data for character");
    Ok(data)
}




