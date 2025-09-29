use crate::domain::walkthrough::models::{
    CharacterWalkthroughProgress, WalkthroughGuide, WalkthroughProgress, WalkthroughStepResult,
};
use crate::errors::AppError;

/// Repository trait for walkthrough data persistence
#[async_trait::async_trait]
pub trait WalkthroughRepository: Send + Sync {
    /// Loads the complete walkthrough guide from storage
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError>;

    /// Saves the walkthrough guide to storage
    async fn save_guide(&self, guide: &WalkthroughGuide) -> Result<(), AppError>;
}

/// Service trait for walkthrough business logic
#[async_trait::async_trait]
pub trait WalkthroughService: Send + Sync {
    /// Gets the complete walkthrough guide
    async fn get_guide(&self) -> Result<WalkthroughGuide, AppError>;

    /// Gets a specific step by ID
    async fn get_step(&self, step_id: &str) -> Result<Option<WalkthroughStepResult>, AppError>;

    /// Gets a character's walkthrough progress
    async fn get_character_progress(
        &self,
        character_id: &str,
    ) -> Result<CharacterWalkthroughProgress, AppError>;

    /// Updates a character's walkthrough progress
    async fn update_character_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> Result<(), AppError>;

    /// Advances a character to the next step
    async fn advance_character_to_next_step(&self, character_id: &str) -> Result<(), AppError>;

    /// Moves a character to a specific step
    async fn move_character_to_step(
        &self,
        character_id: &str,
        step_id: &str,
    ) -> Result<(), AppError>;

    /// Marks a character's campaign as completed
    async fn mark_character_campaign_completed(&self, character_id: &str) -> Result<(), AppError>;

    /// Handles a scene change for walkthrough progress detection
    async fn handle_scene_change(
        &self,
        character_id: &str,
        scene_content: &str,
    ) -> Result<(), AppError>;
}
