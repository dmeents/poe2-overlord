use crate::domain::walkthrough::models::{CharacterWalkthroughProgress, WalkthroughGuide};
use crate::domain::walkthrough::traits::WalkthroughService;
use crate::{to_command_result, CommandResult};
use tauri::State;

/// Tauri command to get the complete walkthrough guide
///
/// This command retrieves the full walkthrough guide with all acts and steps.
/// The guide contains the complete campaign structure with objectives and navigation.
#[tauri::command]
pub async fn get_walkthrough_guide(
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<WalkthroughGuide> {
    let result = to_command_result(walkthrough_service.get_guide().await)?;
    Ok(result)
}

/// Tauri command to get a character's walkthrough progress
///
/// This command retrieves a character's current progress through the walkthrough
/// including navigation context (next/previous step IDs).
#[tauri::command]
pub async fn get_character_walkthrough_progress(
    character_id: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<CharacterWalkthroughProgress> {
    let result = to_command_result(
        walkthrough_service
            .get_character_progress(&character_id)
            .await,
    )?;
    Ok(result)
}

/// Tauri command to update a character's walkthrough progress
///
/// This command allows updating any aspect of a character's walkthrough progress
/// including current step, completion status, and timestamps.
#[tauri::command]
pub async fn update_character_walkthrough_progress(
    character_id: String,
    progress: crate::domain::walkthrough::models::WalkthroughProgress,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<()> {
    to_command_result(
        walkthrough_service
            .update_character_progress(&character_id, progress)
            .await,
    )?;
    Ok(())
}
