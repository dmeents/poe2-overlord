use log::{debug, error, info, warn};
use std::sync::Arc;

use crate::domain::character::traits::CharacterService;
use crate::domain::walkthrough::models::{
    CharacterWalkthroughProgress, WalkthroughGuide, WalkthroughProgress, WalkthroughStepResult,
};
use crate::domain::walkthrough::traits::{WalkthroughRepository, WalkthroughService};
use crate::errors::AppError;
use crate::infrastructure::events::{AppEvent, EventBus};

/// Implementation of the WalkthroughService trait.
///
/// This service provides business logic for walkthrough management including
/// guide loading, progress tracking, and scene change processing. It coordinates
/// between the repository layer and character service to manage walkthrough
/// progression through the game's campaign.
pub struct WalkthroughServiceImpl {
    /// Repository for walkthrough data persistence
    repository: Arc<dyn WalkthroughRepository + Send + Sync>,
    /// Character service for accessing character data
    character_service: Arc<dyn CharacterService + Send + Sync>,
    /// Event bus for publishing walkthrough events
    event_bus: Arc<EventBus>,
}

impl WalkthroughServiceImpl {
    /// Creates a new WalkthroughServiceImpl instance with required dependencies
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

    /// Gets a character's current walkthrough progress from character data
    async fn get_character_walkthrough_progress(
        &self,
        character_id: &str,
    ) -> Result<WalkthroughProgress, AppError> {
        let character_data = self
            .character_service
            .load_character_data(character_id)
            .await?;
        Ok(character_data.get_walkthrough_progress().clone())
    }

    /// Updates a character's walkthrough progress and publishes events
    async fn update_character_walkthrough_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> Result<(), AppError> {
        let mut character_data = self
            .character_service
            .load_character_data(character_id)
            .await?;
        character_data.update_walkthrough_progress(progress);
        character_data.touch();

        // Save character data
        self.character_service
            .save_character_data(&character_data)
            .await?;

        // Get enriched character data for event
        let enriched_data = self.character_service.get_character(character_id).await?;

        // Emit character updated event (includes walkthrough progress)
        let event = AppEvent::character_updated(character_id.to_string(), enriched_data);
        if let Err(e) = self.event_bus.publish(event).await {
            error!(
                "Failed to publish character updated event for {}: {}",
                character_id, e
            );
        }

        debug!("Updated character {} walkthrough progress", character_id);
        Ok(())
    }
}

#[async_trait::async_trait]
impl WalkthroughService for WalkthroughServiceImpl {
    /// Gets the complete walkthrough guide
    async fn get_guide(&self) -> Result<WalkthroughGuide, AppError> {
        // Load guide from repository (no caching implemented yet)
        let guide = self.repository.load_guide().await?;
        info!("Retrieved walkthrough guide with {} acts", guide.acts.len());
        Ok(guide)
    }

    /// Gets a character's walkthrough progress with navigation context
    async fn get_character_progress(
        &self,
        character_id: &str,
    ) -> Result<CharacterWalkthroughProgress, AppError> {
        let progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;

        // Get step IDs from the progress and guide for navigation context
        let (next_step_id, previous_step_id) = if let Some(step_id) = &progress.current_step_id {
            // Load guide and use array-based navigation
            let guide = self.repository.load_guide().await?;
            (guide.next_step_id(step_id), guide.previous_step_id(step_id))
        } else {
            (None, None)
        };

        let result = CharacterWalkthroughProgress {
            progress,
            next_step_id,
            previous_step_id,
        };

        Ok(result)
    }

    /// Updates a character's walkthrough progress
    ///
    /// Validates that the step ID exists in the guide if provided.
    /// This prevents data corruption from invalid step references.
    async fn update_character_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> Result<(), AppError> {
        // Validate step ID exists in guide if provided
        if let Some(step_id) = &progress.current_step_id {
            let guide = self.repository.load_guide().await?;

            if !guide.step_exists(step_id) {
                return Err(AppError::validation_error(
                    "update_character_progress",
                    &format!(
                        "Invalid step ID '{}' - does not exist in walkthrough guide",
                        step_id
                    ),
                ));
            }
        }

        self.update_character_walkthrough_progress(character_id, progress)
            .await?;

        info!("Updated character {} walkthrough progress", character_id);
        Ok(())
    }

    /// Handles a scene change for walkthrough progress detection and advancement
    async fn handle_scene_change(
        &self,
        character_id: &str,
        scene_content: &str,
    ) -> Result<(), AppError> {
        let progress = self
            .get_character_walkthrough_progress(character_id)
            .await?;

        // If character has completed campaign, skip processing
        if progress.is_completed {
            debug!(
                "Character {} has completed campaign, skipping scene change processing",
                character_id
            );
            return Ok(());
        }

        // scene_content is already the zone name
        let zone_name = scene_content.trim();

        if !zone_name.is_empty() {
            debug!(
                "Detected zone change to {} for character {}",
                zone_name, character_id
            );

            // Check if this zone matches the current step's completion_zone
            if let Some(step_id) = &progress.current_step_id {
                // Load guide and find the current step using array-based navigation
                let guide = self.repository.load_guide().await?;

                if let Some((act_idx, step_idx)) = guide.find_step(step_id) {
                    let act = &guide.acts[act_idx];
                    let step = &act.steps[step_idx];

                    if step.completion_zone == zone_name {
                        debug!(
                            "Zone {} matches completion zone for step {}, advancing",
                            zone_name, step_id
                        );

                        // Create step result for events
                        let step_result = WalkthroughStepResult {
                            step: step.clone(),
                            act_name: act.act_name.clone(),
                            act_number: (act_idx + 1) as u32, // 1-based index
                        };

                        // Get the next step ID and move to it
                        if let Some(next_step_id) = guide.next_step_id(step_id) {
                            let from_step_id = progress.current_step_id.clone();

                            // Create new progress with next step
                            let mut new_progress = progress.clone();
                            new_progress.set_current_step(next_step_id.clone());
                            self.update_character_walkthrough_progress(character_id, new_progress)
                                .await?;

                            // Publish step advanced event
                            let event = AppEvent::walkthrough_step_advanced(
                                character_id.to_string(),
                                from_step_id,
                                Some(next_step_id.clone()),
                            );
                            if let Err(e) = self.event_bus.publish(event).await {
                                warn!("Failed to publish walkthrough step advanced event: {}", e);
                            }

                            // Publish step completed event
                            let event = AppEvent::walkthrough_step_completed(
                                character_id.to_string(),
                                step_result,
                            );
                            if let Err(e) = self.event_bus.publish(event).await {
                                warn!("Failed to publish walkthrough step completed event: {}", e);
                            }
                        } else {
                            // No next step, mark campaign as completed
                            let mut new_progress = progress.clone();
                            new_progress.mark_completed();
                            self.update_character_walkthrough_progress(character_id, new_progress)
                                .await?;

                            // Publish campaign completed event
                            let event =
                                AppEvent::walkthrough_campaign_completed(character_id.to_string());
                            if let Err(e) = self.event_bus.publish(event).await {
                                warn!(
                                    "Failed to publish walkthrough campaign completed event: {}",
                                    e
                                );
                            }
                        }
                    } else {
                        debug!(
                            "Zone {} does not match completion zone {} for step {}",
                            zone_name, step.completion_zone, step_id
                        );
                    }
                }
            }
        }

        Ok(())
    }
}
