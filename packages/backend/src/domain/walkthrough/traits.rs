use crate::domain::walkthrough::models::{
    CharacterWalkthroughProgress, WalkthroughGuide, WalkthroughProgress,
};
use crate::errors::AppResult;

#[async_trait::async_trait]
pub trait WalkthroughRepository: Send + Sync {
    async fn load_guide(&self) -> AppResult<WalkthroughGuide>;
}

#[async_trait::async_trait]
pub trait WalkthroughService: Send + Sync {
    async fn get_guide(&self) -> AppResult<WalkthroughGuide>;

    async fn get_character_progress(
        &self,
        character_id: &str,
    ) -> AppResult<CharacterWalkthroughProgress>;

    async fn update_character_progress(
        &self,
        character_id: &str,
        progress: WalkthroughProgress,
    ) -> AppResult<()>;

    async fn handle_scene_change(
        &self,
        character_id: &str,
        scene_content: &str,
    ) -> AppResult<()>;
}
