use crate::domain::walkthrough::models::{
    CharacterWalkthroughProgress, WalkthroughGuide, WalkthroughProgress,
};
use crate::errors::AppError;

#[async_trait::async_trait]
pub trait WalkthroughRepository: Send + Sync {
    async fn load_guide(&self) -> Result<WalkthroughGuide, AppError>;
}

#[async_trait::async_trait]
pub trait WalkthroughService: Send + Sync {
    async fn get_guide(&self) -> Result<WalkthroughGuide, AppError>;

    async fn get_character_progress(
        &self,
        character_id: &str,
    ) -> Result<CharacterWalkthroughProgress, AppError>;

    async fn update_character_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> Result<(), AppError>;

    async fn handle_scene_change(
        &self,
        character_id: &str,
        scene_content: &str,
    ) -> Result<(), AppError>;
}
