use tauri::State;
use log::debug;
use crate::domain::walkthrough::models::{WalkthroughGuide, CharacterWalkthroughProgress};
use crate::domain::walkthrough::traits::WalkthroughService;
use crate::infrastructure::tauri::CommandResult;
use crate::infrastructure::tauri::to_command_result;

/// Tauri command to get the complete walkthrough guide
///
/// This command retrieves the full walkthrough guide with all acts and steps.
#[tauri::command]
pub async fn get_walkthrough_guide(
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<WalkthroughGuide> {
    debug!("🔍 WALKTHROUGH COMMAND: Getting walkthrough guide");
    
    let result = to_command_result(walkthrough_service.get_guide().await)?;
    debug!("✅ WALKTHROUGH COMMAND: Retrieved walkthrough guide");
    Ok(result)
}

/// Tauri command to get a specific walkthrough step
///
/// This command retrieves a specific step by its ID.
#[tauri::command]
pub async fn get_walkthrough_step(
    step_id: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<Option<crate::domain::walkthrough::models::WalkthroughStepResult>> {
    debug!("🔍 WALKTHROUGH COMMAND: Getting walkthrough step {}", step_id);
    
    let result = to_command_result(walkthrough_service.get_step(&step_id).await)?;
    debug!("✅ WALKTHROUGH COMMAND: Retrieved walkthrough step {}", step_id);
    Ok(result)
}

/// Tauri command to get a character's walkthrough progress
///
/// This command retrieves a character's current progress through the walkthrough.
#[tauri::command]
pub async fn get_character_walkthrough_progress(
    character_id: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<CharacterWalkthroughProgress> {
    debug!("🔍 WALKTHROUGH COMMAND: Getting character {} walkthrough progress", character_id);
    
    let result = to_command_result(walkthrough_service.get_character_progress(&character_id).await)?;
    debug!("✅ WALKTHROUGH COMMAND: Retrieved character {} walkthrough progress", character_id);
    Ok(result)
}

/// Tauri command to advance a character to the next walkthrough step
///
/// This command moves a character forward to the next step in the walkthrough.
#[tauri::command]
pub async fn advance_character_walkthrough_step(
    character_id: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<()> {
    debug!("🔍 WALKTHROUGH COMMAND: Advancing character {} to next step", character_id);
    
    to_command_result(walkthrough_service.advance_character_to_next_step(&character_id).await)?;
    debug!("✅ WALKTHROUGH COMMAND: Advanced character {} to next step", character_id);
    Ok(())
}

/// Tauri command to move a character to a specific walkthrough step
///
/// This command allows manual navigation to any step in the walkthrough.
#[tauri::command]
pub async fn move_character_to_walkthrough_step(
    character_id: String,
    step_id: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<()> {
    debug!("🔍 WALKTHROUGH COMMAND: Moving character {} to step {}", character_id, step_id);
    
    to_command_result(walkthrough_service.move_character_to_step(&character_id, &step_id).await)?;
    debug!("✅ WALKTHROUGH COMMAND: Moved character {} to step {}", character_id, step_id);
    Ok(())
}

/// Tauri command to mark a character's campaign as completed
///
/// This command marks the character as having completed the entire campaign.
#[tauri::command]
pub async fn mark_character_campaign_completed(
    character_id: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<()> {
    debug!("🔍 WALKTHROUGH COMMAND: Marking character {} campaign as completed", character_id);
    
    to_command_result(walkthrough_service.mark_character_campaign_completed(&character_id).await)?;
    debug!("✅ WALKTHROUGH COMMAND: Marked character {} campaign as completed", character_id);
    Ok(())
}

/// Tauri command to handle a scene change for walkthrough progress detection
///
/// This command is called when a character enters a new zone to check for walkthrough progress.
#[tauri::command]
pub async fn handle_walkthrough_scene_change(
    character_id: String,
    zone_name: String,
    walkthrough_service: State<'_, Box<dyn WalkthroughService + Send + Sync>>,
) -> CommandResult<()> {
    debug!("🔍 WALKTHROUGH COMMAND: Handling scene change for character {} to zone {}", character_id, zone_name);
    
    to_command_result(walkthrough_service.handle_scene_change(&character_id, &zone_name).await)?;
    debug!("✅ WALKTHROUGH COMMAND: Handled scene change for character {} to zone {}", character_id, zone_name);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::domain::walkthrough::models::{WalkthroughGuide, WalkthroughAct, WalkthroughStep, Objective, WalkthroughProgress, CharacterWalkthroughProgress, WalkthroughStepResult};
    use crate::domain::walkthrough::traits::WalkthroughService;
    use crate::errors::AppError;
    use async_trait::async_trait;

    // Mock implementation for testing
    struct MockWalkthroughService;

    #[async_trait]
    impl WalkthroughService for MockWalkthroughService {
        async fn get_guide(&self) -> Result<WalkthroughGuide, AppError> {
            let mut objectives = Vec::new();
            objectives.push(Objective {
                text: "Test objective".to_string(),
                details: Some("Test details".to_string()),
                required: true,
                rewards: vec!["Test reward".to_string()],
                notes: Some("Test notes".to_string()),
            });

            let mut steps = HashMap::new();
            steps.insert("act_4_step_1".to_string(), WalkthroughStep {
                id: "act_4_step_1".to_string(),
                title: "Test Step".to_string(),
                description: "Test description".to_string(),
                objectives,
                current_zone: "Test Zone".to_string(),
                completion_zone: "Next Zone".to_string(),
                next_step_id: Some("act_4_step_2".to_string()),
                previous_step_id: None,
                wiki_items: vec!["Test Item".to_string()],
            });

            let mut acts = HashMap::new();
            acts.insert("act_4".to_string(), WalkthroughAct {
                act_name: "Act 4".to_string(),
                act_number: 4,
                steps,
            });

            Ok(WalkthroughGuide { acts })
        }

        async fn get_step(&self, step_id: &str) -> Result<Option<WalkthroughStepResult>, AppError> {
            if step_id == "act_4_step_1" {
                Ok(Some(WalkthroughStepResult {
                    step: WalkthroughStep {
                        id: "act_4_step_1".to_string(),
                        title: "Test Step".to_string(),
                        description: "Test description".to_string(),
                        objectives: vec![],
                        current_zone: "Test Zone".to_string(),
                        completion_zone: "Next Zone".to_string(),
                        next_step_id: Some("act_4_step_2".to_string()),
                        previous_step_id: None,
                        wiki_items: vec!["Test Item".to_string()],
                    },
                    act_name: "Act 4".to_string(),
                    act_number: 4,
                }))
            } else {
                Ok(None)
            }
        }

        async fn get_character_progress(&self, _character_id: &str) -> Result<CharacterWalkthroughProgress, AppError> {
            Ok(CharacterWalkthroughProgress {
                progress: WalkthroughProgress::new(),
                current_step: None,
                next_step: None,
                previous_step: None,
            })
        }

        async fn update_character_progress(&self, _character_id: &str, _progress: WalkthroughProgress) -> Result<(), AppError> {
            Ok(())
        }

        async fn advance_character_to_next_step(&self, _character_id: &str) -> Result<(), AppError> {
            Ok(())
        }

        async fn move_character_to_step(&self, _character_id: &str, _step_id: &str) -> Result<(), AppError> {
            Ok(())
        }

        async fn mark_character_campaign_completed(&self, _character_id: &str) -> Result<(), AppError> {
            Ok(())
        }

        async fn handle_scene_change(&self, _character_id: &str, _zone_name: &str) -> Result<(), AppError> {
            Ok(())
        }
    }

    // Note: Tauri command tests require proper State setup which is complex
    // These tests are disabled for now but the command functions are tested
    // through integration tests
}
