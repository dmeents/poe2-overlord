use crate::domain::walkthrough::models::{
    CharacterWalkthroughProgress, WalkthroughGuide, WalkthroughProgress,
};
use crate::errors::AppError;

/// Repository trait for walkthrough data persistence operations.
///
/// This trait defines the contract for persisting and retrieving walkthrough
/// guide data from storage. Implementations handle loading the complete
/// walkthrough structure from configuration files.
#[async_trait::async_trait]
pub trait WalkthroughRepository: Send + Sync {
    /// Loads the complete walkthrough guide from storage
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError>;
}

/// Service trait for walkthrough business logic operations.
///
/// This trait defines the contract for walkthrough management business logic
/// including guide loading, progress tracking, and scene change processing.
/// It coordinates between the repository layer and character service to manage
/// walkthrough progression through the game's campaign.
#[async_trait::async_trait]
pub trait WalkthroughService: Send + Sync {
    /// Gets the complete walkthrough guide
    async fn get_guide(&self) -> Result<WalkthroughGuide, AppError>;

    /// Gets a character's walkthrough progress with navigation context
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

    /// Handles a scene change for walkthrough progress detection and advancement
    async fn handle_scene_change(
        &self,
        character_id: &str,
        scene_content: &str,
    ) -> Result<(), AppError>;
}
