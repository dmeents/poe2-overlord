use log::{debug, info, warn};
use std::sync::Arc;

use crate::domain::character::traits::CharacterService;
use crate::domain::events::{AppEvent, EventBus};
use crate::domain::walkthrough::models::{
    CharacterWalkthroughProgress, WalkthroughGuide, WalkthroughProgress, WalkthroughStepResult,
};
use crate::domain::walkthrough::traits::{WalkthroughRepository, WalkthroughService};
use crate::errors::AppError;

/// Implementation of WalkthroughService for business logic
pub struct WalkthroughServiceImpl {
    /// Repository for walkthrough data persistence
    repository: Arc<dyn WalkthroughRepository + Send + Sync>,
    /// Character service for accessing character data
    character_service: Arc<dyn CharacterService + Send + Sync>,
    /// Event bus for publishing walkthrough events
    event_bus: Arc<EventBus>,
}

impl WalkthroughServiceImpl {
    /// Creates a new WalkthroughServiceImpl
    pub fn new(
        repository: Arc<dyn WalkthroughRepository + Send + Sync>,
        character_service: Arc<dyn CharacterService + Send + Sync>,
        event_bus: Arc<EventBus>,
    ) -> Self {
        Self {
            repository,
            character_service,
            event_bus,
        }
    }

    /// Gets a character's current walkthrough progress
    async fn get_character_walkthrough_progress(
        &self,
        character_id: &str,
    ) -> Result<WalkthroughProgress, AppError> {
        let character_data = self.character_service.get_character(character_id).await?;
        Ok(character_data.get_walkthrough_progress().clone())
    }

    /// Updates a character's walkthrough progress
    async fn update_character_walkthrough_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> Result<(), AppError> {
        let mut character_data = self.character_service.get_character(character_id).await?;
        let progress_clone = progress.clone();
        character_data.update_walkthrough_progress(progress);
        self.character_service
            .save_character_data(&character_data)
            .await?;

        // Publish walkthrough progress updated event
        let event =
            AppEvent::walkthrough_progress_updated(character_id.to_string(), progress_clone);
        let _ = self.event_bus.publish(event).await;

        debug!(
            "🔍 WALKTHROUGH SERVICE: Updated character {} walkthrough progress",
            character_id
        );
        Ok(())
    }
}

#[async_trait::async_trait]
impl WalkthroughService for WalkthroughServiceImpl {
    /// Gets the complete walkthrough guide
    async fn get_guide(&self) -> Result<WalkthroughGuide, AppError> {
        debug!("🔍 WALKTHROUGH SERVICE: Getting walkthrough guide");

        // We need to make this mutable to access the cache
        // This is a limitation of the current design - we'll need to use Arc<Mutex<>> for the cache
        // For now, we'll load from repository each time
        let guide = self.repository.load_guide().await?;
        info!(
            "✅ WALKTHROUGH SERVICE: Retrieved walkthrough guide with {} acts",
            guide.acts.len()
        );
        Ok(guide)
    }

    /// Gets a specific step by ID
    async fn get_step(&self, step_id: &str) -> Result<Option<WalkthroughStepResult>, AppError> {
        debug!("🔍 WALKTHROUGH SERVICE: Getting step {}", step_id);

        let guide = self.repository.load_guide().await?;

        for (_, act) in &guide.acts {
            if let Some(step) = act.steps.get(step_id) {
                let result = WalkthroughStepResult {
                    step: step.clone(),
                    act_name: act.act_name.clone(),
                    act_number: act.act_number,
                };
                info!(
                    "✅ WALKTHROUGH SERVICE: Found step {} in act {}",
                    step_id, act.act_name
                );
                return Ok(Some(result));
            }
        }

        warn!("⚠️ WALKTHROUGH SERVICE: Step {} not found", step_id);
        Ok(None)
    }

    /// Gets a character's walkthrough progress
    async fn get_character_progress(
        &self,
        character_id: &str,
    ) -> Result<CharacterWalkthroughProgress, AppError> {
        debug!(
            "🔍 WALKTHROUGH SERVICE: Getting character {} progress",
            character_id
        );

        let progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;

        let current_step = if let Some(step_id) = &progress.current_step_id {
            self.get_step(step_id).await?
        } else {
            None
        };

        let next_step = if let Some(_step_id) = &progress.current_step_id {
            if let Some(current) = &current_step {
                self.get_step(&current.step.next_step_id.clone().unwrap_or_default())
                    .await?
            } else {
                None
            }
        } else {
            None
        };

        let previous_step = if let Some(_step_id) = &progress.current_step_id {
            if let Some(current) = &current_step {
                if let Some(prev_id) = &current.step.previous_step_id {
                    self.get_step(prev_id).await?
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        let result = CharacterWalkthroughProgress {
            progress,
            current_step,
            next_step,
            previous_step,
        };

        info!(
            "✅ WALKTHROUGH SERVICE: Retrieved character {} progress",
            character_id
        );
        Ok(result)
    }

    /// Updates a character's walkthrough progress
    async fn update_character_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> Result<(), AppError> {
        debug!(
            "🔍 WALKTHROUGH SERVICE: Updating character {} progress",
            character_id
        );

        self.update_character_walkthrough_progress(character_id, progress)
            .await?;

        info!(
            "✅ WALKTHROUGH SERVICE: Updated character {} progress",
            character_id
        );
        Ok(())
    }

    /// Advances a character to the next step
    async fn advance_character_to_next_step(&self, character_id: &str) -> Result<(), AppError> {
        debug!(
            "🔍 WALKTHROUGH SERVICE: Advancing character {} to next step",
            character_id
        );

        let mut progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;

        if progress.is_completed {
            warn!(
                "⚠️ WALKTHROUGH SERVICE: Character {} has already completed the campaign",
                character_id
            );
            return Ok(());
        }

        let current_step = if let Some(step_id) = &progress.current_step_id {
            self.get_step(step_id).await?
        } else {
            return Err(AppError::Validation {
                message: "Character has no current step".to_string(),
            });
        };

        if let Some(current) = current_step {
            let from_step_id = progress.current_step_id.clone();
            progress.advance_to_next_step(current.step.next_step_id.clone());
            let to_step_id = progress.current_step_id.clone();
            self.update_character_walkthrough_progress(character_id, progress)
                .await?;

            // Publish step advanced event
            let event = AppEvent::walkthrough_step_advanced(
                character_id.to_string(),
                from_step_id,
                to_step_id,
            );
            let _ = self.event_bus.publish(event).await;

            // If step was completed, publish step completed event
            let event = AppEvent::walkthrough_step_completed(character_id.to_string(), current);
            let _ = self.event_bus.publish(event).await;

            info!(
                "✅ WALKTHROUGH SERVICE: Advanced character {} to next step",
                character_id
            );
        } else {
            return Err(AppError::Validation {
                message: "Current step not found".to_string(),
            });
        }

        Ok(())
    }

    /// Moves a character to a specific step
    async fn move_character_to_step(
        &self,
        character_id: &str,
        step_id: &str,
    ) -> Result<(), AppError> {
        debug!(
            "🔍 WALKTHROUGH SERVICE: Moving character {} to step {}",
            character_id, step_id
        );

        // Verify the step exists
        if self.get_step(step_id).await?.is_none() {
            return Err(AppError::Validation {
                message: format!("Step {} not found", step_id),
            });
        }

        let mut progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;
        progress.set_current_step(step_id.to_string());
        self.update_character_walkthrough_progress(character_id, progress)
            .await?;

        info!(
            "✅ WALKTHROUGH SERVICE: Moved character {} to step {}",
            character_id, step_id
        );
        Ok(())
    }

    /// Marks a character's campaign as completed
    async fn mark_character_campaign_completed(&self, character_id: &str) -> Result<(), AppError> {
        debug!(
            "🔍 WALKTHROUGH SERVICE: Marking character {} campaign as completed",
            character_id
        );

        let mut progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;
        progress.mark_completed();
        self.update_character_walkthrough_progress(character_id, progress)
            .await?;

        // Publish campaign completed event
        let event = AppEvent::walkthrough_campaign_completed(character_id.to_string());
        let _ = self.event_bus.publish(event).await;

        info!(
            "✅ WALKTHROUGH SERVICE: Marked character {} campaign as completed",
            character_id
        );
        Ok(())
    }

    /// Handles a scene change for walkthrough progress detection
    async fn handle_scene_change(
        &self,
        character_id: &str,
        scene_content: &str,
    ) -> Result<(), AppError> {
        debug!(
            "🔍 WALKTHROUGH SERVICE: Handling scene change for character {}",
            character_id
        );

        let progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;

        // If character has completed campaign, skip processing
        if progress.is_completed {
            debug!("🔍 WALKTHROUGH SERVICE: Character {} has completed campaign, skipping scene change processing", character_id);
            return Ok(());
        }

        // Process the scene change content to get the zone name
        if let Some(scene_event) = self
            .character_service
            .process_scene_content(scene_content, character_id)
            .await?
        {
            if scene_event.is_zone() {
                let zone_name = scene_event.get_name();
                debug!(
                    "🔍 WALKTHROUGH SERVICE: Detected zone change to {} for character {}",
                    zone_name, character_id
                );

                // Check if this zone matches the current step's completion_zone
                if let Some(step_id) = &progress.current_step_id {
                    if let Some(step_result) = self.get_step(step_id).await? {
                        if step_result.step.completion_zone == zone_name {
                            debug!("🔍 WALKTHROUGH SERVICE: Zone {} matches completion zone for step {}, advancing", zone_name, step_id);
                            self.advance_character_to_next_step(character_id).await?;
                        } else {
                            debug!("🔍 WALKTHROUGH SERVICE: Zone {} does not match completion zone {} for step {}", zone_name, step_result.step.completion_zone, step_id);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walkthrough_progress_new() {
        let progress = WalkthroughProgress::new();
        assert_eq!(progress.current_step_id, Some("act_4_step_1".to_string()));
        assert!(!progress.is_completed);
    }

    #[test]
    fn test_walkthrough_progress_advance() {
        let mut progress = WalkthroughProgress::new();
        progress.advance_to_next_step(Some("act_4_step_2".to_string()));
        assert_eq!(progress.current_step_id, Some("act_4_step_2".to_string()));
        assert!(!progress.is_completed);

        progress.advance_to_next_step(None);
        assert_eq!(progress.current_step_id, None);
        assert!(progress.is_completed);
    }
}
